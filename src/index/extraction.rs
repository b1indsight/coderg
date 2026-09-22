//! Bounded parallel extraction shared by worktree and Git snapshot builds.
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc,
};

use anyhow::Result;

use crate::{build::PostingsBuilder, ngram::GramHash};

/// Feed per-source gram batches to one postings collector. Each producer owns
/// its reader state (for example a Git repository); dropping the receiver on
/// failure unblocks every producer before the scoped threads are joined.
pub(super) fn collect<T: Sync, W>(
    sources: &[T],
    workers: usize,
    postings: &mut PostingsBuilder,
    document_id: impl Fn(&T) -> u32,
    initialize: impl Fn() -> W + Sync,
    extract: impl Fn(&mut W, &T, &mut dyn FnMut(Vec<GramHash>) -> Result<()>) -> Result<()> + Sync,
) -> Result<Vec<bool>> {
    let mut searchable = vec![false; sources.len()];
    let next = AtomicUsize::new(0);
    std::thread::scope(|scope| -> Result<()> {
        let (sender, receiver) = mpsc::sync_channel(workers);
        for _ in 0..workers {
            let sender = sender.clone();
            let (next, initialize, extract) = (&next, &initialize, &extract);
            // Dedicated producers leave Rayon's workers available to sort the
            // collector's postings even while the bounded channel is full.
            scope.spawn(move || {
                let mut reader = initialize();
                loop {
                    let position = next.fetch_add(1, Ordering::Relaxed);
                    let Some(source) = sources.get(position) else {
                        break;
                    };
                    let result = extract(&mut reader, source, &mut |grams| {
                        sender
                            .send(Ok((position, grams)))
                            .map_err(|_| anyhow::anyhow!("index build cancelled"))
                    });
                    if let Err(error) = result {
                        // A disconnected collector has already returned its error.
                        let _ = sender.send(Err(error));
                        break;
                    }
                }
            });
        }
        drop(sender);
        for batch in receiver {
            let (position, grams) = batch?;
            searchable[position] = true;
            postings.extend(document_id(&sources[position]), &grams)?;
        }
        Ok(())
    })?;
    Ok(searchable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collector_failure_cancels_producers_and_preserves_the_original_error() {
        let temp = tempfile::tempdir().unwrap();
        let missing = temp.path().join("missing");
        let mut postings = PostingsBuilder::new(&missing, 8);
        // A failed spill (for example, an unavailable index directory) must
        // release producers sending further batches into the bounded channel.
        let error = collect(
            &[0, 1],
            2,
            &mut postings,
            |id| *id,
            || (),
            |_, _, emit| {
                for _ in 0..100 {
                    emit(vec![1, 2])?;
                }
                Ok(())
            },
        )
        .unwrap_err();
        assert_eq!(
            error.downcast_ref::<std::io::Error>().unwrap().kind(),
            std::io::ErrorKind::NotFound,
        );
        assert!(!missing.exists());
    }
}
