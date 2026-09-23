//! Stateful workloads: real/synthetic history, dirty trees and divergent branches.
use super::{
    config::Query,
    corpus::{append, candidates, commit},
    process::{self, git},
    search,
    session::{Fixture, Session},
};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs};

pub fn probes(s: &Session<'_>) -> Vec<Query> {
    let mut queries: Vec<_> = s.corpus.config.queries.iter().take(3).cloned().collect();
    queries.push(Query {
        name: "workflow_markers".into(),
        pattern: "CODERG_(WORKFLOW|BRANCH|HISTORY)".into(),
        flags: vec![],
    });
    for marker in [
        "CODERG_BRANCH_A1",
        "CODERG_BRANCH_A2",
        "CODERG_BRANCH_B1",
        "CODERG_BRANCH_B2",
    ] {
        queries.push(Query {
            name: marker.into(),
            pattern: marker.into(),
            flags: vec!["-F".into(), "-l".into()],
        });
    }
    queries
}

pub fn history(s: &mut Session<'_>) -> Result<()> {
    let commits = s.corpus.commits.clone();
    let queries = probes(s);
    for round in 0..s.config.history_rounds + s.config.rss_runs {
        let resource = round >= s.config.history_rounds;
        let f = s.fixture(&commits[0])?;
        s.initialize(&f)?;
        if !resource {
            search::checkpoint(s, &f, "history_0", round)?;
        }
        let checkpoints: Vec<_> = (1..=4)
            .map(|n| (s.corpus.config.updates * n).div_ceil(4))
            .collect();
        for (step, id) in commits.iter().enumerate().skip(1) {
            git(&f.root, &["checkout", "--quiet", "--detach", id])?;
            let case = format!("step_{step:03}");
            transition(s, &f, "history", &case, round, resource, &queries, None)?;
            if !resource && checkpoints.contains(&step) {
                eprintln!(
                    "{} history round {}: {step}/{} updates",
                    s.corpus.config.name,
                    round + 1,
                    s.corpus.config.updates
                );
                search::checkpoint(s, &f, &format!("history_{step}"), round)?;
            }
        }
        for step in [0, s.corpus.config.updates / 2, s.corpus.config.updates] {
            git(
                &f.root,
                &["checkout", "--quiet", "--detach", &commits[step]],
            )?;
            transition(
                s,
                &f,
                "history",
                &format!("revisit_{step}"),
                round,
                resource,
                &queries,
                None,
            )?;
        }
        s.report.event(json!({"event": "history_complete", "corpus": s.corpus.config.name, "history_kind": s.corpus.history_kind, "round": round, "rss": resource, "updates": s.corpus.config.updates}))?;
    }
    Ok(())
}

/// Compare the first default search after a state change with rg on that same tree.
#[allow(clippy::too_many_arguments)]
pub fn transition(
    s: &mut Session<'_>,
    f: &Fixture,
    scenario: &str,
    case: &str,
    round: usize,
    rss: bool,
    queries: &[Query],
    previous_refs: Option<&[Value]>,
) -> Result<Vec<Value>> {
    let mut refs = vec![Value::Null; s.variants.len()];
    for v in 0..s.variants.len() {
        let name = s.variants[v].name.clone();
        let mut engines = [true, false];
        process::shuffle(&mut engines, &mut s.seed);
        for current in engines {
            let (engine, mut command) = if current {
                (name.as_str(), s.search(f, v, &queries[0]))
            } else {
                ("rg", s.rg(f, &queries[0]))
            };
            if rss {
                s.resource(scenario, case, engine, round, &command, true)?;
            } else {
                s.measure(scenario, case, engine, round, &mut command, true)?;
            }
        }
        refs[v] = s.stats(f, v)?["segments"].clone();
        if name == "current"
            && let Some(previous) = previous_refs
        {
            ensure!(
                refs[v] == previous[v],
                "{scenario}/{case}: commit promotion rewrote already indexed segment references"
            );
        }
    }
    s.verify(f, case, queries)?;
    if !rss {
        s.state(f, scenario, case, round)?;
    }
    Ok(refs)
}

