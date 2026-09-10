import csv
import json
import os
from pathlib import Path
import shutil
import subprocess

work=Path(__file__).resolve().parent
repo=work.parents[2]
old=repo/'.cache/2026-09-10/chromium-letter-distributions'
data=json.loads((work/'counts.json').read_text())
previous=json.loads((old/'counts.json').read_text())
for key in ['files','binary_files','source_bytes','all','initial']:
    assert data['counts'][key]==previous['counts'][key],key
counts=data['counts']['pairs'];total=sum(counts)
assert total==data['all_total']-data['initial_total']
scale=100000**2
frequencies=[(n*scale+total//2)//total for n in counts]
uni=json.loads((old/'tables.json').read_text())['chromium_all']
maximum=max(max(frequencies),uni['common_frequency']**2)
assert maximum<2**32
tables=dict(chromium_all=uni,chromium_pairs=dict(counts=counts,total=total,scale=scale,
    frequencies=frequencies,max_frequency=maximum,index_version=8,
    rules='Directed ASCII pairs, case folded; pair frequency conditional on both bytes being letters; mixed/nonletter pairs keep Chromium unigram product fallback'))
(work/'tables.json').write_text(json.dumps(tables,indent=2)+'\n')
with (work/'pair-frequencies.csv').open('w') as f:
    writer=csv.writer(f);writer.writerow(['pair','count','probability_pct','independent_probability_pct','actual_over_independent','weight_frequency'])
    for i,n in enumerate(counts):
        left,right=divmod(i,26)
        independent=uni['frequencies'][left]*uni['frequencies'][right]/scale
        writer.writerow([chr(97+left)+chr(97+right),n,100*n/total,100*independent,(n/total)/independent,frequencies[i]])
src=work/'chromium_pairs-src'
shutil.copytree(old/'chromium_all-src',src)
p=src/'src/ngram.rs';s=p.read_text()
anchor='const COMMON_FREQUENCY: u32 = 11608;'
assert anchor in s
table='\n'.join('    '+', '.join(map(str,frequencies[i:i+13]))+',' for i in range(0,len(frequencies),13))
s=s.replace(anchor,anchor+'\n\n// Empirical Chromium pair probabilities scaled by 100000^2, row-major a..z.\n'
    +'const PAIR_FREQUENCIES: [u32; 26 * 26] = [\n'+table+'\n];\n'
    +f'const MAX_PAIR_FREQUENCY: u32 = {maximum};')
start=s.index('fn pair_weight_value(');end=s.index('\nfn byte_frequency',start)
s=s[:start]+'''fn pair_weight_value(value: u16) -> u64 {
    let [left, right] = value.to_le_bytes();
    let rarity = MAX_PAIR_FREQUENCY - pair_frequency(left, right);
    let tie_break = mix(u64::from(value) ^ 0x9e37_79b9_7f4a_7c15) & u64::from(u32::MAX);
    (u64::from(rarity) << 32) | tie_break
}

fn pair_frequency(left: u8, right: u8) -> u32 {
    let a = left.to_ascii_lowercase();
    let b = right.to_ascii_lowercase();
    if a.is_ascii_lowercase() && b.is_ascii_lowercase() {
        PAIR_FREQUENCIES[usize::from(a - b'a') * 26 + usize::from(b - b'a')]
    } else {
        // Preserve the previous unigram estimate for nonletter/mixed pairs.
        byte_frequency(left) * byte_frequency(right)
    }
}
'''+s[end:]
start=s.index('    #[test]\n    fn rare_letters_raise_pair_weights_in_either_position()')
end=s.index('    #[test]\n    fn frequency_prior_folds_ascii_case',start)
s=s[:start]+'''    #[test]
    fn empirical_pair_order_and_case_folding_are_preserved() {
        assert!(pair_frequency(b'q', b'u') > pair_frequency(b'u', b'q'));
        assert!(pair_weight(b"qu") < pair_weight(b"uq"));
        for left in b'a'..=b'z' {
            for right in b'a'..=b'z' {
                let expected = pair_weight(&[left, right]) >> 32;
                for a in [left, left.to_ascii_uppercase()] {
                    for b in [right, right.to_ascii_uppercase()] {
                        assert_eq!(pair_weight(&[a, b]) >> 32, expected);
                    }
                }
            }
        }
    }

    #[test]
    fn nonletter_fallback_keeps_the_unigram_scale_and_order() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                if !left.is_ascii_alphabetic() || !right.is_ascii_alphabetic() {
                    assert_eq!(pair_frequency(left, right), byte_frequency(left) * byte_frequency(right));
                }
            }
        }
    }

'''+s[end:]
s=s.replace('fn frequency_prior_folds_ascii_case_and_does_not_boost_nonletters()', 'fn unigram_fallback_folds_ascii_case_and_assigns_common_nonletters()')
s=s.replace('assert_eq!(pair_weight(&[byte, b\'_\']) >> 32, 0);',
    'assert_eq!(pair_weight(&[byte, b\'_\']) >> 32, u64::from(MAX_PAIR_FREQUENCY - COMMON_FREQUENCY * COMMON_FREQUENCY));')
p.write_text(s)
p=src/'src/index.rs';p.write_text(p.read_text().replace('const VERSION: u32 = 6;','const VERSION: u32 = 8;'))
(work/'chromium_pairs-ngram.rs').write_bytes((src/'src/ngram.rs').read_bytes())
env=dict(os.environ,CARGO_TARGET_DIR=str(repo/'target'))
commands=[]
for cmd in [
    ['cargo','test','--locked','--offline','--manifest-path',str(src/'Cargo.toml'),'--bin','coderg'],
    ['cargo','rustc','--release','--locked','--offline','--manifest-path',str(src/'Cargo.toml'),'--bin','coderg','--','--cfg','benchmark_chromium_empirical_pairs'],
]:
    commands.append(cmd);subprocess.run(cmd,cwd=repo,env=env,check=True)
shutil.copy2(repo/'target/release/coderg',work/'chromium_pairs')
(work/'build-commands.json').write_text(json.dumps(commands,indent=2)+'\n')
print('Pairs',total,'nonzero',sum(n>0 for n in counts),'max frequency',maximum,flush=True)