pub fn workflow(s: &mut Session<'_>, branches: bool) -> Result<()> {
    let revision = s.corpus.commits.last().unwrap().clone();
    let extension = s.corpus.config.extension.clone();
    let queries = probes(s);
    let scenario = if branches { "branches" } else { "workflow" };
    for fraction in [1, 50] {
        for round in 0..s.config.rounds + s.config.rss_runs {
            let rss = round >= s.config.rounds;
            let f = s.fixture(&revision)?;
            s.initialize(&f)?;
            let mut chosen = candidates(&f.root, &extension)?;
            let mut seed = s.config.seed;
            process::shuffle(&mut chosen, &mut seed);
            chosen.truncate((chosen.len() * fraction / 100).max(1));
            s.report.event(json!({"event": "workflow_selection", "corpus": s.corpus.config.name, "scenario": scenario, "round": round, "fraction_percent": fraction, "paths": chosen}))?;
            let label = |step: &str| format!("edit_{fraction}pct-{step}");
            if branches {
                let mut tips = BTreeMap::from([("A", revision.clone()), ("B", revision.clone())]);
                for n in 1..=2 {
                    for branch in ["A", "B"] {
                        git(&f.root, &["checkout", "--quiet", "--detach", &tips[branch]])?;
                        transition(
                            s,
                            &f,
                            scenario,
                            &label(&format!("switch_{branch}{n}")),
                            round,
                            rss,
                            &queries,
                            None,
                        )?;
                        append(
                            &f.root,
                            &chosen,
                            &format!("CODERG_BRANCH_{branch}{n}"),
                            &extension,
                        )?;
                        let refs = transition(
                            s,
                            &f,
                            scenario,
                            &label(&format!("edit_{branch}{n}")),
                            round,
                            rss,
                            &queries,
                            None,
                        )?;
                        let id = commit(
                            &f.root,
                            &format!("branch {branch}{n}"),
                            n * 10 + usize::from(branch == "B"),
                        )?;
                        tips.insert(branch, id);
                        transition(
                            s,
                            &f,
                            scenario,
                            &label(&format!("commit_{branch}{n}")),
                            round,
                            rss,
                            &queries,
                            Some(&refs),
                        )?;
                    }
                }
                for branch in ["A", "B"] {
                    git(&f.root, &["checkout", "--quiet", "--detach", &tips[branch]])?;
                    transition(
                        s,
                        &f,
                        scenario,
                        &label(&format!("revisit_{branch}")),
                        round,
                        rss,
                        &queries,
                        None,
                    )?;
                }
            } else {
                append(&f.root, &chosen, "CODERG_WORKFLOW_COMMITTED", &extension)?;
                let refs = transition(s, &f, scenario, &label("edit"), round, rss, &queries, None)?;
                let tip = commit(&f.root, "workflow edit", 1)?;
                transition(
                    s,
                    &f,
                    scenario,
                    &label("promotion"),
                    round,
                    rss,
                    &queries,
                    Some(&refs),
                )?;
                for (name, id) in [("rollback", &revision), ("revisit", &tip)] {
                    git(&f.root, &["checkout", "--quiet", "--detach", id])?;
                    transition(s, &f, scenario, &label(name), round, rss, &queries, None)?;
                }
                append(&f.root, &chosen, "CODERG_WORKFLOW_DIRTY", &extension)?;
                transition(s, &f, scenario, &label("dirty"), round, rss, &queries, None)?;
                git(&f.root, &["restore", "--worktree", "."])?;
                transition(
                    s,
                    &f,
                    scenario,
                    &label("discard"),
                    round,
                    rss,
                    &queries,
                    None,
                )?;
                let added = f.root.join(format!("coderg_workflow_added{extension}"));
                let renamed = f.root.join(format!("coderg_workflow_renamed{extension}"));
                ensure!(
                    !added.exists() && !renamed.exists(),
                    "workflow fixture collision"
                );
                fs::write(&added, "// CODERG_WORKFLOW_ADDED\n")?;
                transition(
                    s,
                    &f,
                    scenario,
                    &label("untracked"),
                    round,
                    rss,
                    &queries,
                    None,
                )?;
                commit(&f.root, "workflow add", 2)?;
                transition(
                    s,
                    &f,
                    scenario,
                    &label("add_commit"),
                    round,
                    rss,
                    &queries,
                    None,
                )?;
                fs::rename(&added, &renamed)?;
                transition(
                    s,
                    &f,
                    scenario,
                    &label("rename"),
                    round,
                    rss,
                    &queries,
                    None,
                )?;
                fs::remove_file(&renamed)?;
                transition(
                    s,
                    &f,
                    scenario,
                    &label("delete"),
                    round,
                    rss,
                    &queries,
                    None,
                )?;
                // A new ignore rule must remove an already indexed untracked file.
                fs::write(&added, "// CODERG_WORKFLOW_IGNORED\n")?;
                transition(
                    s,
                    &f,
                    scenario,
                    &label("before_ignore"),
                    round,
                    rss,
                    &queries,
                    None,
                )?;
                let ignored = f.root.join("coderg_workflow_ignored");
                ensure!(!ignored.exists(), "workflow ignore fixture collision");
                fs::create_dir(&ignored)?;
                fs::write(ignored.join(".ignore"), format!("*{extension}\n"))?;
                fs::rename(&added, ignored.join(format!("ignored{extension}")))?;
                transition(
                    s,
                    &f,
                    scenario,
                    &label("ignored"),
                    round,
                    rss,
                    &queries,
                    None,
                )?;
            }
        }
    }
    Ok(())
}
