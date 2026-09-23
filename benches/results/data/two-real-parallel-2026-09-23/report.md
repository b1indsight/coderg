# Benchmark: two-real-parallel

Status: **complete**. Warm filesystem cache; fresh CLI processes. RSS sampled separately.

Maximum concurrent corpora: **2**; values above 1 include cross-corpus contention. Records are grouped by corpus, not global execution order.

History steps are heterogeneous workloads; p95 is descriptive, not a confidence interval.

| Corpus | Scenario | Case | Variant | Metric | N | Median | Mean | P95 | Max | Unit |
|---|---|---|---|---|---:|---:|---:|---:|---:|---|
| agentflow | branches | edit_1pct-commit_A1 | current | index_size | 3 | 3304781.000 | 3304781.000 | 3304781.000 | 3304781.000 | bytes |
| agentflow | branches | edit_1pct-commit_A1 | current | peak_rss | 2 | 10354688.000 | 10354688.000 | 10371072.000 | 10371072.000 | bytes |
| agentflow | branches | edit_1pct-commit_A1 | current | searchable_bytes | 3 | 2130720.000 | 2130720.000 | 2130720.000 | 2130720.000 | bytes |
| agentflow | branches | edit_1pct-commit_A1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-commit_A1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-commit_A1 | current | wall | 3 | 5.814 | 5.837 | 5.916 | 5.916 | ms |
| agentflow | branches | edit_1pct-commit_A1 | rg | peak_rss | 2 | 6496256.000 | 6496256.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | branches | edit_1pct-commit_A1 | rg | wall | 3 | 5.842 | 6.359 | 7.394 | 7.394 | ms |
| agentflow | branches | edit_1pct-commit_A2 | current | index_size | 3 | 3435040.000 | 3435040.000 | 3435040.000 | 3435040.000 | bytes |
| agentflow | branches | edit_1pct-commit_A2 | current | peak_rss | 2 | 10444800.000 | 10444800.000 | 10485760.000 | 10485760.000 | bytes |
| agentflow | branches | edit_1pct-commit_A2 | current | searchable_bytes | 3 | 2132257.000 | 2132257.000 | 2132257.000 | 2132257.000 | bytes |
| agentflow | branches | edit_1pct-commit_A2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-commit_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-commit_A2 | current | wall | 3 | 6.494 | 6.486 | 6.554 | 6.554 | ms |
| agentflow | branches | edit_1pct-commit_A2 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | branches | edit_1pct-commit_A2 | rg | wall | 3 | 5.814 | 6.333 | 7.386 | 7.386 | ms |
| agentflow | branches | edit_1pct-commit_B1 | current | index_size | 3 | 3369295.000 | 3369295.000 | 3369295.000 | 3369295.000 | bytes |
| agentflow | branches | edit_1pct-commit_B1 | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10387456.000 | 10387456.000 | bytes |
| agentflow | branches | edit_1pct-commit_B1 | current | searchable_bytes | 3 | 2130720.000 | 2130720.000 | 2130720.000 | 2130720.000 | bytes |
| agentflow | branches | edit_1pct-commit_B1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-commit_B1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-commit_B1 | current | wall | 3 | 6.655 | 6.416 | 6.845 | 6.845 | ms |
| agentflow | branches | edit_1pct-commit_B1 | rg | peak_rss | 2 | 6463488.000 | 6463488.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | branches | edit_1pct-commit_B1 | rg | wall | 3 | 6.398 | 6.814 | 7.969 | 7.969 | ms |
| agentflow | branches | edit_1pct-commit_B2 | current | index_size | 3 | 3500666.000 | 3500666.000 | 3500666.000 | 3500666.000 | bytes |
| agentflow | branches | edit_1pct-commit_B2 | current | peak_rss | 2 | 10428416.000 | 10428416.000 | 10469376.000 | 10469376.000 | bytes |
| agentflow | branches | edit_1pct-commit_B2 | current | searchable_bytes | 3 | 2132257.000 | 2132257.000 | 2132257.000 | 2132257.000 | bytes |
| agentflow | branches | edit_1pct-commit_B2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-commit_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-commit_B2 | current | wall | 3 | 6.442 | 6.426 | 6.773 | 6.773 | ms |
| agentflow | branches | edit_1pct-commit_B2 | rg | peak_rss | 2 | 6447104.000 | 6447104.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | branches | edit_1pct-commit_B2 | rg | wall | 3 | 7.568 | 7.999 | 9.647 | 9.647 | ms |
| agentflow | branches | edit_1pct-edit_A1 | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-edit_A1 | current | index_size | 3 | 3288274.000 | 3288274.000 | 3288274.000 | 3288274.000 | bytes |
| agentflow | branches | edit_1pct-edit_A1 | current | peak_rss | 2 | 11329536.000 | 11329536.000 | 11370496.000 | 11370496.000 | bytes |
| agentflow | branches | edit_1pct-edit_A1 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-edit_A1 | current | searchable_bytes | 3 | 2130720.000 | 2130720.000 | 2130720.000 | 2130720.000 | bytes |
| agentflow | branches | edit_1pct-edit_A1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-edit_A1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-edit_A1 | current | wall | 3 | 7.305 | 7.195 | 7.347 | 7.347 | ms |
| agentflow | branches | edit_1pct-edit_A1 | rg | peak_rss | 2 | 6447104.000 | 6447104.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | branches | edit_1pct-edit_A1 | rg | wall | 3 | 7.806 | 7.740 | 8.130 | 8.130 | ms |
| agentflow | branches | edit_1pct-edit_A2 | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-edit_A2 | current | index_size | 3 | 3418533.000 | 3418533.000 | 3418533.000 | 3418533.000 | bytes |
| agentflow | branches | edit_1pct-edit_A2 | current | peak_rss | 2 | 11345920.000 | 11345920.000 | 11354112.000 | 11354112.000 | bytes |
| agentflow | branches | edit_1pct-edit_A2 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-edit_A2 | current | searchable_bytes | 3 | 2132257.000 | 2132257.000 | 2132257.000 | 2132257.000 | bytes |
| agentflow | branches | edit_1pct-edit_A2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-edit_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-edit_A2 | current | wall | 3 | 6.676 | 6.903 | 7.418 | 7.418 | ms |
| agentflow | branches | edit_1pct-edit_A2 | rg | peak_rss | 2 | 6447104.000 | 6447104.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | branches | edit_1pct-edit_A2 | rg | wall | 3 | 7.150 | 6.892 | 7.418 | 7.418 | ms |
| agentflow | branches | edit_1pct-edit_B1 | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-edit_B1 | current | index_size | 3 | 3352788.000 | 3352788.000 | 3352788.000 | 3352788.000 | bytes |
| agentflow | branches | edit_1pct-edit_B1 | current | peak_rss | 2 | 11280384.000 | 11280384.000 | 11321344.000 | 11321344.000 | bytes |
| agentflow | branches | edit_1pct-edit_B1 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-edit_B1 | current | searchable_bytes | 3 | 2130720.000 | 2130720.000 | 2130720.000 | 2130720.000 | bytes |
| agentflow | branches | edit_1pct-edit_B1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-edit_B1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-edit_B1 | current | wall | 3 | 7.018 | 6.909 | 7.220 | 7.220 | ms |
| agentflow | branches | edit_1pct-edit_B1 | rg | peak_rss | 2 | 6496256.000 | 6496256.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | branches | edit_1pct-edit_B1 | rg | wall | 3 | 7.903 | 9.091 | 13.087 | 13.087 | ms |
| agentflow | branches | edit_1pct-edit_B2 | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-edit_B2 | current | index_size | 3 | 3484159.000 | 3484159.000 | 3484159.000 | 3484159.000 | bytes |
| agentflow | branches | edit_1pct-edit_B2 | current | peak_rss | 2 | 11337728.000 | 11337728.000 | 11354112.000 | 11354112.000 | bytes |
| agentflow | branches | edit_1pct-edit_B2 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-edit_B2 | current | searchable_bytes | 3 | 2132257.000 | 2132257.000 | 2132257.000 | 2132257.000 | bytes |
| agentflow | branches | edit_1pct-edit_B2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-edit_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-edit_B2 | current | wall | 3 | 7.094 | 7.675 | 9.190 | 9.190 | ms |
| agentflow | branches | edit_1pct-edit_B2 | rg | peak_rss | 2 | 6496256.000 | 6496256.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | branches | edit_1pct-edit_B2 | rg | wall | 3 | 6.660 | 6.727 | 7.407 | 7.407 | ms |
| agentflow | branches | edit_1pct-revisit_A | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-revisit_A | current | index_size | 3 | 3500666.000 | 3500666.000 | 3500666.000 | 3500666.000 | bytes |
| agentflow | branches | edit_1pct-revisit_A | current | peak_rss | 2 | 10207232.000 | 10207232.000 | 10223616.000 | 10223616.000 | bytes |
| agentflow | branches | edit_1pct-revisit_A | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-revisit_A | current | searchable_bytes | 3 | 2132257.000 | 2132257.000 | 2132257.000 | 2132257.000 | bytes |
| agentflow | branches | edit_1pct-revisit_A | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-revisit_A | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-revisit_A | current | wall | 3 | 5.713 | 5.899 | 6.613 | 6.613 | ms |
| agentflow | branches | edit_1pct-revisit_A | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | branches | edit_1pct-revisit_A | rg | wall | 3 | 6.946 | 7.166 | 7.977 | 7.977 | ms |
| agentflow | branches | edit_1pct-revisit_B | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-revisit_B | current | index_size | 3 | 3500666.000 | 3500666.000 | 3500666.000 | 3500666.000 | bytes |
| agentflow | branches | edit_1pct-revisit_B | current | peak_rss | 2 | 10256384.000 | 10256384.000 | 10272768.000 | 10272768.000 | bytes |
| agentflow | branches | edit_1pct-revisit_B | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-revisit_B | current | searchable_bytes | 3 | 2132257.000 | 2132257.000 | 2132257.000 | 2132257.000 | bytes |
| agentflow | branches | edit_1pct-revisit_B | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-revisit_B | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-revisit_B | current | wall | 3 | 5.477 | 5.656 | 6.085 | 6.085 | ms |
| agentflow | branches | edit_1pct-revisit_B | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | branches | edit_1pct-revisit_B | rg | wall | 3 | 6.002 | 6.698 | 8.161 | 8.161 | ms |
| agentflow | branches | edit_1pct-switch_A1 | current | index_size | 3 | 3240038.000 | 3240038.000 | 3240038.000 | 3240038.000 | bytes |
| agentflow | branches | edit_1pct-switch_A1 | current | peak_rss | 2 | 9650176.000 | 9650176.000 | 9650176.000 | 9650176.000 | bytes |
| agentflow | branches | edit_1pct-switch_A1 | current | searchable_bytes | 3 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | branches | edit_1pct-switch_A1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-switch_A1 | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-switch_A1 | current | wall | 3 | 5.091 | 5.013 | 5.175 | 5.175 | ms |
| agentflow | branches | edit_1pct-switch_A1 | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | branches | edit_1pct-switch_A1 | rg | wall | 3 | 8.286 | 8.405 | 9.557 | 9.557 | ms |
| agentflow | branches | edit_1pct-switch_A2 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-switch_A2 | current | index_size | 3 | 3369295.000 | 3369295.000 | 3369295.000 | 3369295.000 | bytes |
| agentflow | branches | edit_1pct-switch_A2 | current | peak_rss | 2 | 10313728.000 | 10313728.000 | 10321920.000 | 10321920.000 | bytes |
| agentflow | branches | edit_1pct-switch_A2 | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-switch_A2 | current | searchable_bytes | 3 | 2130720.000 | 2130720.000 | 2130720.000 | 2130720.000 | bytes |
| agentflow | branches | edit_1pct-switch_A2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-switch_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-switch_A2 | current | wall | 3 | 5.764 | 6.187 | 7.450 | 7.450 | ms |
| agentflow | branches | edit_1pct-switch_A2 | rg | peak_rss | 2 | 6463488.000 | 6463488.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | branches | edit_1pct-switch_A2 | rg | wall | 3 | 8.017 | 7.741 | 9.637 | 9.637 | ms |
| agentflow | branches | edit_1pct-switch_B1 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-switch_B1 | current | index_size | 3 | 3304675.000 | 3304675.000 | 3304675.000 | 3304675.000 | bytes |
| agentflow | branches | edit_1pct-switch_B1 | current | peak_rss | 2 | 10207232.000 | 10207232.000 | 10207232.000 | 10207232.000 | bytes |
| agentflow | branches | edit_1pct-switch_B1 | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-switch_B1 | current | searchable_bytes | 3 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | branches | edit_1pct-switch_B1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-switch_B1 | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-switch_B1 | current | wall | 3 | 5.804 | 5.996 | 6.551 | 6.551 | ms |
| agentflow | branches | edit_1pct-switch_B1 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | branches | edit_1pct-switch_B1 | rg | wall | 3 | 6.396 | 6.514 | 7.134 | 7.134 | ms |
| agentflow | branches | edit_1pct-switch_B2 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_1pct-switch_B2 | current | index_size | 3 | 3435040.000 | 3435040.000 | 3435040.000 | 3435040.000 | bytes |
| agentflow | branches | edit_1pct-switch_B2 | current | peak_rss | 2 | 10256384.000 | 10256384.000 | 10272768.000 | 10272768.000 | bytes |
| agentflow | branches | edit_1pct-switch_B2 | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_1pct-switch_B2 | current | searchable_bytes | 3 | 2130720.000 | 2130720.000 | 2130720.000 | 2130720.000 | bytes |
| agentflow | branches | edit_1pct-switch_B2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_1pct-switch_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_1pct-switch_B2 | current | wall | 3 | 6.390 | 6.471 | 7.521 | 7.521 | ms |
| agentflow | branches | edit_1pct-switch_B2 | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | branches | edit_1pct-switch_B2 | rg | wall | 3 | 7.590 | 7.247 | 7.993 | 7.993 | ms |
| agentflow | branches | edit_50pct-commit_A1 | current | index_size | 3 | 4084531.000 | 4084531.000 | 4084531.000 | 4084531.000 | bytes |
| agentflow | branches | edit_50pct-commit_A1 | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10387456.000 | 10387456.000 | bytes |
| agentflow | branches | edit_50pct-commit_A1 | current | searchable_bytes | 3 | 2221403.000 | 2221403.000 | 2221403.000 | 2221403.000 | bytes |
| agentflow | branches | edit_50pct-commit_A1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-commit_A1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-commit_A1 | current | wall | 3 | 5.957 | 6.071 | 6.359 | 6.359 | ms |
| agentflow | branches | edit_50pct-commit_A1 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | branches | edit_50pct-commit_A1 | rg | wall | 3 | 6.634 | 7.071 | 8.517 | 8.517 | ms |
| agentflow | branches | edit_50pct-commit_A2 | current | index_size | 3 | 5788878.000 | 5788878.000 | 5788878.000 | 5788878.000 | bytes |
| agentflow | branches | edit_50pct-commit_A2 | current | peak_rss | 2 | 10395648.000 | 10395648.000 | 10403840.000 | 10403840.000 | bytes |
| agentflow | branches | edit_50pct-commit_A2 | current | searchable_bytes | 3 | 2313623.000 | 2313623.000 | 2313623.000 | 2313623.000 | bytes |
| agentflow | branches | edit_50pct-commit_A2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-commit_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-commit_A2 | current | wall | 3 | 6.895 | 6.930 | 7.643 | 7.643 | ms |
| agentflow | branches | edit_50pct-commit_A2 | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | branches | edit_50pct-commit_A2 | rg | wall | 3 | 6.115 | 6.566 | 7.817 | 7.817 | ms |
| agentflow | branches | edit_50pct-commit_B1 | current | index_size | 3 | 4926966.000 | 4926966.000 | 4926966.000 | 4926966.000 | bytes |
| agentflow | branches | edit_50pct-commit_B1 | current | peak_rss | 2 | 10412032.000 | 10412032.000 | 10436608.000 | 10436608.000 | bytes |
| agentflow | branches | edit_50pct-commit_B1 | current | searchable_bytes | 3 | 2221403.000 | 2221403.000 | 2221403.000 | 2221403.000 | bytes |
| agentflow | branches | edit_50pct-commit_B1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-commit_B1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-commit_B1 | current | wall | 3 | 6.555 | 6.709 | 7.233 | 7.233 | ms |
| agentflow | branches | edit_50pct-commit_B1 | rg | peak_rss | 2 | 6463488.000 | 6463488.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | branches | edit_50pct-commit_B1 | rg | wall | 3 | 6.690 | 6.692 | 7.197 | 7.197 | ms |
| agentflow | branches | edit_50pct-commit_B2 | current | index_size | 3 | 6648811.000 | 6648811.000 | 6648811.000 | 6648811.000 | bytes |
| agentflow | branches | edit_50pct-commit_B2 | current | peak_rss | 2 | 10420224.000 | 10420224.000 | 10452992.000 | 10452992.000 | bytes |
| agentflow | branches | edit_50pct-commit_B2 | current | searchable_bytes | 3 | 2313623.000 | 2313623.000 | 2313623.000 | 2313623.000 | bytes |
| agentflow | branches | edit_50pct-commit_B2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-commit_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-commit_B2 | current | wall | 3 | 7.096 | 7.237 | 8.048 | 8.048 | ms |
| agentflow | branches | edit_50pct-commit_B2 | rg | peak_rss | 2 | 6422528.000 | 6422528.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | branches | edit_50pct-commit_B2 | rg | wall | 3 | 7.201 | 7.381 | 8.268 | 8.268 | ms |
| agentflow | branches | edit_50pct-edit_A1 | current | extracted | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-edit_A1 | current | index_size | 3 | 4068024.000 | 4068024.000 | 4068024.000 | 4068024.000 | bytes |
| agentflow | branches | edit_50pct-edit_A1 | current | peak_rss | 2 | 16384000.000 | 16384000.000 | 16465920.000 | 16465920.000 | bytes |
| agentflow | branches | edit_50pct-edit_A1 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-edit_A1 | current | searchable_bytes | 3 | 2221403.000 | 2221403.000 | 2221403.000 | 2221403.000 | bytes |
| agentflow | branches | edit_50pct-edit_A1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-edit_A1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-edit_A1 | current | wall | 3 | 16.524 | 16.590 | 18.305 | 18.305 | ms |
| agentflow | branches | edit_50pct-edit_A1 | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6602752.000 | 6602752.000 | bytes |
| agentflow | branches | edit_50pct-edit_A1 | rg | wall | 3 | 8.138 | 7.860 | 8.179 | 8.179 | ms |
| agentflow | branches | edit_50pct-edit_A2 | current | extracted | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-edit_A2 | current | index_size | 3 | 5772371.000 | 5772371.000 | 5772371.000 | 5772371.000 | bytes |
| agentflow | branches | edit_50pct-edit_A2 | current | peak_rss | 2 | 17547264.000 | 17547264.000 | 17727488.000 | 17727488.000 | bytes |
| agentflow | branches | edit_50pct-edit_A2 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-edit_A2 | current | searchable_bytes | 3 | 2313623.000 | 2313623.000 | 2313623.000 | 2313623.000 | bytes |
| agentflow | branches | edit_50pct-edit_A2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-edit_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-edit_A2 | current | wall | 3 | 15.540 | 15.567 | 15.632 | 15.632 | ms |
| agentflow | branches | edit_50pct-edit_A2 | rg | peak_rss | 2 | 6455296.000 | 6455296.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | branches | edit_50pct-edit_A2 | rg | wall | 3 | 6.499 | 7.004 | 9.067 | 9.067 | ms |
| agentflow | branches | edit_50pct-edit_B1 | current | extracted | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-edit_B1 | current | index_size | 3 | 4910459.000 | 4910459.000 | 4910459.000 | 4910459.000 | bytes |
| agentflow | branches | edit_50pct-edit_B1 | current | peak_rss | 2 | 16629760.000 | 16629760.000 | 16777216.000 | 16777216.000 | bytes |
| agentflow | branches | edit_50pct-edit_B1 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-edit_B1 | current | searchable_bytes | 3 | 2221403.000 | 2221403.000 | 2221403.000 | 2221403.000 | bytes |
| agentflow | branches | edit_50pct-edit_B1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-edit_B1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-edit_B1 | current | wall | 3 | 15.096 | 15.426 | 16.288 | 16.288 | ms |
| agentflow | branches | edit_50pct-edit_B1 | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | branches | edit_50pct-edit_B1 | rg | wall | 3 | 7.696 | 7.474 | 8.677 | 8.677 | ms |
| agentflow | branches | edit_50pct-edit_B2 | current | extracted | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-edit_B2 | current | index_size | 3 | 6632304.000 | 6632304.000 | 6632304.000 | 6632304.000 | bytes |
| agentflow | branches | edit_50pct-edit_B2 | current | peak_rss | 2 | 16531456.000 | 16531456.000 | 16744448.000 | 16744448.000 | bytes |
| agentflow | branches | edit_50pct-edit_B2 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-edit_B2 | current | searchable_bytes | 3 | 2313623.000 | 2313623.000 | 2313623.000 | 2313623.000 | bytes |
| agentflow | branches | edit_50pct-edit_B2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-edit_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-edit_B2 | current | wall | 3 | 15.674 | 15.692 | 15.875 | 15.875 | ms |
| agentflow | branches | edit_50pct-edit_B2 | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6586368.000 | 6586368.000 | bytes |
| agentflow | branches | edit_50pct-edit_B2 | rg | wall | 3 | 7.941 | 7.786 | 8.353 | 8.353 | ms |
| agentflow | branches | edit_50pct-revisit_A | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-revisit_A | current | index_size | 3 | 6648811.000 | 6648811.000 | 6648811.000 | 6648811.000 | bytes |
| agentflow | branches | edit_50pct-revisit_A | current | peak_rss | 2 | 10502144.000 | 10502144.000 | 10518528.000 | 10518528.000 | bytes |
| agentflow | branches | edit_50pct-revisit_A | current | reused | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-revisit_A | current | searchable_bytes | 3 | 2313623.000 | 2313623.000 | 2313623.000 | 2313623.000 | bytes |
| agentflow | branches | edit_50pct-revisit_A | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-revisit_A | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-revisit_A | current | wall | 3 | 7.418 | 7.352 | 8.246 | 8.246 | ms |
| agentflow | branches | edit_50pct-revisit_A | rg | peak_rss | 2 | 6438912.000 | 6438912.000 | 6455296.000 | 6455296.000 | bytes |
| agentflow | branches | edit_50pct-revisit_A | rg | wall | 3 | 7.161 | 7.212 | 7.977 | 7.977 | ms |
| agentflow | branches | edit_50pct-revisit_B | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-revisit_B | current | index_size | 3 | 6648811.000 | 6648811.000 | 6648811.000 | 6648811.000 | bytes |
| agentflow | branches | edit_50pct-revisit_B | current | peak_rss | 2 | 10534912.000 | 10534912.000 | 10567680.000 | 10567680.000 | bytes |
| agentflow | branches | edit_50pct-revisit_B | current | reused | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-revisit_B | current | searchable_bytes | 3 | 2313623.000 | 2313623.000 | 2313623.000 | 2313623.000 | bytes |
| agentflow | branches | edit_50pct-revisit_B | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-revisit_B | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-revisit_B | current | wall | 3 | 6.436 | 6.503 | 6.650 | 6.650 | ms |
| agentflow | branches | edit_50pct-revisit_B | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | branches | edit_50pct-revisit_B | rg | wall | 3 | 7.199 | 6.958 | 7.795 | 7.795 | ms |
| agentflow | branches | edit_50pct-switch_A1 | current | index_size | 3 | 3240038.000 | 3240038.000 | 3240038.000 | 3240038.000 | bytes |
| agentflow | branches | edit_50pct-switch_A1 | current | peak_rss | 2 | 9666560.000 | 9666560.000 | 9699328.000 | 9699328.000 | bytes |
| agentflow | branches | edit_50pct-switch_A1 | current | searchable_bytes | 3 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | branches | edit_50pct-switch_A1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-switch_A1 | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_50pct-switch_A1 | current | wall | 3 | 4.546 | 4.487 | 4.638 | 4.638 | ms |
| agentflow | branches | edit_50pct-switch_A1 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | branches | edit_50pct-switch_A1 | rg | wall | 3 | 7.483 | 7.313 | 8.458 | 8.458 | ms |
| agentflow | branches | edit_50pct-switch_A2 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-switch_A2 | current | index_size | 3 | 4926966.000 | 4926966.000 | 4926966.000 | 4926966.000 | bytes |
| agentflow | branches | edit_50pct-switch_A2 | current | peak_rss | 2 | 10493952.000 | 10493952.000 | 10502144.000 | 10502144.000 | bytes |
| agentflow | branches | edit_50pct-switch_A2 | current | reused | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-switch_A2 | current | searchable_bytes | 3 | 2221403.000 | 2221403.000 | 2221403.000 | 2221403.000 | bytes |
| agentflow | branches | edit_50pct-switch_A2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-switch_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-switch_A2 | current | wall | 3 | 6.816 | 6.701 | 6.884 | 6.884 | ms |
| agentflow | branches | edit_50pct-switch_A2 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | branches | edit_50pct-switch_A2 | rg | wall | 3 | 7.453 | 6.892 | 7.471 | 7.471 | ms |
| agentflow | branches | edit_50pct-switch_B1 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-switch_B1 | current | index_size | 3 | 4084425.000 | 4084425.000 | 4084425.000 | 4084425.000 | bytes |
| agentflow | branches | edit_50pct-switch_B1 | current | peak_rss | 2 | 10477568.000 | 10477568.000 | 10485760.000 | 10485760.000 | bytes |
| agentflow | branches | edit_50pct-switch_B1 | current | reused | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-switch_B1 | current | searchable_bytes | 3 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | branches | edit_50pct-switch_B1 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-switch_B1 | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | branches | edit_50pct-switch_B1 | current | wall | 3 | 5.932 | 6.252 | 6.915 | 6.915 | ms |
| agentflow | branches | edit_50pct-switch_B1 | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | branches | edit_50pct-switch_B1 | rg | wall | 3 | 6.761 | 6.820 | 7.086 | 7.086 | ms |
| agentflow | branches | edit_50pct-switch_B2 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | branches | edit_50pct-switch_B2 | current | index_size | 3 | 5788878.000 | 5788878.000 | 5788878.000 | 5788878.000 | bytes |
| agentflow | branches | edit_50pct-switch_B2 | current | peak_rss | 2 | 10526720.000 | 10526720.000 | 10567680.000 | 10567680.000 | bytes |
| agentflow | branches | edit_50pct-switch_B2 | current | reused | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | branches | edit_50pct-switch_B2 | current | searchable_bytes | 3 | 2221403.000 | 2221403.000 | 2221403.000 | 2221403.000 | bytes |
| agentflow | branches | edit_50pct-switch_B2 | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | branches | edit_50pct-switch_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | branches | edit_50pct-switch_B2 | current | wall | 3 | 6.560 | 6.482 | 6.570 | 6.570 | ms |
| agentflow | branches | edit_50pct-switch_B2 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | branches | edit_50pct-switch_B2 | rg | wall | 3 | 6.514 | 6.686 | 7.330 | 7.330 | ms |
| agentflow | build | default | current | index_size | 3 | 3240038.000 | 3240038.000 | 3240038.000 | 3240038.000 | bytes |
| agentflow | build | default | current | peak_rss | 2 | 28090368.000 | 28090368.000 | 28459008.000 | 28459008.000 | bytes |
| agentflow | build | default | current | searchable_bytes | 3 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | build | default | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | build | default | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | build | default | current | wall | 3 | 64.746 | 68.941 | 78.362 | 78.362 | ms |
| agentflow | history | revisit_0 | current | extracted | 2 | 27.000 | 27.000 | 27.000 | 27.000 | count |
| agentflow | history | revisit_0 | current | index_size | 2 | 4843732.000 | 4843732.000 | 4843732.000 | 4843732.000 | bytes |
| agentflow | history | revisit_0 | current | peak_rss | 2 | 17227776.000 | 17227776.000 | 17334272.000 | 17334272.000 | bytes |
| agentflow | history | revisit_0 | current | reused | 2 | 27.000 | 27.000 | 27.000 | 27.000 | count |
| agentflow | history | revisit_0 | current | searchable_bytes | 2 | 588428.000 | 588428.000 | 588428.000 | 588428.000 | bytes |
| agentflow | history | revisit_0 | current | searchable_files | 2 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| agentflow | history | revisit_0 | current | segments | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | history | revisit_0 | current | wall | 2 | 21.952 | 21.952 | 22.496 | 22.496 | ms |
| agentflow | history | revisit_0 | rg | peak_rss | 2 | 6471680.000 | 6471680.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | revisit_0 | rg | wall | 2 | 5.526 | 5.526 | 6.186 | 6.186 | ms |
| agentflow | history | revisit_28 | current | extracted | 2 | 133.000 | 133.000 | 133.000 | 133.000 | count |
| agentflow | history | revisit_28 | current | index_size | 2 | 7328973.000 | 7328973.000 | 7328973.000 | 7328973.000 | bytes |
| agentflow | history | revisit_28 | current | peak_rss | 2 | 23732224.000 | 23732224.000 | 23953408.000 | 23953408.000 | bytes |
| agentflow | history | revisit_28 | current | reused | 2 | 133.000 | 133.000 | 133.000 | 133.000 | count |
| agentflow | history | revisit_28 | current | searchable_bytes | 2 | 1736280.000 | 1736280.000 | 1736280.000 | 1736280.000 | bytes |
| agentflow | history | revisit_28 | current | searchable_files | 2 | 134.000 | 134.000 | 134.000 | 134.000 | count |
| agentflow | history | revisit_28 | current | segments | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | history | revisit_28 | current | wall | 2 | 33.892 | 33.892 | 34.953 | 34.953 | ms |
| agentflow | history | revisit_28 | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | revisit_28 | rg | wall | 2 | 6.780 | 6.780 | 7.685 | 7.685 | ms |
| agentflow | history | revisit_56 | current | extracted | 2 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | history | revisit_56 | current | index_size | 2 | 7332871.000 | 7332871.000 | 7332871.000 | 7332871.000 | bytes |
| agentflow | history | revisit_56 | current | peak_rss | 2 | 10715136.000 | 10715136.000 | 10715136.000 | 10715136.000 | bytes |
| agentflow | history | revisit_56 | current | reused | 2 | 85.000 | 85.000 | 85.000 | 85.000 | count |
| agentflow | history | revisit_56 | current | searchable_bytes | 2 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | history | revisit_56 | current | searchable_files | 2 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | history | revisit_56 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | revisit_56 | current | wall | 2 | 6.898 | 6.898 | 7.096 | 7.096 | ms |
| agentflow | history | revisit_56 | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6602752.000 | 6602752.000 | bytes |
| agentflow | history | revisit_56 | rg | wall | 2 | 6.380 | 6.380 | 7.009 | 7.009 | ms |
| agentflow | history | step_001 | current | extracted | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_001 | current | index_size | 2 | 1282944.000 | 1282944.000 | 1282944.000 | 1282944.000 | bytes |
| agentflow | history | step_001 | current | peak_rss | 2 | 14426112.000 | 14426112.000 | 14548992.000 | 14548992.000 | bytes |
| agentflow | history | step_001 | current | reused | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_001 | current | searchable_bytes | 2 | 618234.000 | 618234.000 | 618234.000 | 618234.000 | bytes |
| agentflow | history | step_001 | current | searchable_files | 2 | 36.000 | 36.000 | 36.000 | 36.000 | count |
| agentflow | history | step_001 | current | segments | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | history | step_001 | current | wall | 2 | 9.593 | 9.593 | 10.748 | 10.748 | ms |
| agentflow | history | step_001 | rg | peak_rss | 2 | 6463488.000 | 6463488.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | history | step_001 | rg | wall | 2 | 4.872 | 4.872 | 4.922 | 4.922 | ms |
| agentflow | history | step_002 | current | extracted | 2 | 35.000 | 35.000 | 35.000 | 35.000 | count |
| agentflow | history | step_002 | current | index_size | 2 | 1741836.000 | 1741836.000 | 1741836.000 | 1741836.000 | bytes |
| agentflow | history | step_002 | current | peak_rss | 2 | 14344192.000 | 14344192.000 | 14385152.000 | 14385152.000 | bytes |
| agentflow | history | step_002 | current | reused | 2 | 35.000 | 35.000 | 35.000 | 35.000 | count |
| agentflow | history | step_002 | current | searchable_bytes | 2 | 634045.000 | 634045.000 | 634045.000 | 634045.000 | bytes |
| agentflow | history | step_002 | current | searchable_files | 2 | 45.000 | 45.000 | 45.000 | 45.000 | count |
| agentflow | history | step_002 | current | segments | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | history | step_002 | current | wall | 2 | 10.392 | 10.392 | 10.431 | 10.431 | ms |
| agentflow | history | step_002 | rg | peak_rss | 2 | 6471680.000 | 6471680.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | history | step_002 | rg | wall | 2 | 5.578 | 5.578 | 7.486 | 7.486 | ms |
| agentflow | history | step_003 | current | extracted | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | history | step_003 | current | index_size | 2 | 1779974.000 | 1779974.000 | 1779974.000 | 1779974.000 | bytes |
| agentflow | history | step_003 | current | peak_rss | 2 | 11100160.000 | 11100160.000 | 11141120.000 | 11141120.000 | bytes |
| agentflow | history | step_003 | current | reused | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | history | step_003 | current | searchable_bytes | 2 | 640030.000 | 640030.000 | 640030.000 | 640030.000 | bytes |
| agentflow | history | step_003 | current | searchable_files | 2 | 46.000 | 46.000 | 46.000 | 46.000 | count |
| agentflow | history | step_003 | current | segments | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_003 | current | wall | 2 | 6.397 | 6.397 | 6.412 | 6.412 | ms |
| agentflow | history | step_003 | rg | peak_rss | 2 | 6479872.000 | 6479872.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_003 | rg | wall | 2 | 4.787 | 4.787 | 4.929 | 4.929 | ms |
| agentflow | history | step_004 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_004 | current | index_size | 2 | 1840632.000 | 1840632.000 | 1840632.000 | 1840632.000 | bytes |
| agentflow | history | step_004 | current | peak_rss | 2 | 11362304.000 | 11362304.000 | 11370496.000 | 11370496.000 | bytes |
| agentflow | history | step_004 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_004 | current | searchable_bytes | 2 | 648407.000 | 648407.000 | 648407.000 | 648407.000 | bytes |
| agentflow | history | step_004 | current | searchable_files | 2 | 48.000 | 48.000 | 48.000 | 48.000 | count |
| agentflow | history | step_004 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | step_004 | current | wall | 2 | 6.765 | 6.765 | 6.921 | 6.921 | ms |
| agentflow | history | step_004 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6586368.000 | 6586368.000 | bytes |
| agentflow | history | step_004 | rg | wall | 2 | 3.723 | 3.723 | 3.910 | 3.910 | ms |
| agentflow | history | step_005 | current | extracted | 2 | 27.000 | 27.000 | 27.000 | 27.000 | count |
| agentflow | history | step_005 | current | index_size | 2 | 2743780.000 | 2743780.000 | 2743780.000 | 2743780.000 | bytes |
| agentflow | history | step_005 | current | peak_rss | 2 | 16613376.000 | 16613376.000 | 16793600.000 | 16793600.000 | bytes |
| agentflow | history | step_005 | current | reused | 2 | 27.000 | 27.000 | 27.000 | 27.000 | count |
| agentflow | history | step_005 | current | searchable_bytes | 2 | 686829.000 | 686829.000 | 686829.000 | 686829.000 | bytes |
| agentflow | history | step_005 | current | searchable_files | 2 | 51.000 | 51.000 | 51.000 | 51.000 | count |
| agentflow | history | step_005 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | step_005 | current | wall | 2 | 19.848 | 19.848 | 19.860 | 19.860 | ms |
| agentflow | history | step_005 | rg | peak_rss | 2 | 6430720.000 | 6430720.000 | 6455296.000 | 6455296.000 | bytes |
| agentflow | history | step_005 | rg | wall | 2 | 4.639 | 4.639 | 5.672 | 5.672 | ms |
| agentflow | history | step_006 | current | extracted | 2 | 50.000 | 50.000 | 50.000 | 50.000 | count |
| agentflow | history | step_006 | current | index_size | 2 | 3799561.000 | 3799561.000 | 3799561.000 | 3799561.000 | bytes |
| agentflow | history | step_006 | current | peak_rss | 2 | 17637376.000 | 17637376.000 | 17940480.000 | 17940480.000 | bytes |
| agentflow | history | step_006 | current | reused | 2 | 50.000 | 50.000 | 50.000 | 50.000 | count |
| agentflow | history | step_006 | current | searchable_bytes | 2 | 777428.000 | 777428.000 | 777428.000 | 777428.000 | bytes |
| agentflow | history | step_006 | current | searchable_files | 2 | 69.000 | 69.000 | 69.000 | 69.000 | count |
| agentflow | history | step_006 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | step_006 | current | wall | 2 | 21.154 | 21.154 | 21.785 | 21.785 | ms |
| agentflow | history | step_006 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_006 | rg | wall | 2 | 6.101 | 6.101 | 6.124 | 6.124 | ms |
| agentflow | history | step_007 | current | extracted | 2 | 29.000 | 29.000 | 29.000 | 29.000 | count |
| agentflow | history | step_007 | current | index_size | 2 | 4176171.000 | 4176171.000 | 4176171.000 | 4176171.000 | bytes |
| agentflow | history | step_007 | current | peak_rss | 2 | 13459456.000 | 13459456.000 | 13615104.000 | 13615104.000 | bytes |
| agentflow | history | step_007 | current | reused | 2 | 29.000 | 29.000 | 29.000 | 29.000 | count |
| agentflow | history | step_007 | current | searchable_bytes | 2 | 809608.000 | 809608.000 | 809608.000 | 809608.000 | bytes |
| agentflow | history | step_007 | current | searchable_files | 2 | 70.000 | 70.000 | 70.000 | 70.000 | count |
| agentflow | history | step_007 | current | segments | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| agentflow | history | step_007 | current | wall | 2 | 9.859 | 9.859 | 10.017 | 10.017 | ms |
| agentflow | history | step_007 | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | history | step_007 | rg | wall | 2 | 6.081 | 6.081 | 6.644 | 6.644 | ms |
| agentflow | history | step_008 | current | extracted | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| agentflow | history | step_008 | current | index_size | 2 | 4647956.000 | 4647956.000 | 4647956.000 | 4647956.000 | bytes |
| agentflow | history | step_008 | current | peak_rss | 2 | 14655488.000 | 14655488.000 | 14696448.000 | 14696448.000 | bytes |
| agentflow | history | step_008 | current | reused | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| agentflow | history | step_008 | current | searchable_bytes | 2 | 879864.000 | 879864.000 | 879864.000 | 879864.000 | bytes |
| agentflow | history | step_008 | current | searchable_files | 2 | 73.000 | 73.000 | 73.000 | 73.000 | count |
| agentflow | history | step_008 | current | segments | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| agentflow | history | step_008 | current | wall | 2 | 11.142 | 11.142 | 11.788 | 11.788 | ms |
| agentflow | history | step_008 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_008 | rg | wall | 2 | 5.799 | 5.799 | 6.289 | 6.289 | ms |
| agentflow | history | step_009 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_009 | current | index_size | 2 | 4798308.000 | 4798308.000 | 4798308.000 | 4798308.000 | bytes |
| agentflow | history | step_009 | current | peak_rss | 2 | 12304384.000 | 12304384.000 | 12304384.000 | 12304384.000 | bytes |
| agentflow | history | step_009 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_009 | current | searchable_bytes | 2 | 880974.000 | 880974.000 | 880974.000 | 880974.000 | bytes |
| agentflow | history | step_009 | current | searchable_files | 2 | 73.000 | 73.000 | 73.000 | 73.000 | count |
| agentflow | history | step_009 | current | segments | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| agentflow | history | step_009 | current | wall | 2 | 7.407 | 7.407 | 7.591 | 7.591 | ms |
| agentflow | history | step_009 | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | history | step_009 | rg | wall | 2 | 6.150 | 6.150 | 6.463 | 6.463 | ms |
| agentflow | history | step_010 | current | extracted | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| agentflow | history | step_010 | current | index_size | 2 | 5272449.000 | 5272449.000 | 5272449.000 | 5272449.000 | bytes |
| agentflow | history | step_010 | current | peak_rss | 2 | 14630912.000 | 14630912.000 | 14663680.000 | 14663680.000 | bytes |
| agentflow | history | step_010 | current | reused | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| agentflow | history | step_010 | current | searchable_bytes | 2 | 926450.000 | 926450.000 | 926450.000 | 926450.000 | bytes |
| agentflow | history | step_010 | current | searchable_files | 2 | 76.000 | 76.000 | 76.000 | 76.000 | count |
| agentflow | history | step_010 | current | segments | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| agentflow | history | step_010 | current | wall | 2 | 10.867 | 10.867 | 11.184 | 11.184 | ms |
| agentflow | history | step_010 | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_010 | rg | wall | 2 | 5.186 | 5.186 | 5.491 | 5.491 | ms |
| agentflow | history | step_011 | current | extracted | 2 | 19.000 | 19.000 | 19.000 | 19.000 | count |
| agentflow | history | step_011 | current | index_size | 2 | 6131589.000 | 6131589.000 | 6131589.000 | 6131589.000 | bytes |
| agentflow | history | step_011 | current | peak_rss | 2 | 16343040.000 | 16343040.000 | 16433152.000 | 16433152.000 | bytes |
| agentflow | history | step_011 | current | reused | 2 | 19.000 | 19.000 | 19.000 | 19.000 | count |
| agentflow | history | step_011 | current | searchable_bytes | 2 | 963718.000 | 963718.000 | 963718.000 | 963718.000 | bytes |
| agentflow | history | step_011 | current | searchable_files | 2 | 79.000 | 79.000 | 79.000 | 79.000 | count |
| agentflow | history | step_011 | current | segments | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| agentflow | history | step_011 | current | wall | 2 | 19.045 | 19.045 | 19.144 | 19.144 | ms |
| agentflow | history | step_011 | rg | peak_rss | 2 | 6496256.000 | 6496256.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | step_011 | rg | wall | 2 | 6.116 | 6.116 | 7.257 | 7.257 | ms |
| agentflow | history | step_012 | current | extracted | 2 | 44.000 | 44.000 | 44.000 | 44.000 | count |
| agentflow | history | step_012 | current | index_size | 2 | 6799289.000 | 6799289.000 | 6799289.000 | 6799289.000 | bytes |
| agentflow | history | step_012 | current | peak_rss | 2 | 15843328.000 | 15843328.000 | 16023552.000 | 16023552.000 | bytes |
| agentflow | history | step_012 | current | reused | 2 | 44.000 | 44.000 | 44.000 | 44.000 | count |
| agentflow | history | step_012 | current | searchable_bytes | 2 | 1087807.000 | 1087807.000 | 1087807.000 | 1087807.000 | bytes |
| agentflow | history | step_012 | current | searchable_files | 2 | 85.000 | 85.000 | 85.000 | 85.000 | count |
| agentflow | history | step_012 | current | segments | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| agentflow | history | step_012 | current | wall | 2 | 13.248 | 13.248 | 13.312 | 13.312 | ms |
| agentflow | history | step_012 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_012 | rg | wall | 2 | 6.253 | 6.253 | 6.316 | 6.316 | ms |
| agentflow | history | step_013 | current | extracted | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| agentflow | history | step_013 | current | index_size | 2 | 6632197.000 | 6632197.000 | 6632197.000 | 6632197.000 | bytes |
| agentflow | history | step_013 | current | peak_rss | 2 | 13975552.000 | 13975552.000 | 14073856.000 | 14073856.000 | bytes |
| agentflow | history | step_013 | current | reused | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| agentflow | history | step_013 | current | searchable_bytes | 2 | 1101405.000 | 1101405.000 | 1101405.000 | 1101405.000 | bytes |
| agentflow | history | step_013 | current | searchable_files | 2 | 86.000 | 86.000 | 86.000 | 86.000 | count |
| agentflow | history | step_013 | current | segments | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| agentflow | history | step_013 | current | wall | 2 | 8.637 | 8.637 | 8.956 | 8.956 | ms |
| agentflow | history | step_013 | rg | peak_rss | 2 | 6463488.000 | 6463488.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | history | step_013 | rg | wall | 2 | 5.781 | 5.781 | 6.455 | 6.455 | ms |
| agentflow | history | step_014 | current | extracted | 2 | 33.000 | 33.000 | 33.000 | 33.000 | count |
| agentflow | history | step_014 | current | index_size | 2 | 7195720.000 | 7195720.000 | 7195720.000 | 7195720.000 | bytes |
| agentflow | history | step_014 | current | peak_rss | 2 | 15556608.000 | 15556608.000 | 15630336.000 | 15630336.000 | bytes |
| agentflow | history | step_014 | current | reused | 2 | 33.000 | 33.000 | 33.000 | 33.000 | count |
| agentflow | history | step_014 | current | searchable_bytes | 2 | 1200293.000 | 1200293.000 | 1200293.000 | 1200293.000 | bytes |
| agentflow | history | step_014 | current | searchable_files | 2 | 95.000 | 95.000 | 95.000 | 95.000 | count |
| agentflow | history | step_014 | current | segments | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| agentflow | history | step_014 | current | wall | 2 | 12.070 | 12.070 | 12.516 | 12.516 | ms |
| agentflow | history | step_014 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | history | step_014 | rg | wall | 2 | 5.381 | 5.381 | 5.905 | 5.905 | ms |
| agentflow | history | step_015 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_015 | current | index_size | 2 | 7241602.000 | 7241602.000 | 7241602.000 | 7241602.000 | bytes |
| agentflow | history | step_015 | current | peak_rss | 2 | 11870208.000 | 11870208.000 | 11894784.000 | 11894784.000 | bytes |
| agentflow | history | step_015 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_015 | current | searchable_bytes | 2 | 1202087.000 | 1202087.000 | 1202087.000 | 1202087.000 | bytes |
| agentflow | history | step_015 | current | searchable_files | 2 | 95.000 | 95.000 | 95.000 | 95.000 | count |
| agentflow | history | step_015 | current | segments | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| agentflow | history | step_015 | current | wall | 2 | 6.932 | 6.932 | 6.977 | 6.977 | ms |
| agentflow | history | step_015 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | history | step_015 | rg | wall | 2 | 5.917 | 5.917 | 6.684 | 6.684 | ms |
| agentflow | history | step_016 | current | extracted | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| agentflow | history | step_016 | current | index_size | 2 | 7825617.000 | 7825617.000 | 7825617.000 | 7825617.000 | bytes |
| agentflow | history | step_016 | current | peak_rss | 2 | 15196160.000 | 15196160.000 | 15253504.000 | 15253504.000 | bytes |
| agentflow | history | step_016 | current | reused | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| agentflow | history | step_016 | current | searchable_bytes | 2 | 1279000.000 | 1279000.000 | 1279000.000 | 1279000.000 | bytes |
| agentflow | history | step_016 | current | searchable_files | 2 | 101.000 | 101.000 | 101.000 | 101.000 | count |
| agentflow | history | step_016 | current | segments | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| agentflow | history | step_016 | current | wall | 2 | 12.284 | 12.284 | 12.613 | 12.613 | ms |
| agentflow | history | step_016 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6586368.000 | 6586368.000 | bytes |
| agentflow | history | step_016 | rg | wall | 2 | 5.462 | 5.462 | 5.843 | 5.843 | ms |
| agentflow | history | step_017 | current | extracted | 2 | 43.000 | 43.000 | 43.000 | 43.000 | count |
| agentflow | history | step_017 | current | index_size | 2 | 9102294.000 | 9102294.000 | 9102294.000 | 9102294.000 | bytes |
| agentflow | history | step_017 | current | peak_rss | 2 | 19021824.000 | 19021824.000 | 19316736.000 | 19316736.000 | bytes |
| agentflow | history | step_017 | current | reused | 2 | 43.000 | 43.000 | 43.000 | 43.000 | count |
| agentflow | history | step_017 | current | searchable_bytes | 2 | 1368927.000 | 1368927.000 | 1368927.000 | 1368927.000 | bytes |
| agentflow | history | step_017 | current | searchable_files | 2 | 109.000 | 109.000 | 109.000 | 109.000 | count |
| agentflow | history | step_017 | current | segments | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| agentflow | history | step_017 | current | wall | 2 | 21.943 | 21.943 | 22.317 | 22.317 | ms |
| agentflow | history | step_017 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | history | step_017 | rg | wall | 2 | 6.373 | 6.373 | 6.695 | 6.695 | ms |
| agentflow | history | step_018 | current | extracted | 2 | 41.000 | 41.000 | 41.000 | 41.000 | count |
| agentflow | history | step_018 | current | index_size | 2 | 9846384.000 | 9846384.000 | 9846384.000 | 9846384.000 | bytes |
| agentflow | history | step_018 | current | peak_rss | 2 | 16384000.000 | 16384000.000 | 16515072.000 | 16515072.000 | bytes |
| agentflow | history | step_018 | current | reused | 2 | 41.000 | 41.000 | 41.000 | 41.000 | count |
| agentflow | history | step_018 | current | searchable_bytes | 2 | 1460636.000 | 1460636.000 | 1460636.000 | 1460636.000 | bytes |
| agentflow | history | step_018 | current | searchable_files | 2 | 114.000 | 114.000 | 114.000 | 114.000 | count |
| agentflow | history | step_018 | current | segments | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_018 | current | wall | 2 | 14.382 | 14.382 | 15.274 | 15.274 | ms |
| agentflow | history | step_018 | rg | peak_rss | 2 | 6479872.000 | 6479872.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | step_018 | rg | wall | 2 | 6.240 | 6.240 | 6.317 | 6.317 | ms |
| agentflow | history | step_019 | current | extracted | 2 | 27.000 | 27.000 | 27.000 | 27.000 | count |
| agentflow | history | step_019 | current | index_size | 2 | 12074145.000 | 12074145.000 | 12074145.000 | 12074145.000 | bytes |
| agentflow | history | step_019 | current | peak_rss | 2 | 25812992.000 | 25812992.000 | 25821184.000 | 25821184.000 | bytes |
| agentflow | history | step_019 | current | reused | 2 | 27.000 | 27.000 | 27.000 | 27.000 | count |
| agentflow | history | step_019 | current | searchable_bytes | 2 | 1544973.000 | 1544973.000 | 1544973.000 | 1544973.000 | bytes |
| agentflow | history | step_019 | current | searchable_files | 2 | 121.000 | 121.000 | 121.000 | 121.000 | count |
| agentflow | history | step_019 | current | segments | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_019 | current | wall | 2 | 75.352 | 75.352 | 75.815 | 75.815 | ms |
| agentflow | history | step_019 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6586368.000 | 6586368.000 | bytes |
| agentflow | history | step_019 | rg | wall | 2 | 6.015 | 6.015 | 6.075 | 6.075 | ms |
| agentflow | history | step_020 | current | extracted | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_020 | current | index_size | 2 | 12539849.000 | 12539849.000 | 12539849.000 | 12539849.000 | bytes |
| agentflow | history | step_020 | current | peak_rss | 2 | 14229504.000 | 14229504.000 | 14352384.000 | 14352384.000 | bytes |
| agentflow | history | step_020 | current | reused | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_020 | current | searchable_bytes | 2 | 1553894.000 | 1553894.000 | 1553894.000 | 1553894.000 | bytes |
| agentflow | history | step_020 | current | searchable_files | 2 | 121.000 | 121.000 | 121.000 | 121.000 | count |
| agentflow | history | step_020 | current | segments | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | history | step_020 | current | wall | 2 | 9.856 | 9.856 | 9.914 | 9.914 | ms |
| agentflow | history | step_020 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | history | step_020 | rg | wall | 2 | 5.069 | 5.069 | 5.415 | 5.415 | ms |
| agentflow | history | step_021 | current | extracted | 2 | 30.000 | 30.000 | 30.000 | 30.000 | count |
| agentflow | history | step_021 | current | index_size | 2 | 13168740.000 | 13168740.000 | 13168740.000 | 13168740.000 | bytes |
| agentflow | history | step_021 | current | peak_rss | 2 | 15515648.000 | 15515648.000 | 15581184.000 | 15581184.000 | bytes |
| agentflow | history | step_021 | current | reused | 2 | 30.000 | 30.000 | 30.000 | 30.000 | count |
| agentflow | history | step_021 | current | searchable_bytes | 2 | 1592857.000 | 1592857.000 | 1592857.000 | 1592857.000 | bytes |
| agentflow | history | step_021 | current | searchable_files | 2 | 123.000 | 123.000 | 123.000 | 123.000 | count |
| agentflow | history | step_021 | current | segments | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | history | step_021 | current | wall | 2 | 12.213 | 12.213 | 12.271 | 12.271 | ms |
| agentflow | history | step_021 | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | history | step_021 | rg | wall | 2 | 5.744 | 5.744 | 6.350 | 6.350 | ms |
| agentflow | history | step_022 | current | extracted | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| agentflow | history | step_022 | current | index_size | 2 | 13836167.000 | 13836167.000 | 13836167.000 | 13836167.000 | bytes |
| agentflow | history | step_022 | current | peak_rss | 2 | 15589376.000 | 15589376.000 | 15613952.000 | 15613952.000 | bytes |
| agentflow | history | step_022 | current | reused | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| agentflow | history | step_022 | current | searchable_bytes | 2 | 1656865.000 | 1656865.000 | 1656865.000 | 1656865.000 | bytes |
| agentflow | history | step_022 | current | searchable_files | 2 | 128.000 | 128.000 | 128.000 | 128.000 | count |
| agentflow | history | step_022 | current | segments | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_022 | current | wall | 2 | 13.135 | 13.135 | 13.423 | 13.423 | ms |
| agentflow | history | step_022 | rg | peak_rss | 2 | 6438912.000 | 6438912.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | history | step_022 | rg | wall | 2 | 6.424 | 6.424 | 8.019 | 8.019 | ms |
| agentflow | history | step_023 | current | extracted | 2 | 24.000 | 24.000 | 24.000 | 24.000 | count |
| agentflow | history | step_023 | current | index_size | 2 | 13921248.000 | 13921248.000 | 13921248.000 | 13921248.000 | bytes |
| agentflow | history | step_023 | current | peak_rss | 2 | 14524416.000 | 14524416.000 | 14647296.000 | 14647296.000 | bytes |
| agentflow | history | step_023 | current | reused | 2 | 24.000 | 24.000 | 24.000 | 24.000 | count |
| agentflow | history | step_023 | current | searchable_bytes | 2 | 1691250.000 | 1691250.000 | 1691250.000 | 1691250.000 | bytes |
| agentflow | history | step_023 | current | searchable_files | 2 | 131.000 | 131.000 | 131.000 | 131.000 | count |
| agentflow | history | step_023 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | step_023 | current | wall | 2 | 11.145 | 11.145 | 11.587 | 11.587 | ms |
| agentflow | history | step_023 | rg | peak_rss | 2 | 6496256.000 | 6496256.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_023 | rg | wall | 2 | 5.541 | 5.541 | 6.047 | 6.047 | ms |
| agentflow | history | step_024 | current | extracted | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_024 | current | index_size | 2 | 14446833.000 | 14446833.000 | 14446833.000 | 14446833.000 | bytes |
| agentflow | history | step_024 | current | peak_rss | 2 | 14909440.000 | 14909440.000 | 14925824.000 | 14925824.000 | bytes |
| agentflow | history | step_024 | current | reused | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_024 | current | searchable_bytes | 2 | 1711607.000 | 1711607.000 | 1711607.000 | 1711607.000 | bytes |
| agentflow | history | step_024 | current | searchable_files | 2 | 131.000 | 131.000 | 131.000 | 131.000 | count |
| agentflow | history | step_024 | current | segments | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| agentflow | history | step_024 | current | wall | 2 | 11.039 | 11.039 | 11.216 | 11.216 | ms |
| agentflow | history | step_024 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_024 | rg | wall | 2 | 6.360 | 6.360 | 6.766 | 6.766 | ms |
| agentflow | history | step_025 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_025 | current | index_size | 2 | 14554187.000 | 14554187.000 | 14554187.000 | 14554187.000 | bytes |
| agentflow | history | step_025 | current | peak_rss | 2 | 12156928.000 | 12156928.000 | 12173312.000 | 12173312.000 | bytes |
| agentflow | history | step_025 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_025 | current | searchable_bytes | 2 | 1724682.000 | 1724682.000 | 1724682.000 | 1724682.000 | bytes |
| agentflow | history | step_025 | current | searchable_files | 2 | 133.000 | 133.000 | 133.000 | 133.000 | count |
| agentflow | history | step_025 | current | segments | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| agentflow | history | step_025 | current | wall | 2 | 7.567 | 7.567 | 7.938 | 7.938 | ms |
| agentflow | history | step_025 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_025 | rg | wall | 2 | 6.353 | 6.353 | 6.560 | 6.560 | ms |
| agentflow | history | step_026 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_026 | current | index_size | 2 | 5274874.000 | 5274874.000 | 5274874.000 | 5274874.000 | bytes |
| agentflow | history | step_026 | current | peak_rss | 2 | 11583488.000 | 11583488.000 | 11599872.000 | 11599872.000 | bytes |
| agentflow | history | step_026 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_026 | current | searchable_bytes | 2 | 1724263.000 | 1724263.000 | 1724263.000 | 1724263.000 | bytes |
| agentflow | history | step_026 | current | searchable_files | 2 | 133.000 | 133.000 | 133.000 | 133.000 | count |
| agentflow | history | step_026 | current | segments | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| agentflow | history | step_026 | current | wall | 2 | 8.531 | 8.531 | 8.798 | 8.798 | ms |
| agentflow | history | step_026 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_026 | rg | wall | 2 | 6.586 | 6.586 | 7.856 | 7.856 | ms |
| agentflow | history | step_027 | current | extracted | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_027 | current | index_size | 2 | 5754514.000 | 5754514.000 | 5754514.000 | 5754514.000 | bytes |
| agentflow | history | step_027 | current | peak_rss | 2 | 14688256.000 | 14688256.000 | 14729216.000 | 14729216.000 | bytes |
| agentflow | history | step_027 | current | reused | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_027 | current | searchable_bytes | 2 | 1734215.000 | 1734215.000 | 1734215.000 | 1734215.000 | bytes |
| agentflow | history | step_027 | current | searchable_files | 2 | 134.000 | 134.000 | 134.000 | 134.000 | count |
| agentflow | history | step_027 | current | segments | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| agentflow | history | step_027 | current | wall | 2 | 10.693 | 10.693 | 11.136 | 11.136 | ms |
| agentflow | history | step_027 | rg | peak_rss | 2 | 6496256.000 | 6496256.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | step_027 | rg | wall | 2 | 6.916 | 6.916 | 7.926 | 7.926 | ms |
| agentflow | history | step_028 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_028 | current | index_size | 2 | 5914447.000 | 5914447.000 | 5914447.000 | 5914447.000 | bytes |
| agentflow | history | step_028 | current | peak_rss | 2 | 12771328.000 | 12771328.000 | 12779520.000 | 12779520.000 | bytes |
| agentflow | history | step_028 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_028 | current | searchable_bytes | 2 | 1736280.000 | 1736280.000 | 1736280.000 | 1736280.000 | bytes |
| agentflow | history | step_028 | current | searchable_files | 2 | 134.000 | 134.000 | 134.000 | 134.000 | count |
| agentflow | history | step_028 | current | segments | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| agentflow | history | step_028 | current | wall | 2 | 7.549 | 7.549 | 7.592 | 7.592 | ms |
| agentflow | history | step_028 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | step_028 | rg | wall | 2 | 5.858 | 5.858 | 6.390 | 6.390 | ms |
| agentflow | history | step_029 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_029 | current | index_size | 2 | 5943044.000 | 5943044.000 | 5943044.000 | 5943044.000 | bytes |
| agentflow | history | step_029 | current | peak_rss | 2 | 10919936.000 | 10919936.000 | 10928128.000 | 10928128.000 | bytes |
| agentflow | history | step_029 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_029 | current | searchable_bytes | 2 | 1740086.000 | 1740086.000 | 1740086.000 | 1740086.000 | bytes |
| agentflow | history | step_029 | current | searchable_files | 2 | 135.000 | 135.000 | 135.000 | 135.000 | count |
| agentflow | history | step_029 | current | segments | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| agentflow | history | step_029 | current | wall | 2 | 7.069 | 7.069 | 7.159 | 7.159 | ms |
| agentflow | history | step_029 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | step_029 | rg | wall | 2 | 7.581 | 7.581 | 7.716 | 7.716 | ms |
| agentflow | history | step_030 | current | extracted | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| agentflow | history | step_030 | current | index_size | 2 | 6288469.000 | 6288469.000 | 6288469.000 | 6288469.000 | bytes |
| agentflow | history | step_030 | current | peak_rss | 2 | 14573568.000 | 14573568.000 | 14680064.000 | 14680064.000 | bytes |
| agentflow | history | step_030 | current | reused | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| agentflow | history | step_030 | current | searchable_bytes | 2 | 1755088.000 | 1755088.000 | 1755088.000 | 1755088.000 | bytes |
| agentflow | history | step_030 | current | searchable_files | 2 | 136.000 | 136.000 | 136.000 | 136.000 | count |
| agentflow | history | step_030 | current | segments | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| agentflow | history | step_030 | current | wall | 2 | 9.966 | 9.966 | 10.001 | 10.001 | ms |
| agentflow | history | step_030 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_030 | rg | wall | 2 | 5.207 | 5.207 | 5.447 | 5.447 | ms |
| agentflow | history | step_031 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_031 | current | index_size | 2 | 6320412.000 | 6320412.000 | 6320412.000 | 6320412.000 | bytes |
| agentflow | history | step_031 | current | peak_rss | 2 | 11550720.000 | 11550720.000 | 11567104.000 | 11567104.000 | bytes |
| agentflow | history | step_031 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_031 | current | searchable_bytes | 2 | 1759607.000 | 1759607.000 | 1759607.000 | 1759607.000 | bytes |
| agentflow | history | step_031 | current | searchable_files | 2 | 137.000 | 137.000 | 137.000 | 137.000 | count |
| agentflow | history | step_031 | current | segments | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| agentflow | history | step_031 | current | wall | 2 | 7.306 | 7.306 | 7.387 | 7.387 | ms |
| agentflow | history | step_031 | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_031 | rg | wall | 2 | 5.664 | 5.664 | 5.692 | 5.692 | ms |
| agentflow | history | step_032 | current | extracted | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_032 | current | index_size | 2 | 6803725.000 | 6803725.000 | 6803725.000 | 6803725.000 | bytes |
| agentflow | history | step_032 | current | peak_rss | 2 | 15171584.000 | 15171584.000 | 15187968.000 | 15187968.000 | bytes |
| agentflow | history | step_032 | current | reused | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_032 | current | searchable_bytes | 2 | 1784484.000 | 1784484.000 | 1784484.000 | 1784484.000 | bytes |
| agentflow | history | step_032 | current | searchable_files | 2 | 140.000 | 140.000 | 140.000 | 140.000 | count |
| agentflow | history | step_032 | current | segments | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| agentflow | history | step_032 | current | wall | 2 | 11.288 | 11.288 | 11.775 | 11.775 | ms |
| agentflow | history | step_032 | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_032 | rg | wall | 2 | 6.816 | 6.816 | 7.980 | 7.980 | ms |
| agentflow | history | step_033 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_033 | current | index_size | 2 | 6960709.000 | 6960709.000 | 6960709.000 | 6960709.000 | bytes |
| agentflow | history | step_033 | current | peak_rss | 2 | 13017088.000 | 13017088.000 | 13025280.000 | 13025280.000 | bytes |
| agentflow | history | step_033 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_033 | current | searchable_bytes | 2 | 1785939.000 | 1785939.000 | 1785939.000 | 1785939.000 | bytes |
| agentflow | history | step_033 | current | searchable_files | 2 | 140.000 | 140.000 | 140.000 | 140.000 | count |
| agentflow | history | step_033 | current | segments | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| agentflow | history | step_033 | current | wall | 2 | 8.130 | 8.130 | 8.460 | 8.460 | ms |
| agentflow | history | step_033 | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_033 | rg | wall | 2 | 6.114 | 6.114 | 6.631 | 6.631 | ms |
| agentflow | history | step_034 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_034 | current | index_size | 2 | 6837765.000 | 6837765.000 | 6837765.000 | 6837765.000 | bytes |
| agentflow | history | step_034 | current | peak_rss | 2 | 11624448.000 | 11624448.000 | 11665408.000 | 11665408.000 | bytes |
| agentflow | history | step_034 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_034 | current | searchable_bytes | 2 | 1790291.000 | 1790291.000 | 1790291.000 | 1790291.000 | bytes |
| agentflow | history | step_034 | current | searchable_files | 2 | 141.000 | 141.000 | 141.000 | 141.000 | count |
| agentflow | history | step_034 | current | segments | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| agentflow | history | step_034 | current | wall | 2 | 7.541 | 7.541 | 7.963 | 7.963 | ms |
| agentflow | history | step_034 | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | step_034 | rg | wall | 2 | 7.440 | 7.440 | 8.417 | 8.417 | ms |
| agentflow | history | step_035 | current | extracted | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_035 | current | index_size | 2 | 7208320.000 | 7208320.000 | 7208320.000 | 7208320.000 | bytes |
| agentflow | history | step_035 | current | peak_rss | 2 | 14696448.000 | 14696448.000 | 14745600.000 | 14745600.000 | bytes |
| agentflow | history | step_035 | current | reused | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_035 | current | searchable_bytes | 2 | 1820206.000 | 1820206.000 | 1820206.000 | 1820206.000 | bytes |
| agentflow | history | step_035 | current | searchable_files | 2 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| agentflow | history | step_035 | current | segments | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| agentflow | history | step_035 | current | wall | 2 | 10.115 | 10.115 | 10.300 | 10.300 | ms |
| agentflow | history | step_035 | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | history | step_035 | rg | wall | 2 | 6.580 | 6.580 | 7.010 | 7.010 | ms |
| agentflow | history | step_036 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| agentflow | history | step_036 | current | index_size | 2 | 7410863.000 | 7410863.000 | 7410863.000 | 7410863.000 | bytes |
| agentflow | history | step_036 | current | peak_rss | 2 | 13484032.000 | 13484032.000 | 13484032.000 | 13484032.000 | bytes |
| agentflow | history | step_036 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| agentflow | history | step_036 | current | searchable_bytes | 2 | 1826543.000 | 1826543.000 | 1826543.000 | 1826543.000 | bytes |
| agentflow | history | step_036 | current | searchable_files | 2 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| agentflow | history | step_036 | current | segments | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| agentflow | history | step_036 | current | wall | 2 | 8.605 | 8.605 | 9.053 | 9.053 | ms |
| agentflow | history | step_036 | rg | peak_rss | 2 | 6479872.000 | 6479872.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_036 | rg | wall | 2 | 5.722 | 5.722 | 5.765 | 5.765 | ms |
| agentflow | history | step_037 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_037 | current | index_size | 2 | 7575286.000 | 7575286.000 | 7575286.000 | 7575286.000 | bytes |
| agentflow | history | step_037 | current | peak_rss | 2 | 13598720.000 | 13598720.000 | 13647872.000 | 13647872.000 | bytes |
| agentflow | history | step_037 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_037 | current | searchable_bytes | 2 | 1830199.000 | 1830199.000 | 1830199.000 | 1830199.000 | bytes |
| agentflow | history | step_037 | current | searchable_files | 2 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| agentflow | history | step_037 | current | segments | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| agentflow | history | step_037 | current | wall | 2 | 8.498 | 8.498 | 8.609 | 8.609 | ms |
| agentflow | history | step_037 | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | history | step_037 | rg | wall | 2 | 6.577 | 6.577 | 7.313 | 7.313 | ms |
| agentflow | history | step_038 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_038 | current | index_size | 2 | 7604797.000 | 7604797.000 | 7604797.000 | 7604797.000 | bytes |
| agentflow | history | step_038 | current | peak_rss | 2 | 11902976.000 | 11902976.000 | 11927552.000 | 11927552.000 | bytes |
| agentflow | history | step_038 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_038 | current | searchable_bytes | 2 | 1834353.000 | 1834353.000 | 1834353.000 | 1834353.000 | bytes |
| agentflow | history | step_038 | current | searchable_files | 2 | 146.000 | 146.000 | 146.000 | 146.000 | count |
| agentflow | history | step_038 | current | segments | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_038 | current | wall | 2 | 7.875 | 7.875 | 8.886 | 8.886 | ms |
| agentflow | history | step_038 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_038 | rg | wall | 2 | 7.322 | 7.322 | 7.473 | 7.473 | ms |
| agentflow | history | step_039 | current | extracted | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| agentflow | history | step_039 | current | index_size | 2 | 7956382.000 | 7956382.000 | 7956382.000 | 7956382.000 | bytes |
| agentflow | history | step_039 | current | peak_rss | 2 | 15032320.000 | 15032320.000 | 15089664.000 | 15089664.000 | bytes |
| agentflow | history | step_039 | current | reused | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| agentflow | history | step_039 | current | searchable_bytes | 2 | 1858874.000 | 1858874.000 | 1858874.000 | 1858874.000 | bytes |
| agentflow | history | step_039 | current | searchable_files | 2 | 149.000 | 149.000 | 149.000 | 149.000 | count |
| agentflow | history | step_039 | current | segments | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| agentflow | history | step_039 | current | wall | 2 | 10.183 | 10.183 | 10.444 | 10.444 | ms |
| agentflow | history | step_039 | rg | peak_rss | 2 | 6496256.000 | 6496256.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | step_039 | rg | wall | 2 | 7.457 | 7.457 | 8.203 | 8.203 | ms |
| agentflow | history | step_040 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_040 | current | index_size | 2 | 7989512.000 | 7989512.000 | 7989512.000 | 7989512.000 | bytes |
| agentflow | history | step_040 | current | peak_rss | 2 | 11902976.000 | 11902976.000 | 11927552.000 | 11927552.000 | bytes |
| agentflow | history | step_040 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_040 | current | searchable_bytes | 2 | 1863493.000 | 1863493.000 | 1863493.000 | 1863493.000 | bytes |
| agentflow | history | step_040 | current | searchable_files | 2 | 150.000 | 150.000 | 150.000 | 150.000 | count |
| agentflow | history | step_040 | current | segments | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| agentflow | history | step_040 | current | wall | 2 | 7.242 | 7.242 | 7.298 | 7.298 | ms |
| agentflow | history | step_040 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | history | step_040 | rg | wall | 2 | 7.525 | 7.525 | 7.808 | 7.808 | ms |
| agentflow | history | step_041 | current | extracted | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_041 | current | index_size | 2 | 8559636.000 | 8559636.000 | 8559636.000 | 8559636.000 | bytes |
| agentflow | history | step_041 | current | peak_rss | 2 | 16424960.000 | 16424960.000 | 17039360.000 | 17039360.000 | bytes |
| agentflow | history | step_041 | current | reused | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_041 | current | searchable_bytes | 2 | 1897230.000 | 1897230.000 | 1897230.000 | 1897230.000 | bytes |
| agentflow | history | step_041 | current | searchable_files | 2 | 152.000 | 152.000 | 152.000 | 152.000 | count |
| agentflow | history | step_041 | current | segments | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| agentflow | history | step_041 | current | wall | 2 | 12.572 | 12.572 | 12.609 | 12.609 | ms |
| agentflow | history | step_041 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_041 | rg | wall | 2 | 6.005 | 6.005 | 6.483 | 6.483 | ms |
| agentflow | history | step_042 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_042 | current | index_size | 2 | 8698619.000 | 8698619.000 | 8698619.000 | 8698619.000 | bytes |
| agentflow | history | step_042 | current | peak_rss | 2 | 13860864.000 | 13860864.000 | 13893632.000 | 13893632.000 | bytes |
| agentflow | history | step_042 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_042 | current | searchable_bytes | 2 | 1899686.000 | 1899686.000 | 1899686.000 | 1899686.000 | bytes |
| agentflow | history | step_042 | current | searchable_files | 2 | 152.000 | 152.000 | 152.000 | 152.000 | count |
| agentflow | history | step_042 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| agentflow | history | step_042 | current | wall | 2 | 8.079 | 8.079 | 8.206 | 8.206 | ms |
| agentflow | history | step_042 | rg | peak_rss | 2 | 6471680.000 | 6471680.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | history | step_042 | rg | wall | 2 | 7.365 | 7.365 | 7.553 | 7.553 | ms |
| agentflow | history | step_043 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_043 | current | index_size | 2 | 8731079.000 | 8731079.000 | 8731079.000 | 8731079.000 | bytes |
| agentflow | history | step_043 | current | peak_rss | 2 | 12132352.000 | 12132352.000 | 12173312.000 | 12173312.000 | bytes |
| agentflow | history | step_043 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_043 | current | searchable_bytes | 2 | 1904274.000 | 1904274.000 | 1904274.000 | 1904274.000 | bytes |
| agentflow | history | step_043 | current | searchable_files | 2 | 153.000 | 153.000 | 153.000 | 153.000 | count |
| agentflow | history | step_043 | current | segments | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| agentflow | history | step_043 | current | wall | 2 | 8.796 | 8.796 | 10.387 | 10.387 | ms |
| agentflow | history | step_043 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_043 | rg | wall | 2 | 6.754 | 6.754 | 7.089 | 7.089 | ms |
| agentflow | history | step_044 | current | extracted | 2 | 26.000 | 26.000 | 26.000 | 26.000 | count |
| agentflow | history | step_044 | current | index_size | 2 | 9370912.000 | 9370912.000 | 9370912.000 | 9370912.000 | bytes |
| agentflow | history | step_044 | current | peak_rss | 2 | 17440768.000 | 17440768.000 | 17891328.000 | 17891328.000 | bytes |
| agentflow | history | step_044 | current | reused | 2 | 26.000 | 26.000 | 26.000 | 26.000 | count |
| agentflow | history | step_044 | current | searchable_bytes | 2 | 1935108.000 | 1935108.000 | 1935108.000 | 1935108.000 | bytes |
| agentflow | history | step_044 | current | searchable_files | 2 | 156.000 | 156.000 | 156.000 | 156.000 | count |
| agentflow | history | step_044 | current | segments | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| agentflow | history | step_044 | current | wall | 2 | 14.001 | 14.001 | 14.854 | 14.854 | ms |
| agentflow | history | step_044 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | history | step_044 | rg | wall | 2 | 7.329 | 7.329 | 9.075 | 9.075 | ms |
| agentflow | history | step_045 | current | extracted | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| agentflow | history | step_045 | current | index_size | 2 | 9688546.000 | 9688546.000 | 9688546.000 | 9688546.000 | bytes |
| agentflow | history | step_045 | current | peak_rss | 2 | 14729216.000 | 14729216.000 | 14729216.000 | 14729216.000 | bytes |
| agentflow | history | step_045 | current | reused | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| agentflow | history | step_045 | current | searchable_bytes | 2 | 1938271.000 | 1938271.000 | 1938271.000 | 1938271.000 | bytes |
| agentflow | history | step_045 | current | searchable_files | 2 | 157.000 | 157.000 | 157.000 | 157.000 | count |
| agentflow | history | step_045 | current | segments | 2 | 19.000 | 19.000 | 19.000 | 19.000 | count |
| agentflow | history | step_045 | current | wall | 2 | 9.532 | 9.532 | 9.649 | 9.649 | ms |
| agentflow | history | step_045 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_045 | rg | wall | 2 | 6.929 | 6.929 | 8.319 | 8.319 | ms |
| agentflow | history | step_046 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_046 | current | index_size | 2 | 9693277.000 | 9693277.000 | 9693277.000 | 9693277.000 | bytes |
| agentflow | history | step_046 | current | peak_rss | 2 | 12148736.000 | 12148736.000 | 12156928.000 | 12156928.000 | bytes |
| agentflow | history | step_046 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_046 | current | searchable_bytes | 2 | 1942914.000 | 1942914.000 | 1942914.000 | 1942914.000 | bytes |
| agentflow | history | step_046 | current | searchable_files | 2 | 158.000 | 158.000 | 158.000 | 158.000 | count |
| agentflow | history | step_046 | current | segments | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| agentflow | history | step_046 | current | wall | 2 | 7.204 | 7.204 | 7.294 | 7.294 | ms |
| agentflow | history | step_046 | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_046 | rg | wall | 2 | 9.246 | 9.246 | 9.676 | 9.676 | ms |
| agentflow | history | step_047 | current | extracted | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| agentflow | history | step_047 | current | index_size | 2 | 10036673.000 | 10036673.000 | 10036673.000 | 10036673.000 | bytes |
| agentflow | history | step_047 | current | peak_rss | 2 | 14950400.000 | 14950400.000 | 15089664.000 | 15089664.000 | bytes |
| agentflow | history | step_047 | current | reused | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| agentflow | history | step_047 | current | searchable_bytes | 2 | 1969439.000 | 1969439.000 | 1969439.000 | 1969439.000 | bytes |
| agentflow | history | step_047 | current | searchable_files | 2 | 161.000 | 161.000 | 161.000 | 161.000 | count |
| agentflow | history | step_047 | current | segments | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| agentflow | history | step_047 | current | wall | 2 | 10.339 | 10.339 | 10.806 | 10.806 | ms |
| agentflow | history | step_047 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | history | step_047 | rg | wall | 2 | 6.316 | 6.316 | 7.189 | 7.189 | ms |
| agentflow | history | step_048 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_048 | current | index_size | 2 | 10157098.000 | 10157098.000 | 10157098.000 | 10157098.000 | bytes |
| agentflow | history | step_048 | current | peak_rss | 2 | 13148160.000 | 13148160.000 | 13156352.000 | 13156352.000 | bytes |
| agentflow | history | step_048 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_048 | current | searchable_bytes | 2 | 1970833.000 | 1970833.000 | 1970833.000 | 1970833.000 | bytes |
| agentflow | history | step_048 | current | searchable_files | 2 | 161.000 | 161.000 | 161.000 | 161.000 | count |
| agentflow | history | step_048 | current | segments | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| agentflow | history | step_048 | current | wall | 2 | 9.747 | 9.747 | 10.783 | 10.783 | ms |
| agentflow | history | step_048 | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_048 | rg | wall | 2 | 7.898 | 7.898 | 8.043 | 8.043 | ms |
| agentflow | history | step_049 | current | extracted | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| agentflow | history | step_049 | current | index_size | 2 | 10628335.000 | 10628335.000 | 10628335.000 | 10628335.000 | bytes |
| agentflow | history | step_049 | current | peak_rss | 2 | 15720448.000 | 15720448.000 | 15810560.000 | 15810560.000 | bytes |
| agentflow | history | step_049 | current | reused | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| agentflow | history | step_049 | current | searchable_bytes | 2 | 2025456.000 | 2025456.000 | 2025456.000 | 2025456.000 | bytes |
| agentflow | history | step_049 | current | searchable_files | 2 | 165.000 | 165.000 | 165.000 | 165.000 | count |
| agentflow | history | step_049 | current | segments | 2 | 22.000 | 22.000 | 22.000 | 22.000 | count |
| agentflow | history | step_049 | current | wall | 2 | 11.304 | 11.304 | 11.563 | 11.563 | ms |
| agentflow | history | step_049 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | history | step_049 | rg | wall | 2 | 5.681 | 5.681 | 5.767 | 5.767 | ms |
| agentflow | history | step_050 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| agentflow | history | step_050 | current | index_size | 2 | 13494296.000 | 13494296.000 | 13494296.000 | 13494296.000 | bytes |
| agentflow | history | step_050 | current | peak_rss | 2 | 24903680.000 | 24903680.000 | 25034752.000 | 25034752.000 | bytes |
| agentflow | history | step_050 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| agentflow | history | step_050 | current | searchable_bytes | 2 | 2039247.000 | 2039247.000 | 2039247.000 | 2039247.000 | bytes |
| agentflow | history | step_050 | current | searchable_files | 2 | 167.000 | 167.000 | 167.000 | 167.000 | count |
| agentflow | history | step_050 | current | segments | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_050 | current | wall | 2 | 91.826 | 91.826 | 94.161 | 94.161 | ms |
| agentflow | history | step_050 | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_050 | rg | wall | 2 | 9.330 | 9.330 | 9.741 | 9.741 | ms |
| agentflow | history | step_051 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_051 | current | index_size | 2 | 13623861.000 | 13623861.000 | 13623861.000 | 13623861.000 | bytes |
| agentflow | history | step_051 | current | peak_rss | 2 | 12730368.000 | 12730368.000 | 12730368.000 | 12730368.000 | bytes |
| agentflow | history | step_051 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_051 | current | searchable_bytes | 2 | 2057149.000 | 2057149.000 | 2057149.000 | 2057149.000 | bytes |
| agentflow | history | step_051 | current | searchable_files | 2 | 169.000 | 169.000 | 169.000 | 169.000 | count |
| agentflow | history | step_051 | current | segments | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | history | step_051 | current | wall | 2 | 7.735 | 7.735 | 7.844 | 7.844 | ms |
| agentflow | history | step_051 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_051 | rg | wall | 2 | 7.271 | 7.271 | 7.351 | 7.351 | ms |
| agentflow | history | step_052 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_052 | current | index_size | 2 | 13658543.000 | 13658543.000 | 13658543.000 | 13658543.000 | bytes |
| agentflow | history | step_052 | current | peak_rss | 2 | 11255808.000 | 11255808.000 | 11288576.000 | 11288576.000 | bytes |
| agentflow | history | step_052 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_052 | current | searchable_bytes | 2 | 2062861.000 | 2062861.000 | 2062861.000 | 2062861.000 | bytes |
| agentflow | history | step_052 | current | searchable_files | 2 | 170.000 | 170.000 | 170.000 | 170.000 | count |
| agentflow | history | step_052 | current | segments | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | history | step_052 | current | wall | 2 | 7.027 | 7.027 | 7.244 | 7.244 | ms |
| agentflow | history | step_052 | rg | peak_rss | 2 | 6488064.000 | 6488064.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | history | step_052 | rg | wall | 2 | 6.747 | 6.747 | 7.821 | 7.821 | ms |
| agentflow | history | step_053 | current | extracted | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| agentflow | history | step_053 | current | index_size | 2 | 14017117.000 | 14017117.000 | 14017117.000 | 14017117.000 | bytes |
| agentflow | history | step_053 | current | peak_rss | 2 | 13893632.000 | 13893632.000 | 14057472.000 | 14057472.000 | bytes |
| agentflow | history | step_053 | current | reused | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| agentflow | history | step_053 | current | searchable_bytes | 2 | 2091885.000 | 2091885.000 | 2091885.000 | 2091885.000 | bytes |
| agentflow | history | step_053 | current | searchable_files | 2 | 174.000 | 174.000 | 174.000 | 174.000 | count |
| agentflow | history | step_053 | current | segments | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | history | step_053 | current | wall | 2 | 10.271 | 10.271 | 10.410 | 10.410 | ms |
| agentflow | history | step_053 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | history | step_053 | rg | wall | 2 | 7.109 | 7.109 | 7.221 | 7.221 | ms |
| agentflow | history | step_054 | current | extracted | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | step_054 | current | index_size | 2 | 14149004.000 | 14149004.000 | 14149004.000 | 14149004.000 | bytes |
| agentflow | history | step_054 | current | peak_rss | 2 | 12517376.000 | 12517376.000 | 12566528.000 | 12566528.000 | bytes |
| agentflow | history | step_054 | current | reused | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | step_054 | current | searchable_bytes | 2 | 2096067.000 | 2096067.000 | 2096067.000 | 2096067.000 | bytes |
| agentflow | history | step_054 | current | searchable_files | 2 | 174.000 | 174.000 | 174.000 | 174.000 | count |
| agentflow | history | step_054 | current | segments | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| agentflow | history | step_054 | current | wall | 2 | 8.706 | 8.706 | 9.747 | 9.747 | ms |
| agentflow | history | step_054 | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | history | step_054 | rg | wall | 2 | 6.663 | 6.663 | 7.550 | 7.550 | ms |
| agentflow | history | step_055 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_055 | current | index_size | 2 | 14183765.000 | 14183765.000 | 14183765.000 | 14183765.000 | bytes |
| agentflow | history | step_055 | current | peak_rss | 2 | 11395072.000 | 11395072.000 | 11403264.000 | 11403264.000 | bytes |
| agentflow | history | step_055 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | history | step_055 | current | searchable_bytes | 2 | 2101667.000 | 2101667.000 | 2101667.000 | 2101667.000 | bytes |
| agentflow | history | step_055 | current | searchable_files | 2 | 175.000 | 175.000 | 175.000 | 175.000 | count |
| agentflow | history | step_055 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | step_055 | current | wall | 2 | 8.677 | 8.677 | 8.860 | 8.860 | ms |
| agentflow | history | step_055 | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6586368.000 | 6586368.000 | bytes |
| agentflow | history | step_055 | rg | wall | 2 | 10.011 | 10.011 | 11.675 | 11.675 | ms |
| agentflow | history | step_056 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| agentflow | history | step_056 | current | index_size | 2 | 14365725.000 | 14365725.000 | 14365725.000 | 14365725.000 | bytes |
| agentflow | history | step_056 | current | peak_rss | 2 | 12722176.000 | 12722176.000 | 12730368.000 | 12730368.000 | bytes |
| agentflow | history | step_056 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| agentflow | history | step_056 | current | searchable_bytes | 2 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | history | step_056 | current | searchable_files | 2 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | history | step_056 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| agentflow | history | step_056 | current | wall | 2 | 8.107 | 8.107 | 8.162 | 8.162 | ms |
| agentflow | history | step_056 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6586368.000 | 6586368.000 | bytes |
| agentflow | history | step_056 | rg | wall | 2 | 8.443 | 8.443 | 9.783 | 9.783 | ms |
| agentflow | manifest | search_view | current | load | 16 | 0.016 | 0.018 | 0.030 | 0.030 | ms |
| agentflow | search | history_0-absent | current | peak_rss | 4 | 9576448.000 | 9572352.000 | 9601024.000 | 9601024.000 | bytes |
| agentflow | search | history_0-absent | current | wall | 32 | 3.797 | 3.961 | 4.931 | 5.384 | ms |
| agentflow | search | history_0-absent | rg | peak_rss | 4 | 6430720.000 | 6430720.000 | 6455296.000 | 6455296.000 | bytes |
| agentflow | search | history_0-absent | rg | wall | 32 | 4.655 | 4.494 | 5.901 | 5.944 | ms |
| agentflow | search | history_0-anchor | current | peak_rss | 4 | 11239424.000 | 11231232.000 | 11272192.000 | 11272192.000 | bytes |
| agentflow | search | history_0-anchor | current | wall | 32 | 4.563 | 4.708 | 5.459 | 6.513 | ms |
| agentflow | search | history_0-anchor | rg | peak_rss | 4 | 7577600.000 | 7573504.000 | 7913472.000 | 7913472.000 | bytes |
| agentflow | search | history_0-anchor | rg | wall | 32 | 6.843 | 6.686 | 7.419 | 7.491 | ms |
| agentflow | search | history_0-blank | current | peak_rss | 4 | 10305536.000 | 10301440.000 | 10338304.000 | 10338304.000 | bytes |
| agentflow | search | history_0-blank | current | wall | 32 | 4.726 | 4.832 | 5.899 | 6.253 | ms |
| agentflow | search | history_0-blank | rg | peak_rss | 4 | 6643712.000 | 6639616.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | history_0-blank | rg | wall | 32 | 4.696 | 4.835 | 5.977 | 6.920 | ms |
| agentflow | search | history_0-broad | current | peak_rss | 4 | 9846784.000 | 9834496.000 | 9863168.000 | 9863168.000 | bytes |
| agentflow | search | history_0-broad | current | wall | 32 | 3.917 | 4.102 | 5.106 | 5.834 | ms |
| agentflow | search | history_0-broad | rg | peak_rss | 4 | 6438912.000 | 6438912.000 | 6455296.000 | 6455296.000 | bytes |
| agentflow | search | history_0-broad | rg | wall | 32 | 4.600 | 4.447 | 6.601 | 8.244 | ms |
| agentflow | search | history_0-count | current | peak_rss | 4 | 10395648.000 | 10387456.000 | 10502144.000 | 10502144.000 | bytes |
| agentflow | search | history_0-count | current | wall | 32 | 4.880 | 4.948 | 5.786 | 6.268 | ms |
| agentflow | search | history_0-count | rg | peak_rss | 4 | 6938624.000 | 6930432.000 | 7012352.000 | 7012352.000 | bytes |
| agentflow | search | history_0-count | rg | wall | 32 | 5.361 | 5.575 | 6.729 | 6.877 | ms |
| agentflow | search | history_0-icase | current | peak_rss | 4 | 10043392.000 | 10039296.000 | 10092544.000 | 10092544.000 | bytes |
| agentflow | search | history_0-icase | current | wall | 32 | 4.095 | 4.300 | 5.674 | 5.904 | ms |
| agentflow | search | history_0-icase | rg | peak_rss | 4 | 6840320.000 | 6873088.000 | 6995968.000 | 6995968.000 | bytes |
| agentflow | search | history_0-icase | rg | wall | 32 | 4.879 | 4.842 | 5.597 | 5.798 | ms |
| agentflow | search | history_0-icase_literal | current | peak_rss | 4 | 9748480.000 | 9764864.000 | 9846784.000 | 9846784.000 | bytes |
| agentflow | search | history_0-icase_literal | current | wall | 32 | 4.024 | 4.207 | 5.408 | 5.540 | ms |
| agentflow | search | history_0-icase_literal | rg | peak_rss | 4 | 6512640.000 | 6524928.000 | 6635520.000 | 6635520.000 | bytes |
| agentflow | search | history_0-icase_literal | rg | wall | 32 | 4.669 | 4.572 | 6.377 | 6.537 | ms |
| agentflow | search | history_0-literal | current | peak_rss | 4 | 9707520.000 | 9711616.000 | 9748480.000 | 9748480.000 | bytes |
| agentflow | search | history_0-literal | current | wall | 32 | 3.843 | 3.927 | 4.450 | 4.807 | ms |
| agentflow | search | history_0-literal | rg | peak_rss | 4 | 6455296.000 | 6443008.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | search | history_0-literal | rg | wall | 32 | 4.541 | 4.415 | 6.606 | 6.761 | ms |
| agentflow | search | history_0-literal_lines | current | peak_rss | 4 | 9756672.000 | 9773056.000 | 9830400.000 | 9830400.000 | bytes |
| agentflow | search | history_0-literal_lines | current | wall | 32 | 4.037 | 4.142 | 5.121 | 5.192 | ms |
| agentflow | search | history_0-literal_lines | rg | peak_rss | 4 | 6529024.000 | 6533120.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | search | history_0-literal_lines | rg | wall | 32 | 4.640 | 4.442 | 5.394 | 5.405 | ms |
| agentflow | search | history_0-or | current | peak_rss | 4 | 10043392.000 | 10051584.000 | 10092544.000 | 10092544.000 | bytes |
| agentflow | search | history_0-or | current | wall | 32 | 4.046 | 4.232 | 5.670 | 5.892 | ms |
| agentflow | search | history_0-or | rg | peak_rss | 4 | 6725632.000 | 6721536.000 | 6750208.000 | 6750208.000 | bytes |
| agentflow | search | history_0-or | rg | wall | 32 | 4.883 | 4.703 | 5.760 | 6.136 | ms |
| agentflow | search | history_0-short | current | peak_rss | 4 | 10100736.000 | 10100736.000 | 10174464.000 | 10174464.000 | bytes |
| agentflow | search | history_0-short | current | wall | 32 | 4.087 | 4.130 | 4.604 | 5.264 | ms |
| agentflow | search | history_0-short | rg | peak_rss | 4 | 6488064.000 | 6471680.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | search | history_0-short | rg | wall | 32 | 4.686 | 4.518 | 5.238 | 5.301 | ms |
| agentflow | search | history_0-word | current | peak_rss | 4 | 9871360.000 | 9871360.000 | 9912320.000 | 9912320.000 | bytes |
| agentflow | search | history_0-word | current | wall | 32 | 4.023 | 4.180 | 5.695 | 5.786 | ms |
| agentflow | search | history_0-word | rg | peak_rss | 4 | 6660096.000 | 6647808.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | history_0-word | rg | wall | 32 | 4.670 | 4.415 | 4.957 | 5.369 | ms |
| agentflow | search | history_14-absent | current | peak_rss | 4 | 10264576.000 | 10276864.000 | 10321920.000 | 10321920.000 | bytes |
| agentflow | search | history_14-absent | current | wall | 32 | 4.322 | 4.395 | 5.004 | 5.152 | ms |
| agentflow | search | history_14-absent | rg | peak_rss | 4 | 6488064.000 | 6496256.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | search | history_14-absent | rg | wall | 32 | 5.232 | 5.469 | 6.282 | 6.408 | ms |
| agentflow | search | history_14-anchor | current | peak_rss | 4 | 16195584.000 | 16207872.000 | 16269312.000 | 16269312.000 | bytes |
| agentflow | search | history_14-anchor | current | wall | 32 | 6.508 | 6.608 | 7.399 | 7.494 | ms |
| agentflow | search | history_14-anchor | rg | peak_rss | 4 | 7929856.000 | 7946240.000 | 8077312.000 | 8077312.000 | bytes |
| agentflow | search | history_14-anchor | rg | wall | 32 | 6.985 | 7.324 | 8.736 | 8.892 | ms |
| agentflow | search | history_14-blank | current | peak_rss | 4 | 11239424.000 | 11239424.000 | 11272192.000 | 11272192.000 | bytes |
| agentflow | search | history_14-blank | current | wall | 32 | 6.600 | 6.746 | 7.555 | 7.851 | ms |
| agentflow | search | history_14-blank | rg | peak_rss | 4 | 6864896.000 | 6844416.000 | 6914048.000 | 6914048.000 | bytes |
| agentflow | search | history_14-blank | rg | wall | 32 | 6.025 | 6.298 | 7.207 | 7.229 | ms |
| agentflow | search | history_14-broad | current | peak_rss | 4 | 11812864.000 | 11816960.000 | 11862016.000 | 11862016.000 | bytes |
| agentflow | search | history_14-broad | current | wall | 32 | 5.182 | 5.227 | 5.928 | 6.010 | ms |
| agentflow | search | history_14-broad | rg | peak_rss | 4 | 6602752.000 | 6606848.000 | 6651904.000 | 6651904.000 | bytes |
| agentflow | search | history_14-broad | rg | wall | 32 | 5.617 | 5.738 | 7.011 | 7.204 | ms |
| agentflow | search | history_14-count | current | peak_rss | 4 | 11042816.000 | 11042816.000 | 11075584.000 | 11075584.000 | bytes |
| agentflow | search | history_14-count | current | wall | 32 | 5.695 | 5.707 | 6.337 | 6.692 | ms |
| agentflow | search | history_14-count | rg | peak_rss | 4 | 7102464.000 | 7094272.000 | 7127040.000 | 7127040.000 | bytes |
| agentflow | search | history_14-count | rg | wall | 32 | 5.863 | 6.069 | 7.128 | 7.187 | ms |
| agentflow | search | history_14-icase | current | peak_rss | 4 | 12951552.000 | 12939264.000 | 12976128.000 | 12976128.000 | bytes |
| agentflow | search | history_14-icase | current | wall | 32 | 4.896 | 4.991 | 5.717 | 6.013 | ms |
| agentflow | search | history_14-icase | rg | peak_rss | 4 | 6848512.000 | 6864896.000 | 6914048.000 | 6914048.000 | bytes |
| agentflow | search | history_14-icase | rg | wall | 32 | 6.252 | 6.025 | 6.846 | 7.056 | ms |
| agentflow | search | history_14-icase_literal | current | peak_rss | 4 | 11878400.000 | 11902976.000 | 12009472.000 | 12009472.000 | bytes |
| agentflow | search | history_14-icase_literal | current | wall | 32 | 4.645 | 4.732 | 5.257 | 5.364 | ms |
| agentflow | search | history_14-icase_literal | rg | peak_rss | 4 | 6660096.000 | 6639616.000 | 6668288.000 | 6668288.000 | bytes |
| agentflow | search | history_14-icase_literal | rg | wall | 32 | 6.014 | 5.727 | 6.429 | 6.432 | ms |
| agentflow | search | history_14-literal | current | peak_rss | 4 | 11485184.000 | 11485184.000 | 11517952.000 | 11517952.000 | bytes |
| agentflow | search | history_14-literal | current | wall | 32 | 4.700 | 4.801 | 5.460 | 5.821 | ms |
| agentflow | search | history_14-literal | rg | peak_rss | 4 | 6569984.000 | 6574080.000 | 6635520.000 | 6635520.000 | bytes |
| agentflow | search | history_14-literal | rg | wall | 32 | 5.312 | 5.450 | 6.384 | 6.394 | ms |
| agentflow | search | history_14-literal_lines | current | peak_rss | 4 | 11337728.000 | 11341824.000 | 11386880.000 | 11386880.000 | bytes |
| agentflow | search | history_14-literal_lines | current | wall | 32 | 5.076 | 5.212 | 6.060 | 6.321 | ms |
| agentflow | search | history_14-literal_lines | rg | peak_rss | 4 | 6692864.000 | 6696960.000 | 6717440.000 | 6717440.000 | bytes |
| agentflow | search | history_14-literal_lines | rg | wall | 32 | 5.360 | 5.573 | 6.483 | 6.662 | ms |
| agentflow | search | history_14-or | current | peak_rss | 4 | 12754944.000 | 12754944.000 | 12812288.000 | 12812288.000 | bytes |
| agentflow | search | history_14-or | current | wall | 32 | 5.289 | 5.345 | 5.930 | 6.402 | ms |
| agentflow | search | history_14-or | rg | peak_rss | 4 | 6832128.000 | 6815744.000 | 6881280.000 | 6881280.000 | bytes |
| agentflow | search | history_14-or | rg | wall | 32 | 5.613 | 5.811 | 6.573 | 6.648 | ms |
| agentflow | search | history_14-short | current | peak_rss | 4 | 10821632.000 | 10817536.000 | 10862592.000 | 10862592.000 | bytes |
| agentflow | search | history_14-short | current | wall | 32 | 5.165 | 5.255 | 6.113 | 6.126 | ms |
| agentflow | search | history_14-short | rg | peak_rss | 4 | 6627328.000 | 6635520.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | history_14-short | rg | wall | 32 | 5.529 | 5.760 | 7.466 | 7.643 | ms |
| agentflow | search | history_14-word | current | peak_rss | 4 | 11436032.000 | 11444224.000 | 11485184.000 | 11485184.000 | bytes |
| agentflow | search | history_14-word | current | wall | 32 | 5.216 | 5.260 | 5.934 | 6.016 | ms |
| agentflow | search | history_14-word | rg | peak_rss | 4 | 6766592.000 | 6774784.000 | 6848512.000 | 6848512.000 | bytes |
| agentflow | search | history_14-word | rg | wall | 32 | 5.669 | 5.686 | 6.106 | 6.384 | ms |
| agentflow | search | history_28-absent | current | peak_rss | 4 | 9977856.000 | 9973760.000 | 10010624.000 | 10010624.000 | bytes |
| agentflow | search | history_28-absent | current | wall | 32 | 4.305 | 4.373 | 4.805 | 4.864 | ms |
| agentflow | search | history_28-absent | rg | peak_rss | 4 | 6537216.000 | 6537216.000 | 6586368.000 | 6586368.000 | bytes |
| agentflow | search | history_28-absent | rg | wall | 32 | 5.468 | 5.632 | 6.846 | 7.072 | ms |
| agentflow | search | history_28-anchor | current | peak_rss | 4 | 14745600.000 | 14737408.000 | 14794752.000 | 14794752.000 | bytes |
| agentflow | search | history_28-anchor | current | wall | 32 | 6.856 | 6.999 | 8.017 | 8.105 | ms |
| agentflow | search | history_28-anchor | rg | peak_rss | 4 | 7987200.000 | 7999488.000 | 8060928.000 | 8060928.000 | bytes |
| agentflow | search | history_28-anchor | rg | wall | 32 | 7.781 | 8.046 | 8.909 | 9.057 | ms |
| agentflow | search | history_28-blank | current | peak_rss | 4 | 11239424.000 | 11206656.000 | 11255808.000 | 11255808.000 | bytes |
| agentflow | search | history_28-blank | current | wall | 32 | 7.681 | 7.752 | 8.348 | 8.400 | ms |
| agentflow | search | history_28-blank | rg | peak_rss | 4 | 6889472.000 | 6893568.000 | 6930432.000 | 6930432.000 | bytes |
| agentflow | search | history_28-blank | rg | wall | 32 | 6.924 | 7.101 | 8.332 | 8.396 | ms |
| agentflow | search | history_28-broad | current | peak_rss | 4 | 11296768.000 | 11288576.000 | 11321344.000 | 11321344.000 | bytes |
| agentflow | search | history_28-broad | current | wall | 32 | 5.414 | 5.388 | 5.933 | 6.178 | ms |
| agentflow | search | history_28-broad | rg | peak_rss | 4 | 6660096.000 | 6660096.000 | 6717440.000 | 6717440.000 | bytes |
| agentflow | search | history_28-broad | rg | wall | 32 | 6.182 | 6.443 | 7.961 | 7.976 | ms |
| agentflow | search | history_28-count | current | peak_rss | 4 | 10960896.000 | 10944512.000 | 10960896.000 | 10960896.000 | bytes |
| agentflow | search | history_28-count | current | wall | 32 | 5.856 | 5.965 | 6.456 | 7.454 | ms |
| agentflow | search | history_28-count | rg | peak_rss | 4 | 7331840.000 | 7335936.000 | 7389184.000 | 7389184.000 | bytes |
| agentflow | search | history_28-count | rg | wall | 32 | 6.569 | 6.890 | 7.993 | 8.480 | ms |
| agentflow | search | history_28-icase | current | peak_rss | 4 | 11837440.000 | 11833344.000 | 11862016.000 | 11862016.000 | bytes |
| agentflow | search | history_28-icase | current | wall | 32 | 4.809 | 4.899 | 5.654 | 5.698 | ms |
| agentflow | search | history_28-icase | rg | peak_rss | 4 | 6873088.000 | 6881280.000 | 6963200.000 | 6963200.000 | bytes |
| agentflow | search | history_28-icase | rg | wall | 32 | 6.195 | 6.065 | 6.916 | 7.644 | ms |
| agentflow | search | history_28-icase_literal | current | peak_rss | 4 | 11034624.000 | 11038720.000 | 11075584.000 | 11075584.000 | bytes |
| agentflow | search | history_28-icase_literal | current | wall | 32 | 4.620 | 4.721 | 5.383 | 5.387 | ms |
| agentflow | search | history_28-icase_literal | rg | peak_rss | 4 | 6594560.000 | 6598656.000 | 6635520.000 | 6635520.000 | bytes |
| agentflow | search | history_28-icase_literal | rg | wall | 32 | 5.595 | 5.728 | 6.622 | 6.806 | ms |
| agentflow | search | history_28-literal | current | peak_rss | 4 | 11010048.000 | 10993664.000 | 11010048.000 | 11010048.000 | bytes |
| agentflow | search | history_28-literal | current | wall | 32 | 5.039 | 5.139 | 5.915 | 5.952 | ms |
| agentflow | search | history_28-literal | rg | peak_rss | 4 | 6635520.000 | 6619136.000 | 6651904.000 | 6651904.000 | bytes |
| agentflow | search | history_28-literal | rg | wall | 32 | 5.812 | 6.003 | 7.500 | 7.576 | ms |
| agentflow | search | history_28-literal_lines | current | peak_rss | 4 | 11091968.000 | 11063296.000 | 11108352.000 | 11108352.000 | bytes |
| agentflow | search | history_28-literal_lines | current | wall | 32 | 5.630 | 5.668 | 6.348 | 6.350 | ms |
| agentflow | search | history_28-literal_lines | rg | peak_rss | 4 | 6791168.000 | 6778880.000 | 6815744.000 | 6815744.000 | bytes |
| agentflow | search | history_28-literal_lines | rg | wall | 32 | 6.219 | 6.541 | 8.001 | 8.098 | ms |
| agentflow | search | history_28-or | current | peak_rss | 4 | 12001280.000 | 11997184.000 | 12075008.000 | 12075008.000 | bytes |
| agentflow | search | history_28-or | current | wall | 32 | 5.465 | 5.525 | 6.042 | 6.775 | ms |
| agentflow | search | history_28-or | rg | peak_rss | 4 | 6848512.000 | 6860800.000 | 6930432.000 | 6930432.000 | bytes |
| agentflow | search | history_28-or | rg | wall | 32 | 6.087 | 6.213 | 7.236 | 7.944 | ms |
| agentflow | search | history_28-short | current | peak_rss | 4 | 10674176.000 | 10661888.000 | 10698752.000 | 10698752.000 | bytes |
| agentflow | search | history_28-short | current | wall | 32 | 5.541 | 5.570 | 6.067 | 6.174 | ms |
| agentflow | search | history_28-short | rg | peak_rss | 4 | 6651904.000 | 6656000.000 | 6717440.000 | 6717440.000 | bytes |
| agentflow | search | history_28-short | rg | wall | 32 | 6.441 | 6.732 | 8.164 | 8.480 | ms |
| agentflow | search | history_28-word | current | peak_rss | 4 | 11206656.000 | 11190272.000 | 11206656.000 | 11206656.000 | bytes |
| agentflow | search | history_28-word | current | wall | 32 | 5.702 | 5.785 | 6.428 | 6.579 | ms |
| agentflow | search | history_28-word | rg | peak_rss | 4 | 6889472.000 | 6897664.000 | 6963200.000 | 6963200.000 | bytes |
| agentflow | search | history_28-word | rg | wall | 32 | 6.229 | 6.494 | 7.507 | 7.644 | ms |
| agentflow | search | history_42-absent | current | peak_rss | 4 | 10436608.000 | 10448896.000 | 10485760.000 | 10485760.000 | bytes |
| agentflow | search | history_42-absent | current | wall | 32 | 4.711 | 4.794 | 5.606 | 5.645 | ms |
| agentflow | search | history_42-absent | rg | peak_rss | 4 | 6520832.000 | 6533120.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | search | history_42-absent | rg | wall | 32 | 5.804 | 5.971 | 7.369 | 9.642 | ms |
| agentflow | search | history_42-anchor | current | peak_rss | 4 | 17391616.000 | 17395712.000 | 17432576.000 | 17432576.000 | bytes |
| agentflow | search | history_42-anchor | current | wall | 32 | 7.752 | 7.854 | 8.495 | 8.694 | ms |
| agentflow | search | history_42-anchor | rg | peak_rss | 4 | 8044544.000 | 8036352.000 | 8093696.000 | 8093696.000 | bytes |
| agentflow | search | history_42-anchor | rg | wall | 32 | 8.092 | 8.343 | 9.800 | 10.291 | ms |
| agentflow | search | history_42-blank | current | peak_rss | 4 | 11575296.000 | 11583488.000 | 11681792.000 | 11681792.000 | bytes |
| agentflow | search | history_42-blank | current | wall | 32 | 8.402 | 8.489 | 9.634 | 9.861 | ms |
| agentflow | search | history_42-blank | rg | peak_rss | 4 | 6905856.000 | 6901760.000 | 6930432.000 | 6930432.000 | bytes |
| agentflow | search | history_42-blank | rg | wall | 32 | 7.486 | 7.678 | 9.375 | 9.636 | ms |
| agentflow | search | history_42-broad | current | peak_rss | 4 | 12558336.000 | 12558336.000 | 12599296.000 | 12599296.000 | bytes |
| agentflow | search | history_42-broad | current | wall | 32 | 5.774 | 6.011 | 7.732 | 7.894 | ms |
| agentflow | search | history_42-broad | rg | peak_rss | 4 | 6619136.000 | 6631424.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | history_42-broad | rg | wall | 32 | 6.701 | 6.904 | 8.514 | 8.938 | ms |
| agentflow | search | history_42-count | current | peak_rss | 4 | 11272192.000 | 11272192.000 | 11288576.000 | 11288576.000 | bytes |
| agentflow | search | history_42-count | current | wall | 32 | 6.547 | 6.618 | 7.476 | 8.404 | ms |
| agentflow | search | history_42-count | rg | peak_rss | 4 | 7413760.000 | 7376896.000 | 7536640.000 | 7536640.000 | bytes |
| agentflow | search | history_42-count | rg | wall | 32 | 7.255 | 7.354 | 8.833 | 8.992 | ms |
| agentflow | search | history_42-icase | current | peak_rss | 4 | 13606912.000 | 13615104.000 | 13647872.000 | 13647872.000 | bytes |
| agentflow | search | history_42-icase | current | wall | 32 | 5.327 | 5.375 | 5.945 | 6.264 | ms |
| agentflow | search | history_42-icase | rg | peak_rss | 4 | 6889472.000 | 6897664.000 | 6946816.000 | 6946816.000 | bytes |
| agentflow | search | history_42-icase | rg | wall | 32 | 6.289 | 6.249 | 7.087 | 7.155 | ms |
| agentflow | search | history_42-icase_literal | current | peak_rss | 4 | 12566528.000 | 12574720.000 | 12632064.000 | 12632064.000 | bytes |
| agentflow | search | history_42-icase_literal | current | wall | 32 | 5.065 | 5.105 | 5.866 | 5.974 | ms |
| agentflow | search | history_42-icase_literal | rg | peak_rss | 4 | 6586368.000 | 6590464.000 | 6635520.000 | 6635520.000 | bytes |
| agentflow | search | history_42-icase_literal | rg | wall | 32 | 6.191 | 6.294 | 7.639 | 7.948 | ms |
| agentflow | search | history_42-literal | current | peak_rss | 4 | 12214272.000 | 12210176.000 | 12222464.000 | 12222464.000 | bytes |
| agentflow | search | history_42-literal | current | wall | 32 | 5.435 | 5.547 | 6.835 | 6.886 | ms |
| agentflow | search | history_42-literal | rg | peak_rss | 4 | 6660096.000 | 6660096.000 | 6717440.000 | 6717440.000 | bytes |
| agentflow | search | history_42-literal | rg | wall | 32 | 6.198 | 6.414 | 7.879 | 9.377 | ms |
| agentflow | search | history_42-literal_lines | current | peak_rss | 4 | 11878400.000 | 11870208.000 | 11894784.000 | 11894784.000 | bytes |
| agentflow | search | history_42-literal_lines | current | wall | 32 | 6.151 | 6.177 | 6.714 | 6.857 | ms |
| agentflow | search | history_42-literal_lines | rg | peak_rss | 4 | 6791168.000 | 6787072.000 | 6832128.000 | 6832128.000 | bytes |
| agentflow | search | history_42-literal_lines | rg | wall | 32 | 6.611 | 6.904 | 8.808 | 9.104 | ms |
| agentflow | search | history_42-or | current | peak_rss | 4 | 13459456.000 | 13451264.000 | 13500416.000 | 13500416.000 | bytes |
| agentflow | search | history_42-or | current | wall | 32 | 6.158 | 6.216 | 6.882 | 7.374 | ms |
| agentflow | search | history_42-or | rg | peak_rss | 4 | 6897664.000 | 6901760.000 | 6979584.000 | 6979584.000 | bytes |
| agentflow | search | history_42-or | rg | wall | 32 | 6.767 | 6.951 | 9.032 | 11.534 | ms |
| agentflow | search | history_42-short | current | peak_rss | 4 | 11026432.000 | 11026432.000 | 11108352.000 | 11108352.000 | bytes |
| agentflow | search | history_42-short | current | wall | 32 | 6.188 | 6.183 | 7.169 | 7.223 | ms |
| agentflow | search | history_42-short | rg | peak_rss | 4 | 6668288.000 | 6668288.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | history_42-short | rg | wall | 32 | 6.674 | 7.036 | 8.170 | 9.209 | ms |
| agentflow | search | history_42-word | current | peak_rss | 4 | 11943936.000 | 11960320.000 | 12042240.000 | 12042240.000 | bytes |
| agentflow | search | history_42-word | current | wall | 32 | 6.256 | 6.314 | 7.140 | 7.226 | ms |
| agentflow | search | history_42-word | rg | peak_rss | 4 | 6914048.000 | 6918144.000 | 6979584.000 | 6979584.000 | bytes |
| agentflow | search | history_42-word | rg | wall | 32 | 6.746 | 6.955 | 7.957 | 8.156 | ms |
| agentflow | search | history_56-absent | current | peak_rss | 4 | 9895936.000 | 9895936.000 | 9912320.000 | 9912320.000 | bytes |
| agentflow | search | history_56-absent | current | wall | 32 | 4.480 | 4.629 | 5.493 | 5.698 | ms |
| agentflow | search | history_56-absent | rg | peak_rss | 4 | 6488064.000 | 6488064.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | search | history_56-absent | rg | wall | 32 | 6.088 | 6.478 | 8.641 | 10.870 | ms |
| agentflow | search | history_56-anchor | current | peak_rss | 4 | 12959744.000 | 12992512.000 | 13123584.000 | 13123584.000 | bytes |
| agentflow | search | history_56-anchor | current | wall | 32 | 7.295 | 7.477 | 8.492 | 10.932 | ms |
| agentflow | search | history_56-anchor | rg | peak_rss | 4 | 8019968.000 | 8028160.000 | 8077312.000 | 8077312.000 | bytes |
| agentflow | search | history_56-anchor | rg | wall | 32 | 8.632 | 8.855 | 10.529 | 13.100 | ms |
| agentflow | search | history_56-blank | current | peak_rss | 4 | 11280384.000 | 11280384.000 | 11321344.000 | 11321344.000 | bytes |
| agentflow | search | history_56-blank | current | wall | 32 | 8.953 | 9.120 | 11.551 | 12.430 | ms |
| agentflow | search | history_56-blank | rg | peak_rss | 4 | 6955008.000 | 6946816.000 | 6995968.000 | 6995968.000 | bytes |
| agentflow | search | history_56-blank | rg | wall | 32 | 8.202 | 8.343 | 10.478 | 10.563 | ms |
| agentflow | search | history_56-broad | current | peak_rss | 4 | 10739712.000 | 10752000.000 | 10813440.000 | 10813440.000 | bytes |
| agentflow | search | history_56-broad | current | wall | 32 | 5.743 | 5.843 | 6.600 | 7.337 | ms |
| agentflow | search | history_56-broad | rg | peak_rss | 4 | 6651904.000 | 6660096.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | history_56-broad | rg | wall | 32 | 7.651 | 7.716 | 9.102 | 9.829 | ms |
| agentflow | search | history_56-count | current | peak_rss | 4 | 10878976.000 | 10866688.000 | 10928128.000 | 10928128.000 | bytes |
| agentflow | search | history_56-count | current | wall | 32 | 6.673 | 7.007 | 8.512 | 11.852 | ms |
| agentflow | search | history_56-count | rg | peak_rss | 4 | 7413760.000 | 7401472.000 | 7454720.000 | 7454720.000 | bytes |
| agentflow | search | history_56-count | rg | wall | 32 | 7.975 | 8.031 | 9.079 | 10.518 | ms |
| agentflow | search | history_56-icase | current | peak_rss | 4 | 11059200.000 | 11063296.000 | 11124736.000 | 11124736.000 | bytes |
| agentflow | search | history_56-icase | current | wall | 32 | 5.001 | 5.109 | 6.264 | 6.303 | ms |
| agentflow | search | history_56-icase | rg | peak_rss | 4 | 6930432.000 | 6926336.000 | 6946816.000 | 6946816.000 | bytes |
| agentflow | search | history_56-icase | rg | wall | 32 | 6.541 | 6.932 | 8.432 | 11.214 | ms |
| agentflow | search | history_56-icase_literal | current | peak_rss | 4 | 10739712.000 | 10747904.000 | 10829824.000 | 10829824.000 | bytes |
| agentflow | search | history_56-icase_literal | current | wall | 32 | 4.751 | 4.877 | 5.846 | 6.826 | ms |
| agentflow | search | history_56-icase_literal | rg | peak_rss | 4 | 6660096.000 | 6656000.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | history_56-icase_literal | rg | wall | 32 | 6.066 | 6.479 | 7.912 | 9.581 | ms |
| agentflow | search | history_56-literal | current | peak_rss | 4 | 10600448.000 | 10608640.000 | 10649600.000 | 10649600.000 | bytes |
| agentflow | search | history_56-literal | current | wall | 32 | 5.212 | 5.256 | 6.056 | 6.394 | ms |
| agentflow | search | history_56-literal | rg | peak_rss | 4 | 6643712.000 | 6631424.000 | 6651904.000 | 6651904.000 | bytes |
| agentflow | search | history_56-literal | rg | wall | 32 | 6.897 | 7.208 | 8.576 | 11.382 | ms |
| agentflow | search | history_56-literal_lines | current | peak_rss | 4 | 10731520.000 | 10739712.000 | 10813440.000 | 10813440.000 | bytes |
| agentflow | search | history_56-literal_lines | current | wall | 32 | 6.367 | 6.500 | 7.834 | 8.593 | ms |
| agentflow | search | history_56-literal_lines | rg | peak_rss | 4 | 6840320.000 | 6828032.000 | 6848512.000 | 6848512.000 | bytes |
| agentflow | search | history_56-literal_lines | rg | wall | 32 | 7.048 | 7.375 | 8.966 | 9.120 | ms |
| agentflow | search | history_56-or | current | peak_rss | 4 | 11239424.000 | 11239424.000 | 11255808.000 | 11255808.000 | bytes |
| agentflow | search | history_56-or | current | wall | 32 | 5.864 | 5.949 | 6.884 | 7.124 | ms |
| agentflow | search | history_56-or | rg | peak_rss | 4 | 6922240.000 | 6918144.000 | 6963200.000 | 6963200.000 | bytes |
| agentflow | search | history_56-or | rg | wall | 32 | 7.070 | 7.407 | 8.921 | 9.666 | ms |
| agentflow | search | history_56-short | current | peak_rss | 4 | 10641408.000 | 10641408.000 | 10682368.000 | 10682368.000 | bytes |
| agentflow | search | history_56-short | current | wall | 32 | 6.269 | 6.238 | 7.350 | 7.776 | ms |
| agentflow | search | history_56-short | rg | peak_rss | 4 | 6668288.000 | 6672384.000 | 6717440.000 | 6717440.000 | bytes |
| agentflow | search | history_56-short | rg | wall | 32 | 7.994 | 7.893 | 9.087 | 9.397 | ms |
| agentflow | search | history_56-word | current | peak_rss | 4 | 10846208.000 | 10850304.000 | 10911744.000 | 10911744.000 | bytes |
| agentflow | search | history_56-word | current | wall | 32 | 6.285 | 6.513 | 8.476 | 9.101 | ms |
| agentflow | search | history_56-word | rg | peak_rss | 4 | 6938624.000 | 6938624.000 | 6995968.000 | 6995968.000 | bytes |
| agentflow | search | history_56-word | rg | wall | 32 | 7.044 | 7.514 | 9.534 | 13.287 | ms |
| agentflow | search | initial-absent | current | peak_rss | 2 | 9609216.000 | 9609216.000 | 9650176.000 | 9650176.000 | bytes |
| agentflow | search | initial-absent | current | wall | 16 | 4.559 | 4.780 | 6.507 | 6.507 | ms |
| agentflow | search | initial-absent | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | search | initial-absent | rg | wall | 16 | 6.595 | 6.766 | 8.562 | 8.562 | ms |
| agentflow | search | initial-anchor | current | peak_rss | 2 | 12099584.000 | 12099584.000 | 12124160.000 | 12124160.000 | bytes |
| agentflow | search | initial-anchor | current | wall | 16 | 7.768 | 7.848 | 10.066 | 10.066 | ms |
| agentflow | search | initial-anchor | rg | peak_rss | 2 | 8069120.000 | 8069120.000 | 8110080.000 | 8110080.000 | bytes |
| agentflow | search | initial-anchor | rg | wall | 16 | 9.186 | 9.320 | 11.315 | 11.315 | ms |
| agentflow | search | initial-blank | current | peak_rss | 2 | 11206656.000 | 11206656.000 | 11239424.000 | 11239424.000 | bytes |
| agentflow | search | initial-blank | current | wall | 16 | 9.549 | 9.480 | 10.394 | 10.394 | ms |
| agentflow | search | initial-blank | rg | peak_rss | 2 | 6881280.000 | 6881280.000 | 6897664.000 | 6897664.000 | bytes |
| agentflow | search | initial-blank | rg | wall | 16 | 8.658 | 8.625 | 11.048 | 11.048 | ms |
| agentflow | search | initial-broad | current | peak_rss | 2 | 10330112.000 | 10330112.000 | 10338304.000 | 10338304.000 | bytes |
| agentflow | search | initial-broad | current | wall | 16 | 5.944 | 6.105 | 7.673 | 7.673 | ms |
| agentflow | search | initial-broad | rg | peak_rss | 2 | 6651904.000 | 6651904.000 | 6701056.000 | 6701056.000 | bytes |
| agentflow | search | initial-broad | rg | wall | 16 | 7.428 | 7.480 | 8.619 | 8.619 | ms |
| agentflow | search | initial-count | current | peak_rss | 2 | 10723328.000 | 10723328.000 | 10764288.000 | 10764288.000 | bytes |
| agentflow | search | initial-count | current | wall | 16 | 7.280 | 7.468 | 8.993 | 8.993 | ms |
| agentflow | search | initial-count | rg | peak_rss | 2 | 7282688.000 | 7282688.000 | 7340032.000 | 7340032.000 | bytes |
| agentflow | search | initial-count | rg | wall | 16 | 8.924 | 8.739 | 10.712 | 10.712 | ms |
| agentflow | search | initial-icase | current | peak_rss | 2 | 10338304.000 | 10338304.000 | 10387456.000 | 10387456.000 | bytes |
| agentflow | search | initial-icase | current | wall | 16 | 5.054 | 5.227 | 6.626 | 6.626 | ms |
| agentflow | search | initial-icase | rg | peak_rss | 2 | 6938624.000 | 6938624.000 | 6979584.000 | 6979584.000 | bytes |
| agentflow | search | initial-icase | rg | wall | 16 | 6.327 | 6.745 | 8.439 | 8.439 | ms |
| agentflow | search | initial-icase_literal | current | peak_rss | 2 | 10067968.000 | 10067968.000 | 10092544.000 | 10092544.000 | bytes |
| agentflow | search | initial-icase_literal | current | wall | 16 | 5.259 | 5.292 | 6.453 | 6.453 | ms |
| agentflow | search | initial-icase_literal | rg | peak_rss | 2 | 6635520.000 | 6635520.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | initial-icase_literal | rg | wall | 16 | 6.436 | 6.828 | 9.037 | 9.037 | ms |
| agentflow | search | initial-literal | current | peak_rss | 2 | 10125312.000 | 10125312.000 | 10125312.000 | 10125312.000 | bytes |
| agentflow | search | initial-literal | current | wall | 16 | 5.356 | 5.737 | 7.315 | 7.315 | ms |
| agentflow | search | initial-literal | rg | peak_rss | 2 | 6643712.000 | 6643712.000 | 6651904.000 | 6651904.000 | bytes |
| agentflow | search | initial-literal | rg | wall | 16 | 7.715 | 7.655 | 9.150 | 9.150 | ms |
| agentflow | search | initial-literal_lines | current | peak_rss | 2 | 10403840.000 | 10403840.000 | 10403840.000 | 10403840.000 | bytes |
| agentflow | search | initial-literal_lines | current | wall | 16 | 6.762 | 6.660 | 8.090 | 8.090 | ms |
| agentflow | search | initial-literal_lines | rg | peak_rss | 2 | 6832128.000 | 6832128.000 | 6832128.000 | 6832128.000 | bytes |
| agentflow | search | initial-literal_lines | rg | wall | 16 | 7.895 | 8.038 | 9.610 | 9.610 | ms |
| agentflow | search | initial-or | current | peak_rss | 2 | 10772480.000 | 10772480.000 | 10797056.000 | 10797056.000 | bytes |
| agentflow | search | initial-or | current | wall | 16 | 6.342 | 6.369 | 7.328 | 7.328 | ms |
| agentflow | search | initial-or | rg | peak_rss | 2 | 6955008.000 | 6955008.000 | 6979584.000 | 6979584.000 | bytes |
| agentflow | search | initial-or | rg | wall | 16 | 7.367 | 7.377 | 8.573 | 8.573 | ms |
| agentflow | search | initial-short | current | peak_rss | 2 | 10461184.000 | 10461184.000 | 10502144.000 | 10502144.000 | bytes |
| agentflow | search | initial-short | current | wall | 16 | 6.489 | 6.610 | 8.037 | 8.037 | ms |
| agentflow | search | initial-short | rg | peak_rss | 2 | 6660096.000 | 6660096.000 | 6684672.000 | 6684672.000 | bytes |
| agentflow | search | initial-short | rg | wall | 16 | 8.458 | 8.519 | 11.747 | 11.747 | ms |
| agentflow | search | initial-word | current | peak_rss | 2 | 10600448.000 | 10600448.000 | 10665984.000 | 10665984.000 | bytes |
| agentflow | search | initial-word | current | wall | 16 | 6.418 | 6.433 | 7.663 | 7.663 | ms |
| agentflow | search | initial-word | rg | peak_rss | 2 | 6979584.000 | 6979584.000 | 7012352.000 | 7012352.000 | bytes |
| agentflow | search | initial-word | rg | wall | 16 | 8.074 | 7.876 | 10.479 | 10.479 | ms |
| agentflow | workflow | edit_1pct-add_commit | current | index_size | 3 | 3322053.000 | 3322053.000 | 3322053.000 | 3322053.000 | bytes |
| agentflow | workflow | edit_1pct-add_commit | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10387456.000 | 10387456.000 | bytes |
| agentflow | workflow | edit_1pct-add_commit | current | searchable_bytes | 3 | 2131033.000 | 2131033.000 | 2131033.000 | 2131033.000 | bytes |
| agentflow | workflow | edit_1pct-add_commit | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_1pct-add_commit | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_1pct-add_commit | current | wall | 3 | 6.423 | 6.322 | 6.566 | 6.566 | ms |
| agentflow | workflow | edit_1pct-add_commit | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6586368.000 | 6586368.000 | bytes |
| agentflow | workflow | edit_1pct-add_commit | rg | wall | 3 | 7.401 | 7.371 | 8.643 | 8.643 | ms |
| agentflow | workflow | edit_1pct-before_ignore | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-before_ignore | current | index_size | 3 | 3322451.000 | 3322451.000 | 3322451.000 | 3322451.000 | bytes |
| agentflow | workflow | edit_1pct-before_ignore | current | peak_rss | 2 | 10412032.000 | 10412032.000 | 10420224.000 | 10420224.000 | bytes |
| agentflow | workflow | edit_1pct-before_ignore | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-before_ignore | current | searchable_bytes | 3 | 2131035.000 | 2131035.000 | 2131035.000 | 2131035.000 | bytes |
| agentflow | workflow | edit_1pct-before_ignore | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_1pct-before_ignore | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_1pct-before_ignore | current | wall | 3 | 5.726 | 5.791 | 6.016 | 6.016 | ms |
| agentflow | workflow | edit_1pct-before_ignore | rg | peak_rss | 2 | 6479872.000 | 6479872.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | workflow | edit_1pct-before_ignore | rg | wall | 3 | 6.993 | 7.252 | 8.548 | 8.548 | ms |
| agentflow | workflow | edit_1pct-delete | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-delete | current | index_size | 3 | 3321969.000 | 3321969.000 | 3321969.000 | 3321969.000 | bytes |
| agentflow | workflow | edit_1pct-delete | current | peak_rss | 2 | 10174464.000 | 10174464.000 | 10174464.000 | 10174464.000 | bytes |
| agentflow | workflow | edit_1pct-delete | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-delete | current | searchable_bytes | 3 | 2131008.000 | 2131008.000 | 2131008.000 | 2131008.000 | bytes |
| agentflow | workflow | edit_1pct-delete | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_1pct-delete | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_1pct-delete | current | wall | 3 | 5.770 | 5.843 | 6.282 | 6.282 | ms |
| agentflow | workflow | edit_1pct-delete | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | workflow | edit_1pct-delete | rg | wall | 3 | 5.764 | 5.950 | 6.378 | 6.378 | ms |
| agentflow | workflow | edit_1pct-dirty | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-dirty | current | index_size | 3 | 3353771.000 | 3353771.000 | 3353771.000 | 3353771.000 | bytes |
| agentflow | workflow | edit_1pct-dirty | current | peak_rss | 2 | 11296768.000 | 11296768.000 | 11337728.000 | 11337728.000 | bytes |
| agentflow | workflow | edit_1pct-dirty | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-dirty | current | searchable_bytes | 3 | 2132705.000 | 2132705.000 | 2132705.000 | 2132705.000 | bytes |
| agentflow | workflow | edit_1pct-dirty | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_1pct-dirty | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_1pct-dirty | current | wall | 3 | 8.015 | 7.522 | 8.115 | 8.115 | ms |
| agentflow | workflow | edit_1pct-dirty | rg | peak_rss | 2 | 6471680.000 | 6471680.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | workflow | edit_1pct-dirty | rg | wall | 3 | 7.553 | 8.101 | 9.616 | 9.616 | ms |
| agentflow | workflow | edit_1pct-discard | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-discard | current | index_size | 3 | 3304708.000 | 3304708.000 | 3304708.000 | 3304708.000 | bytes |
| agentflow | workflow | edit_1pct-discard | current | peak_rss | 2 | 10207232.000 | 10207232.000 | 10240000.000 | 10240000.000 | bytes |
| agentflow | workflow | edit_1pct-discard | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-discard | current | searchable_bytes | 3 | 2131008.000 | 2131008.000 | 2131008.000 | 2131008.000 | bytes |
| agentflow | workflow | edit_1pct-discard | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_1pct-discard | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_1pct-discard | current | wall | 3 | 5.714 | 5.666 | 5.770 | 5.770 | ms |
| agentflow | workflow | edit_1pct-discard | rg | peak_rss | 2 | 6479872.000 | 6479872.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | workflow | edit_1pct-discard | rg | wall | 3 | 6.759 | 6.513 | 7.069 | 7.069 | ms |
| agentflow | workflow | edit_1pct-edit | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-edit | current | index_size | 3 | 3288201.000 | 3288201.000 | 3288201.000 | 3288201.000 | bytes |
| agentflow | workflow | edit_1pct-edit | current | peak_rss | 2 | 11321344.000 | 11321344.000 | 11337728.000 | 11337728.000 | bytes |
| agentflow | workflow | edit_1pct-edit | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-edit | current | searchable_bytes | 3 | 2131008.000 | 2131008.000 | 2131008.000 | 2131008.000 | bytes |
| agentflow | workflow | edit_1pct-edit | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_1pct-edit | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_1pct-edit | current | wall | 3 | 7.306 | 7.129 | 7.319 | 7.319 | ms |
| agentflow | workflow | edit_1pct-edit | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | workflow | edit_1pct-edit | rg | wall | 3 | 8.229 | 8.196 | 9.311 | 9.311 | ms |
| agentflow | workflow | edit_1pct-ignored | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-ignored | current | index_size | 3 | 3322305.000 | 3322305.000 | 3322305.000 | 3322305.000 | bytes |
| agentflow | workflow | edit_1pct-ignored | current | peak_rss | 2 | 10477568.000 | 10477568.000 | 10485760.000 | 10485760.000 | bytes |
| agentflow | workflow | edit_1pct-ignored | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-ignored | current | searchable_bytes | 3 | 2131013.000 | 2131013.000 | 2131013.000 | 2131013.000 | bytes |
| agentflow | workflow | edit_1pct-ignored | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_1pct-ignored | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_1pct-ignored | current | wall | 3 | 6.775 | 6.794 | 7.045 | 7.045 | ms |
| agentflow | workflow | edit_1pct-ignored | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | workflow | edit_1pct-ignored | rg | wall | 3 | 6.083 | 5.987 | 6.094 | 6.094 | ms |
| agentflow | workflow | edit_1pct-promotion | current | index_size | 3 | 3304708.000 | 3304708.000 | 3304708.000 | 3304708.000 | bytes |
| agentflow | workflow | edit_1pct-promotion | current | peak_rss | 2 | 10395648.000 | 10395648.000 | 10469376.000 | 10469376.000 | bytes |
| agentflow | workflow | edit_1pct-promotion | current | searchable_bytes | 3 | 2131008.000 | 2131008.000 | 2131008.000 | 2131008.000 | bytes |
| agentflow | workflow | edit_1pct-promotion | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_1pct-promotion | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_1pct-promotion | current | wall | 3 | 5.938 | 6.017 | 6.194 | 6.194 | ms |
| agentflow | workflow | edit_1pct-promotion | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6619136.000 | 6619136.000 | bytes |
| agentflow | workflow | edit_1pct-promotion | rg | wall | 3 | 6.002 | 6.010 | 6.253 | 6.253 | ms |
| agentflow | workflow | edit_1pct-rename | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-rename | current | index_size | 3 | 3322430.000 | 3322430.000 | 3322430.000 | 3322430.000 | bytes |
| agentflow | workflow | edit_1pct-rename | current | peak_rss | 2 | 10420224.000 | 10420224.000 | 10436608.000 | 10436608.000 | bytes |
| agentflow | workflow | edit_1pct-rename | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-rename | current | searchable_bytes | 3 | 2131033.000 | 2131033.000 | 2131033.000 | 2131033.000 | bytes |
| agentflow | workflow | edit_1pct-rename | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_1pct-rename | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_1pct-rename | current | wall | 3 | 6.145 | 6.545 | 7.768 | 7.768 | ms |
| agentflow | workflow | edit_1pct-rename | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6553600.000 | 6553600.000 | bytes |
| agentflow | workflow | edit_1pct-rename | rg | wall | 3 | 6.540 | 6.812 | 7.680 | 7.680 | ms |
| agentflow | workflow | edit_1pct-revisit | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-revisit | current | index_size | 3 | 3304708.000 | 3304708.000 | 3304708.000 | 3304708.000 | bytes |
| agentflow | workflow | edit_1pct-revisit | current | peak_rss | 2 | 10289152.000 | 10289152.000 | 10289152.000 | 10289152.000 | bytes |
| agentflow | workflow | edit_1pct-revisit | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-revisit | current | searchable_bytes | 3 | 2131008.000 | 2131008.000 | 2131008.000 | 2131008.000 | bytes |
| agentflow | workflow | edit_1pct-revisit | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_1pct-revisit | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_1pct-revisit | current | wall | 3 | 5.862 | 6.146 | 6.875 | 6.875 | ms |
| agentflow | workflow | edit_1pct-revisit | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | workflow | edit_1pct-revisit | rg | wall | 3 | 7.343 | 6.872 | 7.502 | 7.502 | ms |
| agentflow | workflow | edit_1pct-rollback | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-rollback | current | index_size | 3 | 3304602.000 | 3304602.000 | 3304602.000 | 3304602.000 | bytes |
| agentflow | workflow | edit_1pct-rollback | current | peak_rss | 2 | 10215424.000 | 10215424.000 | 10223616.000 | 10223616.000 | bytes |
| agentflow | workflow | edit_1pct-rollback | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-rollback | current | searchable_bytes | 3 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | workflow | edit_1pct-rollback | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_1pct-rollback | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-rollback | current | wall | 3 | 5.621 | 5.604 | 5.750 | 5.750 | ms |
| agentflow | workflow | edit_1pct-rollback | rg | peak_rss | 2 | 6463488.000 | 6463488.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | workflow | edit_1pct-rollback | rg | wall | 3 | 6.038 | 6.918 | 8.721 | 8.721 | ms |
| agentflow | workflow | edit_1pct-untracked | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_1pct-untracked | current | index_size | 3 | 3305263.000 | 3305263.000 | 3305263.000 | 3305263.000 | bytes |
| agentflow | workflow | edit_1pct-untracked | current | peak_rss | 2 | 10485760.000 | 10485760.000 | 10485760.000 | 10485760.000 | bytes |
| agentflow | workflow | edit_1pct-untracked | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_1pct-untracked | current | searchable_bytes | 3 | 2131033.000 | 2131033.000 | 2131033.000 | 2131033.000 | bytes |
| agentflow | workflow | edit_1pct-untracked | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_1pct-untracked | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_1pct-untracked | current | wall | 3 | 6.252 | 6.438 | 6.897 | 6.897 | ms |
| agentflow | workflow | edit_1pct-untracked | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | workflow | edit_1pct-untracked | rg | wall | 3 | 7.930 | 7.935 | 8.231 | 8.231 | ms |
| agentflow | workflow | edit_50pct-add_commit | current | index_size | 3 | 4100819.000 | 4100819.000 | 4100819.000 | 4100819.000 | bytes |
| agentflow | workflow | edit_50pct-add_commit | current | peak_rss | 2 | 10436608.000 | 10436608.000 | 10469376.000 | 10469376.000 | bytes |
| agentflow | workflow | edit_50pct-add_commit | current | searchable_bytes | 3 | 2238708.000 | 2238708.000 | 2238708.000 | 2238708.000 | bytes |
| agentflow | workflow | edit_50pct-add_commit | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_50pct-add_commit | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_50pct-add_commit | current | wall | 3 | 6.004 | 6.088 | 6.302 | 6.302 | ms |
| agentflow | workflow | edit_50pct-add_commit | rg | peak_rss | 2 | 6447104.000 | 6447104.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | workflow | edit_50pct-add_commit | rg | wall | 3 | 6.303 | 6.163 | 6.435 | 6.435 | ms |
| agentflow | workflow | edit_50pct-before_ignore | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_50pct-before_ignore | current | index_size | 3 | 4101217.000 | 4101217.000 | 4101217.000 | 4101217.000 | bytes |
| agentflow | workflow | edit_50pct-before_ignore | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10387456.000 | 10387456.000 | bytes |
| agentflow | workflow | edit_50pct-before_ignore | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-before_ignore | current | searchable_bytes | 3 | 2238710.000 | 2238710.000 | 2238710.000 | 2238710.000 | bytes |
| agentflow | workflow | edit_50pct-before_ignore | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_50pct-before_ignore | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_50pct-before_ignore | current | wall | 3 | 6.250 | 6.103 | 6.423 | 6.423 | ms |
| agentflow | workflow | edit_50pct-before_ignore | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | workflow | edit_50pct-before_ignore | rg | wall | 3 | 6.387 | 6.324 | 7.258 | 7.258 | ms |
| agentflow | workflow | edit_50pct-delete | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-delete | current | index_size | 3 | 4100735.000 | 4100735.000 | 4100735.000 | 4100735.000 | bytes |
| agentflow | workflow | edit_50pct-delete | current | peak_rss | 2 | 10223616.000 | 10223616.000 | 10289152.000 | 10289152.000 | bytes |
| agentflow | workflow | edit_50pct-delete | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-delete | current | searchable_bytes | 3 | 2238683.000 | 2238683.000 | 2238683.000 | 2238683.000 | bytes |
| agentflow | workflow | edit_50pct-delete | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_50pct-delete | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_50pct-delete | current | wall | 3 | 5.539 | 5.565 | 5.865 | 5.865 | ms |
| agentflow | workflow | edit_50pct-delete | rg | peak_rss | 2 | 6471680.000 | 6471680.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | workflow | edit_50pct-delete | rg | wall | 3 | 5.651 | 5.720 | 6.209 | 6.209 | ms |
| agentflow | workflow | edit_50pct-dirty | current | extracted | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | workflow | edit_50pct-dirty | current | index_size | 3 | 4926229.000 | 4926229.000 | 4926229.000 | 4926229.000 | bytes |
| agentflow | workflow | edit_50pct-dirty | current | peak_rss | 2 | 16728064.000 | 16728064.000 | 16809984.000 | 16809984.000 | bytes |
| agentflow | workflow | edit_50pct-dirty | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-dirty | current | searchable_bytes | 3 | 2340503.000 | 2340503.000 | 2340503.000 | 2340503.000 | bytes |
| agentflow | workflow | edit_50pct-dirty | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_50pct-dirty | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_50pct-dirty | current | wall | 3 | 16.272 | 16.252 | 16.494 | 16.494 | ms |
| agentflow | workflow | edit_50pct-dirty | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | workflow | edit_50pct-dirty | rg | wall | 3 | 10.085 | 10.355 | 12.716 | 12.716 | ms |
| agentflow | workflow | edit_50pct-discard | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-discard | current | index_size | 3 | 4083474.000 | 4083474.000 | 4083474.000 | 4083474.000 | bytes |
| agentflow | workflow | edit_50pct-discard | current | peak_rss | 2 | 10502144.000 | 10502144.000 | 10551296.000 | 10551296.000 | bytes |
| agentflow | workflow | edit_50pct-discard | current | reused | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | workflow | edit_50pct-discard | current | searchable_bytes | 3 | 2238683.000 | 2238683.000 | 2238683.000 | 2238683.000 | bytes |
| agentflow | workflow | edit_50pct-discard | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_50pct-discard | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_50pct-discard | current | wall | 3 | 6.802 | 6.833 | 7.087 | 7.087 | ms |
| agentflow | workflow | edit_50pct-discard | rg | peak_rss | 2 | 6471680.000 | 6471680.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | workflow | edit_50pct-discard | rg | wall | 3 | 6.613 | 6.528 | 7.167 | 7.167 | ms |
| agentflow | workflow | edit_50pct-edit | current | extracted | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | workflow | edit_50pct-edit | current | index_size | 3 | 4066967.000 | 4066967.000 | 4066967.000 | 4066967.000 | bytes |
| agentflow | workflow | edit_50pct-edit | current | peak_rss | 2 | 16408576.000 | 16408576.000 | 16482304.000 | 16482304.000 | bytes |
| agentflow | workflow | edit_50pct-edit | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-edit | current | searchable_bytes | 3 | 2238683.000 | 2238683.000 | 2238683.000 | 2238683.000 | bytes |
| agentflow | workflow | edit_50pct-edit | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_50pct-edit | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_50pct-edit | current | wall | 3 | 15.799 | 15.476 | 15.896 | 15.896 | ms |
| agentflow | workflow | edit_50pct-edit | rg | peak_rss | 2 | 6471680.000 | 6471680.000 | 6488064.000 | 6488064.000 | bytes |
| agentflow | workflow | edit_50pct-edit | rg | wall | 3 | 8.283 | 8.358 | 8.625 | 8.625 | ms |
| agentflow | workflow | edit_50pct-ignored | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_50pct-ignored | current | index_size | 3 | 4101071.000 | 4101071.000 | 4101071.000 | 4101071.000 | bytes |
| agentflow | workflow | edit_50pct-ignored | current | peak_rss | 2 | 10518528.000 | 10518528.000 | 10518528.000 | 10518528.000 | bytes |
| agentflow | workflow | edit_50pct-ignored | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-ignored | current | searchable_bytes | 3 | 2238688.000 | 2238688.000 | 2238688.000 | 2238688.000 | bytes |
| agentflow | workflow | edit_50pct-ignored | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_50pct-ignored | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_50pct-ignored | current | wall | 3 | 6.262 | 7.315 | 9.497 | 9.497 | ms |
| agentflow | workflow | edit_50pct-ignored | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | workflow | edit_50pct-ignored | rg | wall | 3 | 7.393 | 7.354 | 7.695 | 7.695 | ms |
| agentflow | workflow | edit_50pct-promotion | current | index_size | 3 | 4083474.000 | 4083474.000 | 4083474.000 | 4083474.000 | bytes |
| agentflow | workflow | edit_50pct-promotion | current | peak_rss | 2 | 10420224.000 | 10420224.000 | 10420224.000 | 10420224.000 | bytes |
| agentflow | workflow | edit_50pct-promotion | current | searchable_bytes | 3 | 2238683.000 | 2238683.000 | 2238683.000 | 2238683.000 | bytes |
| agentflow | workflow | edit_50pct-promotion | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_50pct-promotion | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_50pct-promotion | current | wall | 3 | 6.873 | 6.951 | 7.646 | 7.646 | ms |
| agentflow | workflow | edit_50pct-promotion | rg | peak_rss | 2 | 6479872.000 | 6479872.000 | 6504448.000 | 6504448.000 | bytes |
| agentflow | workflow | edit_50pct-promotion | rg | wall | 3 | 6.020 | 6.412 | 7.552 | 7.552 | ms |
| agentflow | workflow | edit_50pct-rename | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_50pct-rename | current | index_size | 3 | 4101196.000 | 4101196.000 | 4101196.000 | 4101196.000 | bytes |
| agentflow | workflow | edit_50pct-rename | current | peak_rss | 2 | 10444800.000 | 10444800.000 | 10452992.000 | 10452992.000 | bytes |
| agentflow | workflow | edit_50pct-rename | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-rename | current | searchable_bytes | 3 | 2238708.000 | 2238708.000 | 2238708.000 | 2238708.000 | bytes |
| agentflow | workflow | edit_50pct-rename | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_50pct-rename | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_50pct-rename | current | wall | 3 | 5.826 | 5.950 | 6.253 | 6.253 | ms |
| agentflow | workflow | edit_50pct-rename | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6569984.000 | 6569984.000 | bytes |
| agentflow | workflow | edit_50pct-rename | rg | wall | 3 | 8.150 | 8.883 | 10.852 | 10.852 | ms |
| agentflow | workflow | edit_50pct-revisit | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-revisit | current | index_size | 3 | 4083474.000 | 4083474.000 | 4083474.000 | 4083474.000 | bytes |
| agentflow | workflow | edit_50pct-revisit | current | peak_rss | 2 | 10493952.000 | 10493952.000 | 10518528.000 | 10518528.000 | bytes |
| agentflow | workflow | edit_50pct-revisit | current | reused | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | workflow | edit_50pct-revisit | current | searchable_bytes | 3 | 2238683.000 | 2238683.000 | 2238683.000 | 2238683.000 | bytes |
| agentflow | workflow | edit_50pct-revisit | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_50pct-revisit | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| agentflow | workflow | edit_50pct-revisit | current | wall | 3 | 6.341 | 6.400 | 6.985 | 6.985 | ms |
| agentflow | workflow | edit_50pct-revisit | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6537216.000 | 6537216.000 | bytes |
| agentflow | workflow | edit_50pct-revisit | rg | wall | 3 | 7.186 | 7.135 | 7.980 | 7.980 | ms |
| agentflow | workflow | edit_50pct-rollback | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-rollback | current | index_size | 3 | 4083368.000 | 4083368.000 | 4083368.000 | 4083368.000 | bytes |
| agentflow | workflow | edit_50pct-rollback | current | peak_rss | 2 | 10444800.000 | 10444800.000 | 10469376.000 | 10469376.000 | bytes |
| agentflow | workflow | edit_50pct-rollback | current | reused | 3 | 60.000 | 60.000 | 60.000 | 60.000 | count |
| agentflow | workflow | edit_50pct-rollback | current | searchable_bytes | 3 | 2129183.000 | 2129183.000 | 2129183.000 | 2129183.000 | bytes |
| agentflow | workflow | edit_50pct-rollback | current | searchable_files | 3 | 179.000 | 179.000 | 179.000 | 179.000 | count |
| agentflow | workflow | edit_50pct-rollback | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_50pct-rollback | current | wall | 3 | 6.637 | 6.449 | 6.761 | 6.761 | ms |
| agentflow | workflow | edit_50pct-rollback | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6520832.000 | 6520832.000 | bytes |
| agentflow | workflow | edit_50pct-rollback | rg | wall | 3 | 6.025 | 6.199 | 6.654 | 6.654 | ms |
| agentflow | workflow | edit_50pct-untracked | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| agentflow | workflow | edit_50pct-untracked | current | index_size | 3 | 4084029.000 | 4084029.000 | 4084029.000 | 4084029.000 | bytes |
| agentflow | workflow | edit_50pct-untracked | current | peak_rss | 2 | 10469376.000 | 10469376.000 | 10485760.000 | 10485760.000 | bytes |
| agentflow | workflow | edit_50pct-untracked | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| agentflow | workflow | edit_50pct-untracked | current | searchable_bytes | 3 | 2238708.000 | 2238708.000 | 2238708.000 | 2238708.000 | bytes |
| agentflow | workflow | edit_50pct-untracked | current | searchable_files | 3 | 180.000 | 180.000 | 180.000 | 180.000 | count |
| agentflow | workflow | edit_50pct-untracked | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| agentflow | workflow | edit_50pct-untracked | current | wall | 3 | 6.339 | 6.647 | 7.353 | 7.353 | ms |
| agentflow | workflow | edit_50pct-untracked | rg | peak_rss | 2 | 6463488.000 | 6463488.000 | 6471680.000 | 6471680.000 | bytes |
| agentflow | workflow | edit_50pct-untracked | rg | wall | 3 | 8.275 | 8.798 | 11.105 | 11.105 | ms |
| viberwhisper | branches | edit_1pct-commit_A1 | current | index_size | 3 | 2588438.000 | 2588438.000 | 2588438.000 | 2588438.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_A1 | current | peak_rss | 2 | 10313728.000 | 10313728.000 | 10321920.000 | 10321920.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_A1 | current | searchable_bytes | 3 | 1875933.000 | 1875933.000 | 1875933.000 | 1875933.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_A1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-commit_A1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-commit_A1 | current | wall | 3 | 6.012 | 6.309 | 6.947 | 6.947 | ms |
| viberwhisper | branches | edit_1pct-commit_A1 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_A1 | rg | wall | 3 | 5.825 | 6.013 | 6.594 | 6.594 | ms |
| viberwhisper | branches | edit_1pct-commit_A2 | current | index_size | 3 | 2676768.000 | 2676768.000 | 2676768.000 | 2676768.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_A2 | current | peak_rss | 2 | 10321920.000 | 10321920.000 | 10354688.000 | 10354688.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_A2 | current | searchable_bytes | 3 | 1877502.000 | 1877502.000 | 1877502.000 | 1877502.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_A2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-commit_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-commit_A2 | current | wall | 3 | 7.042 | 7.088 | 7.697 | 7.697 | ms |
| viberwhisper | branches | edit_1pct-commit_A2 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_A2 | rg | wall | 3 | 7.742 | 7.257 | 8.431 | 8.431 | ms |
| viberwhisper | branches | edit_1pct-commit_B1 | current | index_size | 3 | 2631823.000 | 2631823.000 | 2631823.000 | 2631823.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_B1 | current | peak_rss | 2 | 10346496.000 | 10346496.000 | 10371072.000 | 10371072.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_B1 | current | searchable_bytes | 3 | 1875933.000 | 1875933.000 | 1875933.000 | 1875933.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_B1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-commit_B1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-commit_B1 | current | wall | 3 | 7.132 | 7.335 | 7.870 | 7.870 | ms |
| viberwhisper | branches | edit_1pct-commit_B1 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_B1 | rg | wall | 3 | 7.807 | 7.849 | 8.246 | 8.246 | ms |
| viberwhisper | branches | edit_1pct-commit_B2 | current | index_size | 3 | 2721522.000 | 2721522.000 | 2721522.000 | 2721522.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_B2 | current | peak_rss | 2 | 10444800.000 | 10444800.000 | 10469376.000 | 10469376.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_B2 | current | searchable_bytes | 3 | 1877502.000 | 1877502.000 | 1877502.000 | 1877502.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_B2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-commit_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-commit_B2 | current | wall | 3 | 7.135 | 7.226 | 8.235 | 8.235 | ms |
| viberwhisper | branches | edit_1pct-commit_B2 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6504448.000 | 6504448.000 | bytes |
| viberwhisper | branches | edit_1pct-commit_B2 | rg | wall | 3 | 6.340 | 6.684 | 7.452 | 7.452 | ms |
| viberwhisper | branches | edit_1pct-edit_A1 | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-edit_A1 | current | index_size | 3 | 2573876.000 | 2573876.000 | 2573876.000 | 2573876.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_A1 | current | peak_rss | 2 | 11214848.000 | 11214848.000 | 11255808.000 | 11255808.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_A1 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-edit_A1 | current | searchable_bytes | 3 | 1875933.000 | 1875933.000 | 1875933.000 | 1875933.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_A1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-edit_A1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-edit_A1 | current | wall | 3 | 7.695 | 7.437 | 8.032 | 8.032 | ms |
| viberwhisper | branches | edit_1pct-edit_A1 | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6537216.000 | 6537216.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_A1 | rg | wall | 3 | 7.436 | 7.935 | 9.973 | 9.973 | ms |
| viberwhisper | branches | edit_1pct-edit_A2 | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-edit_A2 | current | index_size | 3 | 2662206.000 | 2662206.000 | 2662206.000 | 2662206.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_A2 | current | peak_rss | 2 | 11239424.000 | 11239424.000 | 11288576.000 | 11288576.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_A2 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-edit_A2 | current | searchable_bytes | 3 | 1877502.000 | 1877502.000 | 1877502.000 | 1877502.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_A2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-edit_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-edit_A2 | current | wall | 3 | 7.391 | 7.448 | 7.622 | 7.622 | ms |
| viberwhisper | branches | edit_1pct-edit_A2 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6520832.000 | 6520832.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_A2 | rg | wall | 3 | 6.075 | 6.243 | 6.916 | 6.916 | ms |
| viberwhisper | branches | edit_1pct-edit_B1 | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-edit_B1 | current | index_size | 3 | 2617261.000 | 2617261.000 | 2617261.000 | 2617261.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_B1 | current | peak_rss | 2 | 11190272.000 | 11190272.000 | 11190272.000 | 11190272.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_B1 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-edit_B1 | current | searchable_bytes | 3 | 1875933.000 | 1875933.000 | 1875933.000 | 1875933.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_B1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-edit_B1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-edit_B1 | current | wall | 3 | 7.629 | 7.318 | 7.655 | 7.655 | ms |
| viberwhisper | branches | edit_1pct-edit_B1 | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6635520.000 | 6635520.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_B1 | rg | wall | 3 | 8.241 | 8.310 | 9.390 | 9.390 | ms |
| viberwhisper | branches | edit_1pct-edit_B2 | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-edit_B2 | current | index_size | 3 | 2706960.000 | 2706960.000 | 2706960.000 | 2706960.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_B2 | current | peak_rss | 2 | 11247616.000 | 11247616.000 | 11272192.000 | 11272192.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_B2 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-edit_B2 | current | searchable_bytes | 3 | 1877502.000 | 1877502.000 | 1877502.000 | 1877502.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_B2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-edit_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-edit_B2 | current | wall | 3 | 7.335 | 7.080 | 7.482 | 7.482 | ms |
| viberwhisper | branches | edit_1pct-edit_B2 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | branches | edit_1pct-edit_B2 | rg | wall | 3 | 7.660 | 7.857 | 8.632 | 8.632 | ms |
| viberwhisper | branches | edit_1pct-revisit_A | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-revisit_A | current | index_size | 3 | 2721522.000 | 2721522.000 | 2721522.000 | 2721522.000 | bytes |
| viberwhisper | branches | edit_1pct-revisit_A | current | peak_rss | 2 | 10207232.000 | 10207232.000 | 10240000.000 | 10240000.000 | bytes |
| viberwhisper | branches | edit_1pct-revisit_A | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-revisit_A | current | searchable_bytes | 3 | 1877502.000 | 1877502.000 | 1877502.000 | 1877502.000 | bytes |
| viberwhisper | branches | edit_1pct-revisit_A | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-revisit_A | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-revisit_A | current | wall | 3 | 6.267 | 6.226 | 6.426 | 6.426 | ms |
| viberwhisper | branches | edit_1pct-revisit_A | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | branches | edit_1pct-revisit_A | rg | wall | 3 | 7.131 | 6.843 | 7.512 | 7.512 | ms |
| viberwhisper | branches | edit_1pct-revisit_B | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-revisit_B | current | index_size | 3 | 2721522.000 | 2721522.000 | 2721522.000 | 2721522.000 | bytes |
| viberwhisper | branches | edit_1pct-revisit_B | current | peak_rss | 2 | 10240000.000 | 10240000.000 | 10256384.000 | 10256384.000 | bytes |
| viberwhisper | branches | edit_1pct-revisit_B | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-revisit_B | current | searchable_bytes | 3 | 1877502.000 | 1877502.000 | 1877502.000 | 1877502.000 | bytes |
| viberwhisper | branches | edit_1pct-revisit_B | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-revisit_B | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-revisit_B | current | wall | 3 | 6.854 | 6.497 | 6.957 | 6.957 | ms |
| viberwhisper | branches | edit_1pct-revisit_B | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | branches | edit_1pct-revisit_B | rg | wall | 3 | 5.870 | 5.901 | 6.087 | 6.087 | ms |
| viberwhisper | branches | edit_1pct-switch_A1 | current | index_size | 3 | 2544779.000 | 2544779.000 | 2544779.000 | 2544779.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_A1 | current | peak_rss | 2 | 9658368.000 | 9658368.000 | 9682944.000 | 9682944.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_A1 | current | searchable_bytes | 3 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_A1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-switch_A1 | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-switch_A1 | current | wall | 3 | 5.206 | 5.117 | 5.461 | 5.461 | ms |
| viberwhisper | branches | edit_1pct-switch_A1 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_A1 | rg | wall | 3 | 6.800 | 6.426 | 6.802 | 6.802 | ms |
| viberwhisper | branches | edit_1pct-switch_A2 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-switch_A2 | current | index_size | 3 | 2631823.000 | 2631823.000 | 2631823.000 | 2631823.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_A2 | current | peak_rss | 2 | 10215424.000 | 10215424.000 | 10240000.000 | 10240000.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_A2 | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-switch_A2 | current | searchable_bytes | 3 | 1875933.000 | 1875933.000 | 1875933.000 | 1875933.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_A2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-switch_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-switch_A2 | current | wall | 3 | 5.636 | 5.692 | 5.964 | 5.964 | ms |
| viberwhisper | branches | edit_1pct-switch_A2 | rg | peak_rss | 2 | 6586368.000 | 6586368.000 | 6602752.000 | 6602752.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_A2 | rg | wall | 3 | 5.858 | 5.873 | 5.994 | 5.994 | ms |
| viberwhisper | branches | edit_1pct-switch_B1 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-switch_B1 | current | index_size | 3 | 2588332.000 | 2588332.000 | 2588332.000 | 2588332.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_B1 | current | peak_rss | 2 | 10182656.000 | 10182656.000 | 10240000.000 | 10240000.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_B1 | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-switch_B1 | current | searchable_bytes | 3 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_B1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-switch_B1 | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-switch_B1 | current | wall | 3 | 6.204 | 6.137 | 6.405 | 6.405 | ms |
| viberwhisper | branches | edit_1pct-switch_B1 | rg | peak_rss | 2 | 6479872.000 | 6479872.000 | 6504448.000 | 6504448.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_B1 | rg | wall | 3 | 6.590 | 6.795 | 7.211 | 7.211 | ms |
| viberwhisper | branches | edit_1pct-switch_B2 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_1pct-switch_B2 | current | index_size | 3 | 2676768.000 | 2676768.000 | 2676768.000 | 2676768.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_B2 | current | peak_rss | 2 | 10264576.000 | 10264576.000 | 10289152.000 | 10289152.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_B2 | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_1pct-switch_B2 | current | searchable_bytes | 3 | 1875933.000 | 1875933.000 | 1875933.000 | 1875933.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_B2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_1pct-switch_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_1pct-switch_B2 | current | wall | 3 | 6.577 | 6.442 | 6.916 | 6.916 | ms |
| viberwhisper | branches | edit_1pct-switch_B2 | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | branches | edit_1pct-switch_B2 | rg | wall | 3 | 7.041 | 6.842 | 7.248 | 7.248 | ms |
| viberwhisper | branches | edit_50pct-commit_A1 | current | index_size | 3 | 3079609.000 | 3079609.000 | 3079609.000 | 3079609.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_A1 | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10403840.000 | 10403840.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_A1 | current | searchable_bytes | 3 | 1918296.000 | 1918296.000 | 1918296.000 | 1918296.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_A1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-commit_A1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-commit_A1 | current | wall | 3 | 6.918 | 6.918 | 7.081 | 7.081 | ms |
| viberwhisper | branches | edit_50pct-commit_A1 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6537216.000 | 6537216.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_A1 | rg | wall | 3 | 6.414 | 6.665 | 7.259 | 7.259 | ms |
| viberwhisper | branches | edit_50pct-commit_A2 | current | index_size | 3 | 4156881.000 | 4156881.000 | 4156881.000 | 4156881.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_A2 | current | peak_rss | 2 | 10444800.000 | 10444800.000 | 10469376.000 | 10469376.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_A2 | current | searchable_bytes | 3 | 1962228.000 | 1962228.000 | 1962228.000 | 1962228.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_A2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-commit_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-commit_A2 | current | wall | 3 | 6.882 | 6.949 | 7.467 | 7.467 | ms |
| viberwhisper | branches | edit_50pct-commit_A2 | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_A2 | rg | wall | 3 | 5.882 | 6.211 | 6.948 | 6.948 | ms |
| viberwhisper | branches | edit_50pct-commit_B1 | current | index_size | 3 | 3613357.000 | 3613357.000 | 3613357.000 | 3613357.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_B1 | current | peak_rss | 2 | 10362880.000 | 10362880.000 | 10371072.000 | 10371072.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_B1 | current | searchable_bytes | 3 | 1918296.000 | 1918296.000 | 1918296.000 | 1918296.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_B1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-commit_B1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-commit_B1 | current | wall | 3 | 8.727 | 8.629 | 10.538 | 10.538 | ms |
| viberwhisper | branches | edit_50pct-commit_B1 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6520832.000 | 6520832.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_B1 | rg | wall | 3 | 5.882 | 6.087 | 6.578 | 6.578 | ms |
| viberwhisper | branches | edit_50pct-commit_B2 | current | index_size | 3 | 4699353.000 | 4699353.000 | 4699353.000 | 4699353.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_B2 | current | peak_rss | 2 | 10379264.000 | 10379264.000 | 10436608.000 | 10436608.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_B2 | current | searchable_bytes | 3 | 1962228.000 | 1962228.000 | 1962228.000 | 1962228.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_B2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-commit_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-commit_B2 | current | wall | 3 | 7.820 | 7.908 | 8.614 | 8.614 | ms |
| viberwhisper | branches | edit_50pct-commit_B2 | rg | peak_rss | 2 | 6553600.000 | 6553600.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | branches | edit_50pct-commit_B2 | rg | wall | 3 | 6.905 | 6.614 | 6.919 | 6.919 | ms |
| viberwhisper | branches | edit_50pct-edit_A1 | current | extracted | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-edit_A1 | current | index_size | 3 | 3065047.000 | 3065047.000 | 3065047.000 | 3065047.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_A1 | current | peak_rss | 2 | 15122432.000 | 15122432.000 | 15269888.000 | 15269888.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_A1 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-edit_A1 | current | searchable_bytes | 3 | 1918296.000 | 1918296.000 | 1918296.000 | 1918296.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_A1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-edit_A1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-edit_A1 | current | wall | 3 | 11.504 | 11.363 | 11.727 | 11.727 | ms |
| viberwhisper | branches | edit_50pct-edit_A1 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6520832.000 | 6520832.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_A1 | rg | wall | 3 | 6.750 | 7.257 | 8.978 | 8.978 | ms |
| viberwhisper | branches | edit_50pct-edit_A2 | current | extracted | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-edit_A2 | current | index_size | 3 | 4142319.000 | 4142319.000 | 4142319.000 | 4142319.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_A2 | current | peak_rss | 2 | 15507456.000 | 15507456.000 | 15515648.000 | 15515648.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_A2 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-edit_A2 | current | searchable_bytes | 3 | 1962228.000 | 1962228.000 | 1962228.000 | 1962228.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_A2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-edit_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-edit_A2 | current | wall | 3 | 12.482 | 12.728 | 13.597 | 13.597 | ms |
| viberwhisper | branches | edit_50pct-edit_A2 | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_A2 | rg | wall | 3 | 6.767 | 6.581 | 6.928 | 6.928 | ms |
| viberwhisper | branches | edit_50pct-edit_B1 | current | extracted | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-edit_B1 | current | index_size | 3 | 3598795.000 | 3598795.000 | 3598795.000 | 3598795.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_B1 | current | peak_rss | 2 | 15482880.000 | 15482880.000 | 15892480.000 | 15892480.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_B1 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-edit_B1 | current | searchable_bytes | 3 | 1918296.000 | 1918296.000 | 1918296.000 | 1918296.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_B1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-edit_B1 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-edit_B1 | current | wall | 3 | 10.669 | 11.982 | 14.685 | 14.685 | ms |
| viberwhisper | branches | edit_50pct-edit_B1 | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_B1 | rg | wall | 3 | 8.023 | 7.882 | 8.473 | 8.473 | ms |
| viberwhisper | branches | edit_50pct-edit_B2 | current | extracted | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-edit_B2 | current | index_size | 3 | 4684791.000 | 4684791.000 | 4684791.000 | 4684791.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_B2 | current | peak_rss | 2 | 15613952.000 | 15613952.000 | 16154624.000 | 16154624.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_B2 | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-edit_B2 | current | searchable_bytes | 3 | 1962228.000 | 1962228.000 | 1962228.000 | 1962228.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_B2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-edit_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-edit_B2 | current | wall | 3 | 11.137 | 11.830 | 13.530 | 13.530 | ms |
| viberwhisper | branches | edit_50pct-edit_B2 | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | branches | edit_50pct-edit_B2 | rg | wall | 3 | 7.062 | 7.231 | 8.501 | 8.501 | ms |
| viberwhisper | branches | edit_50pct-revisit_A | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-revisit_A | current | index_size | 3 | 4699353.000 | 4699353.000 | 4699353.000 | 4699353.000 | bytes |
| viberwhisper | branches | edit_50pct-revisit_A | current | peak_rss | 2 | 10444800.000 | 10444800.000 | 10452992.000 | 10452992.000 | bytes |
| viberwhisper | branches | edit_50pct-revisit_A | current | reused | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-revisit_A | current | searchable_bytes | 3 | 1962228.000 | 1962228.000 | 1962228.000 | 1962228.000 | bytes |
| viberwhisper | branches | edit_50pct-revisit_A | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-revisit_A | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-revisit_A | current | wall | 3 | 6.703 | 6.761 | 7.579 | 7.579 | ms |
| viberwhisper | branches | edit_50pct-revisit_A | rg | peak_rss | 2 | 6578176.000 | 6578176.000 | 6651904.000 | 6651904.000 | bytes |
| viberwhisper | branches | edit_50pct-revisit_A | rg | wall | 3 | 7.052 | 7.274 | 7.839 | 7.839 | ms |
| viberwhisper | branches | edit_50pct-revisit_B | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-revisit_B | current | index_size | 3 | 4699353.000 | 4699353.000 | 4699353.000 | 4699353.000 | bytes |
| viberwhisper | branches | edit_50pct-revisit_B | current | peak_rss | 2 | 10428416.000 | 10428416.000 | 10436608.000 | 10436608.000 | bytes |
| viberwhisper | branches | edit_50pct-revisit_B | current | reused | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-revisit_B | current | searchable_bytes | 3 | 1962228.000 | 1962228.000 | 1962228.000 | 1962228.000 | bytes |
| viberwhisper | branches | edit_50pct-revisit_B | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-revisit_B | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-revisit_B | current | wall | 3 | 6.646 | 6.407 | 6.840 | 6.840 | ms |
| viberwhisper | branches | edit_50pct-revisit_B | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6602752.000 | 6602752.000 | bytes |
| viberwhisper | branches | edit_50pct-revisit_B | rg | wall | 3 | 6.292 | 6.429 | 6.708 | 6.708 | ms |
| viberwhisper | branches | edit_50pct-switch_A1 | current | index_size | 3 | 2544779.000 | 2544779.000 | 2544779.000 | 2544779.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_A1 | current | peak_rss | 2 | 9682944.000 | 9682944.000 | 9699328.000 | 9699328.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_A1 | current | searchable_bytes | 3 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_A1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-switch_A1 | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_50pct-switch_A1 | current | wall | 3 | 4.868 | 5.176 | 6.040 | 6.040 | ms |
| viberwhisper | branches | edit_50pct-switch_A1 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_A1 | rg | wall | 3 | 5.983 | 6.049 | 6.298 | 6.298 | ms |
| viberwhisper | branches | edit_50pct-switch_A2 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-switch_A2 | current | index_size | 3 | 3613357.000 | 3613357.000 | 3613357.000 | 3613357.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_A2 | current | peak_rss | 2 | 10452992.000 | 10452992.000 | 10469376.000 | 10469376.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_A2 | current | reused | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-switch_A2 | current | searchable_bytes | 3 | 1918296.000 | 1918296.000 | 1918296.000 | 1918296.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_A2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-switch_A2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-switch_A2 | current | wall | 3 | 6.482 | 6.343 | 6.537 | 6.537 | ms |
| viberwhisper | branches | edit_50pct-switch_A2 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_A2 | rg | wall | 3 | 6.106 | 6.277 | 7.011 | 7.011 | ms |
| viberwhisper | branches | edit_50pct-switch_B1 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-switch_B1 | current | index_size | 3 | 3079503.000 | 3079503.000 | 3079503.000 | 3079503.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_B1 | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10403840.000 | 10403840.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_B1 | current | reused | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-switch_B1 | current | searchable_bytes | 3 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_B1 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-switch_B1 | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | branches | edit_50pct-switch_B1 | current | wall | 3 | 6.667 | 7.042 | 8.138 | 8.138 | ms |
| viberwhisper | branches | edit_50pct-switch_B1 | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_B1 | rg | wall | 3 | 6.647 | 7.170 | 8.380 | 8.380 | ms |
| viberwhisper | branches | edit_50pct-switch_B2 | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | branches | edit_50pct-switch_B2 | current | index_size | 3 | 4156881.000 | 4156881.000 | 4156881.000 | 4156881.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_B2 | current | peak_rss | 2 | 10493952.000 | 10493952.000 | 10518528.000 | 10518528.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_B2 | current | reused | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | branches | edit_50pct-switch_B2 | current | searchable_bytes | 3 | 1918296.000 | 1918296.000 | 1918296.000 | 1918296.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_B2 | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | branches | edit_50pct-switch_B2 | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | branches | edit_50pct-switch_B2 | current | wall | 3 | 6.715 | 6.506 | 6.880 | 6.880 | ms |
| viberwhisper | branches | edit_50pct-switch_B2 | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | branches | edit_50pct-switch_B2 | rg | wall | 3 | 5.735 | 5.931 | 6.534 | 6.534 | ms |
| viberwhisper | build | default | current | index_size | 3 | 2544779.000 | 2544779.000 | 2544779.000 | 2544779.000 | bytes |
| viberwhisper | build | default | current | peak_rss | 2 | 26615808.000 | 26615808.000 | 26705920.000 | 26705920.000 | bytes |
| viberwhisper | build | default | current | searchable_bytes | 3 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | build | default | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | build | default | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | build | default | current | wall | 3 | 57.258 | 56.608 | 59.078 | 59.078 | ms |
| viberwhisper | history | revisit_0 | current | extracted | 2 | 30.000 | 30.000 | 30.000 | 30.000 | count |
| viberwhisper | history | revisit_0 | current | index_size | 2 | 10570585.000 | 10570585.000 | 10570585.000 | 10570585.000 | bytes |
| viberwhisper | history | revisit_0 | current | peak_rss | 2 | 13320192.000 | 13320192.000 | 13320192.000 | 13320192.000 | bytes |
| viberwhisper | history | revisit_0 | current | reused | 2 | 30.000 | 30.000 | 30.000 | 30.000 | count |
| viberwhisper | history | revisit_0 | current | searchable_bytes | 2 | 101299.000 | 101299.000 | 101299.000 | 101299.000 | bytes |
| viberwhisper | history | revisit_0 | current | searchable_files | 2 | 34.000 | 34.000 | 34.000 | 34.000 | count |
| viberwhisper | history | revisit_0 | current | segments | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | revisit_0 | current | wall | 2 | 10.034 | 10.034 | 10.038 | 10.038 | ms |
| viberwhisper | history | revisit_0 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | revisit_0 | rg | wall | 2 | 5.915 | 5.915 | 5.918 | 5.918 | ms |
| viberwhisper | history | revisit_100 | current | extracted | 2 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | history | revisit_100 | current | index_size | 2 | 11473161.000 | 11473161.000 | 11473161.000 | 11473161.000 | bytes |
| viberwhisper | history | revisit_100 | current | peak_rss | 2 | 12034048.000 | 12034048.000 | 12075008.000 | 12075008.000 | bytes |
| viberwhisper | history | revisit_100 | current | reused | 2 | 133.000 | 133.000 | 133.000 | 133.000 | count |
| viberwhisper | history | revisit_100 | current | searchable_bytes | 2 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | history | revisit_100 | current | searchable_files | 2 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | history | revisit_100 | current | segments | 2 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | history | revisit_100 | current | wall | 2 | 12.383 | 12.383 | 12.905 | 12.905 | ms |
| viberwhisper | history | revisit_100 | rg | peak_rss | 2 | 6602752.000 | 6602752.000 | 6619136.000 | 6619136.000 | bytes |
| viberwhisper | history | revisit_100 | rg | wall | 2 | 6.968 | 6.968 | 7.339 | 7.339 | ms |
| viberwhisper | history | revisit_50 | current | extracted | 2 | 84.000 | 84.000 | 84.000 | 84.000 | count |
| viberwhisper | history | revisit_50 | current | index_size | 2 | 11465202.000 | 11465202.000 | 11465202.000 | 11465202.000 | bytes |
| viberwhisper | history | revisit_50 | current | peak_rss | 2 | 17440768.000 | 17440768.000 | 17580032.000 | 17580032.000 | bytes |
| viberwhisper | history | revisit_50 | current | reused | 2 | 84.000 | 84.000 | 84.000 | 84.000 | count |
| viberwhisper | history | revisit_50 | current | searchable_bytes | 2 | 663098.000 | 663098.000 | 663098.000 | 663098.000 | bytes |
| viberwhisper | history | revisit_50 | current | searchable_files | 2 | 86.000 | 86.000 | 86.000 | 86.000 | count |
| viberwhisper | history | revisit_50 | current | segments | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | revisit_50 | current | wall | 2 | 17.767 | 17.767 | 17.879 | 17.879 | ms |
| viberwhisper | history | revisit_50 | rg | peak_rss | 2 | 6389760.000 | 6389760.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | history | revisit_50 | rg | wall | 2 | 6.256 | 6.256 | 6.526 | 6.526 | ms |
| viberwhisper | history | step_001 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_001 | current | index_size | 2 | 265645.000 | 265645.000 | 265645.000 | 265645.000 | bytes |
| viberwhisper | history | step_001 | current | peak_rss | 2 | 10870784.000 | 10870784.000 | 10878976.000 | 10878976.000 | bytes |
| viberwhisper | history | step_001 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_001 | current | searchable_bytes | 2 | 101727.000 | 101727.000 | 101727.000 | 101727.000 | bytes |
| viberwhisper | history | step_001 | current | searchable_files | 2 | 34.000 | 34.000 | 34.000 | 34.000 | count |
| viberwhisper | history | step_001 | current | segments | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_001 | current | wall | 2 | 5.817 | 5.817 | 5.911 | 5.911 | ms |
| viberwhisper | history | step_001 | rg | peak_rss | 2 | 6299648.000 | 6299648.000 | 6324224.000 | 6324224.000 | bytes |
| viberwhisper | history | step_001 | rg | wall | 2 | 5.320 | 5.320 | 5.995 | 5.995 | ms |
| viberwhisper | history | step_002 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_002 | current | index_size | 2 | 439477.000 | 439477.000 | 439477.000 | 439477.000 | bytes |
| viberwhisper | history | step_002 | current | peak_rss | 2 | 12812288.000 | 12812288.000 | 12894208.000 | 12894208.000 | bytes |
| viberwhisper | history | step_002 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_002 | current | searchable_bytes | 2 | 154164.000 | 154164.000 | 154164.000 | 154164.000 | bytes |
| viberwhisper | history | step_002 | current | searchable_files | 2 | 36.000 | 36.000 | 36.000 | 36.000 | count |
| viberwhisper | history | step_002 | current | segments | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_002 | current | wall | 2 | 7.101 | 7.101 | 7.292 | 7.292 | ms |
| viberwhisper | history | step_002 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_002 | rg | wall | 2 | 4.750 | 4.750 | 4.796 | 4.796 | ms |
| viberwhisper | history | step_003 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_003 | current | index_size | 2 | 476113.000 | 476113.000 | 476113.000 | 476113.000 | bytes |
| viberwhisper | history | step_003 | current | peak_rss | 2 | 11124736.000 | 11124736.000 | 11173888.000 | 11173888.000 | bytes |
| viberwhisper | history | step_003 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_003 | current | searchable_bytes | 2 | 154239.000 | 154239.000 | 154239.000 | 154239.000 | bytes |
| viberwhisper | history | step_003 | current | searchable_files | 2 | 36.000 | 36.000 | 36.000 | 36.000 | count |
| viberwhisper | history | step_003 | current | segments | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_003 | current | wall | 2 | 5.932 | 5.932 | 5.945 | 5.945 | ms |
| viberwhisper | history | step_003 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_003 | rg | wall | 2 | 5.031 | 5.031 | 5.381 | 5.381 | ms |
| viberwhisper | history | step_004 | current | extracted | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_004 | current | index_size | 2 | 550618.000 | 550618.000 | 550618.000 | 550618.000 | bytes |
| viberwhisper | history | step_004 | current | peak_rss | 2 | 11378688.000 | 11378688.000 | 11403264.000 | 11403264.000 | bytes |
| viberwhisper | history | step_004 | current | reused | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_004 | current | searchable_bytes | 2 | 175611.000 | 175611.000 | 175611.000 | 175611.000 | bytes |
| viberwhisper | history | step_004 | current | searchable_files | 2 | 37.000 | 37.000 | 37.000 | 37.000 | count |
| viberwhisper | history | step_004 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_004 | current | wall | 2 | 6.219 | 6.219 | 6.446 | 6.446 | ms |
| viberwhisper | history | step_004 | rg | peak_rss | 2 | 6316032.000 | 6316032.000 | 6324224.000 | 6324224.000 | bytes |
| viberwhisper | history | step_004 | rg | wall | 2 | 4.156 | 4.156 | 4.666 | 4.666 | ms |
| viberwhisper | history | step_005 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_005 | current | index_size | 2 | 565555.000 | 565555.000 | 565555.000 | 565555.000 | bytes |
| viberwhisper | history | step_005 | current | peak_rss | 2 | 10330112.000 | 10330112.000 | 10338304.000 | 10338304.000 | bytes |
| viberwhisper | history | step_005 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_005 | current | searchable_bytes | 2 | 175599.000 | 175599.000 | 175599.000 | 175599.000 | bytes |
| viberwhisper | history | step_005 | current | searchable_files | 2 | 37.000 | 37.000 | 37.000 | 37.000 | count |
| viberwhisper | history | step_005 | current | segments | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_005 | current | wall | 2 | 5.622 | 5.622 | 5.700 | 5.700 | ms |
| viberwhisper | history | step_005 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_005 | rg | wall | 2 | 4.876 | 4.876 | 5.064 | 5.064 | ms |
| viberwhisper | history | step_006 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_006 | current | index_size | 2 | 576375.000 | 576375.000 | 576375.000 | 576375.000 | bytes |
| viberwhisper | history | step_006 | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10403840.000 | 10403840.000 | bytes |
| viberwhisper | history | step_006 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_006 | current | searchable_bytes | 2 | 175884.000 | 175884.000 | 175884.000 | 175884.000 | bytes |
| viberwhisper | history | step_006 | current | searchable_files | 2 | 37.000 | 37.000 | 37.000 | 37.000 | count |
| viberwhisper | history | step_006 | current | segments | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_006 | current | wall | 2 | 5.657 | 5.657 | 5.918 | 5.918 | ms |
| viberwhisper | history | step_006 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_006 | rg | wall | 2 | 4.096 | 4.096 | 4.701 | 4.701 | ms |
| viberwhisper | history | step_007 | current | extracted | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_007 | current | index_size | 2 | 651949.000 | 651949.000 | 651949.000 | 651949.000 | bytes |
| viberwhisper | history | step_007 | current | peak_rss | 2 | 11517952.000 | 11517952.000 | 11550720.000 | 11550720.000 | bytes |
| viberwhisper | history | step_007 | current | reused | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_007 | current | searchable_bytes | 2 | 176743.000 | 176743.000 | 176743.000 | 176743.000 | bytes |
| viberwhisper | history | step_007 | current | searchable_files | 2 | 37.000 | 37.000 | 37.000 | 37.000 | count |
| viberwhisper | history | step_007 | current | segments | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_007 | current | wall | 2 | 6.289 | 6.289 | 6.416 | 6.416 | ms |
| viberwhisper | history | step_007 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_007 | rg | wall | 2 | 4.264 | 4.264 | 4.758 | 4.758 | ms |
| viberwhisper | history | step_008 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_008 | current | index_size | 2 | 749371.000 | 749371.000 | 749371.000 | 749371.000 | bytes |
| viberwhisper | history | step_008 | current | peak_rss | 2 | 12296192.000 | 12296192.000 | 12320768.000 | 12320768.000 | bytes |
| viberwhisper | history | step_008 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_008 | current | searchable_bytes | 2 | 204535.000 | 204535.000 | 204535.000 | 204535.000 | bytes |
| viberwhisper | history | step_008 | current | searchable_files | 2 | 38.000 | 38.000 | 38.000 | 38.000 | count |
| viberwhisper | history | step_008 | current | segments | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_008 | current | wall | 2 | 7.079 | 7.079 | 7.397 | 7.397 | ms |
| viberwhisper | history | step_008 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_008 | rg | wall | 2 | 5.456 | 5.456 | 6.052 | 6.052 | ms |
| viberwhisper | history | step_009 | current | extracted | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_009 | current | index_size | 2 | 814043.000 | 814043.000 | 814043.000 | 814043.000 | bytes |
| viberwhisper | history | step_009 | current | peak_rss | 2 | 11583488.000 | 11583488.000 | 11698176.000 | 11698176.000 | bytes |
| viberwhisper | history | step_009 | current | reused | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_009 | current | searchable_bytes | 2 | 219881.000 | 219881.000 | 219881.000 | 219881.000 | bytes |
| viberwhisper | history | step_009 | current | searchable_files | 2 | 39.000 | 39.000 | 39.000 | 39.000 | count |
| viberwhisper | history | step_009 | current | segments | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_009 | current | wall | 2 | 6.303 | 6.303 | 6.420 | 6.420 | ms |
| viberwhisper | history | step_009 | rg | peak_rss | 2 | 6316032.000 | 6316032.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_009 | rg | wall | 2 | 3.889 | 3.889 | 3.994 | 3.994 | ms |
| viberwhisper | history | step_010 | current | extracted | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_010 | current | index_size | 2 | 913627.000 | 913627.000 | 913627.000 | 913627.000 | bytes |
| viberwhisper | history | step_010 | current | peak_rss | 2 | 12148736.000 | 12148736.000 | 12222464.000 | 12222464.000 | bytes |
| viberwhisper | history | step_010 | current | reused | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_010 | current | searchable_bytes | 2 | 218187.000 | 218187.000 | 218187.000 | 218187.000 | bytes |
| viberwhisper | history | step_010 | current | searchable_files | 2 | 39.000 | 39.000 | 39.000 | 39.000 | count |
| viberwhisper | history | step_010 | current | segments | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_010 | current | wall | 2 | 6.912 | 6.912 | 7.074 | 7.074 | ms |
| viberwhisper | history | step_010 | rg | peak_rss | 2 | 6389760.000 | 6389760.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_010 | rg | wall | 2 | 4.168 | 4.168 | 4.738 | 4.738 | ms |
| viberwhisper | history | step_011 | current | extracted | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_011 | current | index_size | 2 | 1142794.000 | 1142794.000 | 1142794.000 | 1142794.000 | bytes |
| viberwhisper | history | step_011 | current | peak_rss | 2 | 13508608.000 | 13508608.000 | 13680640.000 | 13680640.000 | bytes |
| viberwhisper | history | step_011 | current | reused | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_011 | current | searchable_bytes | 2 | 266810.000 | 266810.000 | 266810.000 | 266810.000 | bytes |
| viberwhisper | history | step_011 | current | searchable_files | 2 | 43.000 | 43.000 | 43.000 | 43.000 | count |
| viberwhisper | history | step_011 | current | segments | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_011 | current | wall | 2 | 8.286 | 8.286 | 8.415 | 8.415 | ms |
| viberwhisper | history | step_011 | rg | peak_rss | 2 | 6307840.000 | 6307840.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_011 | rg | wall | 2 | 4.776 | 4.776 | 4.804 | 4.804 | ms |
| viberwhisper | history | step_012 | current | extracted | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_012 | current | index_size | 2 | 1200352.000 | 1200352.000 | 1200352.000 | 1200352.000 | bytes |
| viberwhisper | history | step_012 | current | peak_rss | 2 | 11780096.000 | 11780096.000 | 11829248.000 | 11829248.000 | bytes |
| viberwhisper | history | step_012 | current | reused | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_012 | current | searchable_bytes | 2 | 265896.000 | 265896.000 | 265896.000 | 265896.000 | bytes |
| viberwhisper | history | step_012 | current | searchable_files | 2 | 43.000 | 43.000 | 43.000 | 43.000 | count |
| viberwhisper | history | step_012 | current | segments | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| viberwhisper | history | step_012 | current | wall | 2 | 6.563 | 6.563 | 6.642 | 6.642 | ms |
| viberwhisper | history | step_012 | rg | peak_rss | 2 | 6307840.000 | 6307840.000 | 6340608.000 | 6340608.000 | bytes |
| viberwhisper | history | step_012 | rg | wall | 2 | 4.722 | 4.722 | 4.774 | 4.774 | ms |
| viberwhisper | history | step_013 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_013 | current | index_size | 2 | 1211643.000 | 1211643.000 | 1211643.000 | 1211643.000 | bytes |
| viberwhisper | history | step_013 | current | peak_rss | 2 | 10584064.000 | 10584064.000 | 10633216.000 | 10633216.000 | bytes |
| viberwhisper | history | step_013 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_013 | current | searchable_bytes | 2 | 266874.000 | 266874.000 | 266874.000 | 266874.000 | bytes |
| viberwhisper | history | step_013 | current | searchable_files | 2 | 43.000 | 43.000 | 43.000 | 43.000 | count |
| viberwhisper | history | step_013 | current | segments | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| viberwhisper | history | step_013 | current | wall | 2 | 5.865 | 5.865 | 6.005 | 6.005 | ms |
| viberwhisper | history | step_013 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6340608.000 | 6340608.000 | bytes |
| viberwhisper | history | step_013 | rg | wall | 2 | 5.336 | 5.336 | 5.907 | 5.907 | ms |
| viberwhisper | history | step_014 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_014 | current | index_size | 2 | 1152968.000 | 1152968.000 | 1152968.000 | 1152968.000 | bytes |
| viberwhisper | history | step_014 | current | peak_rss | 2 | 11206656.000 | 11206656.000 | 11288576.000 | 11288576.000 | bytes |
| viberwhisper | history | step_014 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_014 | current | searchable_bytes | 2 | 267270.000 | 267270.000 | 267270.000 | 267270.000 | bytes |
| viberwhisper | history | step_014 | current | searchable_files | 2 | 43.000 | 43.000 | 43.000 | 43.000 | count |
| viberwhisper | history | step_014 | current | segments | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_014 | current | wall | 2 | 6.690 | 6.690 | 6.813 | 6.813 | ms |
| viberwhisper | history | step_014 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_014 | rg | wall | 2 | 4.762 | 4.762 | 4.894 | 4.894 | ms |
| viberwhisper | history | step_015 | current | extracted | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_015 | current | index_size | 2 | 1209100.000 | 1209100.000 | 1209100.000 | 1209100.000 | bytes |
| viberwhisper | history | step_015 | current | peak_rss | 2 | 12189696.000 | 12189696.000 | 12304384.000 | 12304384.000 | bytes |
| viberwhisper | history | step_015 | current | reused | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_015 | current | searchable_bytes | 2 | 282780.000 | 282780.000 | 282780.000 | 282780.000 | bytes |
| viberwhisper | history | step_015 | current | searchable_files | 2 | 48.000 | 48.000 | 48.000 | 48.000 | count |
| viberwhisper | history | step_015 | current | segments | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| viberwhisper | history | step_015 | current | wall | 2 | 7.528 | 7.528 | 7.885 | 7.885 | ms |
| viberwhisper | history | step_015 | rg | peak_rss | 2 | 6316032.000 | 6316032.000 | 6340608.000 | 6340608.000 | bytes |
| viberwhisper | history | step_015 | rg | wall | 2 | 4.551 | 4.551 | 5.432 | 5.432 | ms |
| viberwhisper | history | step_016 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_016 | current | index_size | 2 | 1265587.000 | 1265587.000 | 1265587.000 | 1265587.000 | bytes |
| viberwhisper | history | step_016 | current | peak_rss | 2 | 11730944.000 | 11730944.000 | 11780096.000 | 11780096.000 | bytes |
| viberwhisper | history | step_016 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_016 | current | searchable_bytes | 2 | 290868.000 | 290868.000 | 290868.000 | 290868.000 | bytes |
| viberwhisper | history | step_016 | current | searchable_files | 2 | 49.000 | 49.000 | 49.000 | 49.000 | count |
| viberwhisper | history | step_016 | current | segments | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_016 | current | wall | 2 | 6.785 | 6.785 | 6.798 | 6.798 | ms |
| viberwhisper | history | step_016 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6340608.000 | 6340608.000 | bytes |
| viberwhisper | history | step_016 | rg | wall | 2 | 4.886 | 4.886 | 5.047 | 5.047 | ms |
| viberwhisper | history | step_017 | current | extracted | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_017 | current | index_size | 2 | 1305176.000 | 1305176.000 | 1305176.000 | 1305176.000 | bytes |
| viberwhisper | history | step_017 | current | peak_rss | 2 | 11542528.000 | 11542528.000 | 11583488.000 | 11583488.000 | bytes |
| viberwhisper | history | step_017 | current | reused | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_017 | current | searchable_bytes | 2 | 290240.000 | 290240.000 | 290240.000 | 290240.000 | bytes |
| viberwhisper | history | step_017 | current | searchable_files | 2 | 49.000 | 49.000 | 49.000 | 49.000 | count |
| viberwhisper | history | step_017 | current | segments | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| viberwhisper | history | step_017 | current | wall | 2 | 7.095 | 7.095 | 7.208 | 7.208 | ms |
| viberwhisper | history | step_017 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | history | step_017 | rg | wall | 2 | 4.387 | 4.387 | 5.074 | 5.074 | ms |
| viberwhisper | history | step_018 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_018 | current | index_size | 2 | 1319890.000 | 1319890.000 | 1319890.000 | 1319890.000 | bytes |
| viberwhisper | history | step_018 | current | peak_rss | 2 | 11272192.000 | 11272192.000 | 11304960.000 | 11304960.000 | bytes |
| viberwhisper | history | step_018 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_018 | current | searchable_bytes | 2 | 290279.000 | 290279.000 | 290279.000 | 290279.000 | bytes |
| viberwhisper | history | step_018 | current | searchable_files | 2 | 49.000 | 49.000 | 49.000 | 49.000 | count |
| viberwhisper | history | step_018 | current | segments | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| viberwhisper | history | step_018 | current | wall | 2 | 6.676 | 6.676 | 6.682 | 6.682 | ms |
| viberwhisper | history | step_018 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6340608.000 | 6340608.000 | bytes |
| viberwhisper | history | step_018 | rg | wall | 2 | 3.704 | 3.704 | 3.892 | 3.892 | ms |
| viberwhisper | history | step_019 | current | extracted | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_019 | current | index_size | 2 | 1543747.000 | 1543747.000 | 1543747.000 | 1543747.000 | bytes |
| viberwhisper | history | step_019 | current | peak_rss | 2 | 13770752.000 | 13770752.000 | 13828096.000 | 13828096.000 | bytes |
| viberwhisper | history | step_019 | current | reused | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_019 | current | searchable_bytes | 2 | 306320.000 | 306320.000 | 306320.000 | 306320.000 | bytes |
| viberwhisper | history | step_019 | current | searchable_files | 2 | 53.000 | 53.000 | 53.000 | 53.000 | count |
| viberwhisper | history | step_019 | current | segments | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| viberwhisper | history | step_019 | current | wall | 2 | 9.018 | 9.018 | 9.413 | 9.413 | ms |
| viberwhisper | history | step_019 | rg | peak_rss | 2 | 6316032.000 | 6316032.000 | 6340608.000 | 6340608.000 | bytes |
| viberwhisper | history | step_019 | rg | wall | 2 | 5.669 | 5.669 | 6.278 | 6.278 | ms |
| viberwhisper | history | step_020 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_020 | current | index_size | 2 | 1496118.000 | 1496118.000 | 1496118.000 | 1496118.000 | bytes |
| viberwhisper | history | step_020 | current | peak_rss | 2 | 11272192.000 | 11272192.000 | 11288576.000 | 11288576.000 | bytes |
| viberwhisper | history | step_020 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_020 | current | searchable_bytes | 2 | 306320.000 | 306320.000 | 306320.000 | 306320.000 | bytes |
| viberwhisper | history | step_020 | current | searchable_files | 2 | 53.000 | 53.000 | 53.000 | 53.000 | count |
| viberwhisper | history | step_020 | current | segments | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| viberwhisper | history | step_020 | current | wall | 2 | 6.194 | 6.194 | 6.223 | 6.223 | ms |
| viberwhisper | history | step_020 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_020 | rg | wall | 2 | 6.089 | 6.089 | 6.253 | 6.253 | ms |
| viberwhisper | history | step_021 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_021 | current | index_size | 2 | 1550301.000 | 1550301.000 | 1550301.000 | 1550301.000 | bytes |
| viberwhisper | history | step_021 | current | peak_rss | 2 | 11853824.000 | 11853824.000 | 11878400.000 | 11878400.000 | bytes |
| viberwhisper | history | step_021 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_021 | current | searchable_bytes | 2 | 330671.000 | 330671.000 | 330671.000 | 330671.000 | bytes |
| viberwhisper | history | step_021 | current | searchable_files | 2 | 53.000 | 53.000 | 53.000 | 53.000 | count |
| viberwhisper | history | step_021 | current | segments | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| viberwhisper | history | step_021 | current | wall | 2 | 7.116 | 7.116 | 7.531 | 7.531 | ms |
| viberwhisper | history | step_021 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6340608.000 | 6340608.000 | bytes |
| viberwhisper | history | step_021 | rg | wall | 2 | 3.504 | 3.504 | 3.604 | 3.604 | ms |
| viberwhisper | history | step_022 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_022 | current | index_size | 2 | 1556220.000 | 1556220.000 | 1556220.000 | 1556220.000 | bytes |
| viberwhisper | history | step_022 | current | peak_rss | 2 | 11698176.000 | 11698176.000 | 11714560.000 | 11714560.000 | bytes |
| viberwhisper | history | step_022 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_022 | current | searchable_bytes | 2 | 334994.000 | 334994.000 | 334994.000 | 334994.000 | bytes |
| viberwhisper | history | step_022 | current | searchable_files | 2 | 53.000 | 53.000 | 53.000 | 53.000 | count |
| viberwhisper | history | step_022 | current | segments | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_022 | current | wall | 2 | 6.859 | 6.859 | 7.091 | 7.091 | ms |
| viberwhisper | history | step_022 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_022 | rg | wall | 2 | 5.757 | 5.757 | 6.814 | 6.814 | ms |
| viberwhisper | history | step_023 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_023 | current | index_size | 2 | 1559817.000 | 1559817.000 | 1559817.000 | 1559817.000 | bytes |
| viberwhisper | history | step_023 | current | peak_rss | 2 | 10911744.000 | 10911744.000 | 10944512.000 | 10944512.000 | bytes |
| viberwhisper | history | step_023 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_023 | current | searchable_bytes | 2 | 335140.000 | 335140.000 | 335140.000 | 335140.000 | bytes |
| viberwhisper | history | step_023 | current | searchable_files | 2 | 54.000 | 54.000 | 54.000 | 54.000 | count |
| viberwhisper | history | step_023 | current | segments | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_023 | current | wall | 2 | 6.746 | 6.746 | 6.920 | 6.920 | ms |
| viberwhisper | history | step_023 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_023 | rg | wall | 2 | 4.713 | 4.713 | 4.739 | 4.739 | ms |
| viberwhisper | history | step_024 | current | extracted | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_024 | current | index_size | 2 | 1906142.000 | 1906142.000 | 1906142.000 | 1906142.000 | bytes |
| viberwhisper | history | step_024 | current | peak_rss | 2 | 14409728.000 | 14409728.000 | 14663680.000 | 14663680.000 | bytes |
| viberwhisper | history | step_024 | current | reused | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_024 | current | searchable_bytes | 2 | 448100.000 | 448100.000 | 448100.000 | 448100.000 | bytes |
| viberwhisper | history | step_024 | current | searchable_files | 2 | 63.000 | 63.000 | 63.000 | 63.000 | count |
| viberwhisper | history | step_024 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_024 | current | wall | 2 | 10.023 | 10.023 | 10.124 | 10.124 | ms |
| viberwhisper | history | step_024 | rg | peak_rss | 2 | 6307840.000 | 6307840.000 | 6324224.000 | 6324224.000 | bytes |
| viberwhisper | history | step_024 | rg | wall | 2 | 5.739 | 5.739 | 6.234 | 6.234 | ms |
| viberwhisper | history | step_025 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_025 | current | index_size | 2 | 2014986.000 | 2014986.000 | 2014986.000 | 2014986.000 | bytes |
| viberwhisper | history | step_025 | current | peak_rss | 2 | 12632064.000 | 12632064.000 | 12697600.000 | 12697600.000 | bytes |
| viberwhisper | history | step_025 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_025 | current | searchable_bytes | 2 | 458238.000 | 458238.000 | 458238.000 | 458238.000 | bytes |
| viberwhisper | history | step_025 | current | searchable_files | 2 | 64.000 | 64.000 | 64.000 | 64.000 | count |
| viberwhisper | history | step_025 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_025 | current | wall | 2 | 8.267 | 8.267 | 8.576 | 8.576 | ms |
| viberwhisper | history | step_025 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_025 | rg | wall | 2 | 6.510 | 6.510 | 6.748 | 6.748 | ms |
| viberwhisper | history | step_026 | current | extracted | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_026 | current | index_size | 2 | 2298333.000 | 2298333.000 | 2298333.000 | 2298333.000 | bytes |
| viberwhisper | history | step_026 | current | peak_rss | 2 | 14106624.000 | 14106624.000 | 14188544.000 | 14188544.000 | bytes |
| viberwhisper | history | step_026 | current | reused | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_026 | current | searchable_bytes | 2 | 470908.000 | 470908.000 | 470908.000 | 470908.000 | bytes |
| viberwhisper | history | step_026 | current | searchable_files | 2 | 64.000 | 64.000 | 64.000 | 64.000 | count |
| viberwhisper | history | step_026 | current | segments | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_026 | current | wall | 2 | 10.137 | 10.137 | 10.227 | 10.227 | ms |
| viberwhisper | history | step_026 | rg | peak_rss | 2 | 6373376.000 | 6373376.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_026 | rg | wall | 2 | 6.188 | 6.188 | 6.423 | 6.423 | ms |
| viberwhisper | history | step_027 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_027 | current | index_size | 2 | 2339524.000 | 2339524.000 | 2339524.000 | 2339524.000 | bytes |
| viberwhisper | history | step_027 | current | peak_rss | 2 | 11911168.000 | 11911168.000 | 11943936.000 | 11943936.000 | bytes |
| viberwhisper | history | step_027 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_027 | current | searchable_bytes | 2 | 476334.000 | 476334.000 | 476334.000 | 476334.000 | bytes |
| viberwhisper | history | step_027 | current | searchable_files | 2 | 64.000 | 64.000 | 64.000 | 64.000 | count |
| viberwhisper | history | step_027 | current | segments | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_027 | current | wall | 2 | 7.156 | 7.156 | 7.215 | 7.215 | ms |
| viberwhisper | history | step_027 | rg | peak_rss | 2 | 6332416.000 | 6332416.000 | 6340608.000 | 6340608.000 | bytes |
| viberwhisper | history | step_027 | rg | wall | 2 | 5.811 | 5.811 | 6.285 | 6.285 | ms |
| viberwhisper | history | step_028 | current | extracted | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_028 | current | index_size | 2 | 2465289.000 | 2465289.000 | 2465289.000 | 2465289.000 | bytes |
| viberwhisper | history | step_028 | current | peak_rss | 2 | 12574720.000 | 12574720.000 | 12648448.000 | 12648448.000 | bytes |
| viberwhisper | history | step_028 | current | reused | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_028 | current | searchable_bytes | 2 | 505864.000 | 505864.000 | 505864.000 | 505864.000 | bytes |
| viberwhisper | history | step_028 | current | searchable_files | 2 | 78.000 | 78.000 | 78.000 | 78.000 | count |
| viberwhisper | history | step_028 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_028 | current | wall | 2 | 8.404 | 8.404 | 8.932 | 8.932 | ms |
| viberwhisper | history | step_028 | rg | peak_rss | 2 | 6389760.000 | 6389760.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_028 | rg | wall | 2 | 5.747 | 5.747 | 6.571 | 6.571 | ms |
| viberwhisper | history | step_029 | current | extracted | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_029 | current | index_size | 2 | 2573073.000 | 2573073.000 | 2573073.000 | 2573073.000 | bytes |
| viberwhisper | history | step_029 | current | peak_rss | 2 | 13025280.000 | 13025280.000 | 13090816.000 | 13090816.000 | bytes |
| viberwhisper | history | step_029 | current | reused | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_029 | current | searchable_bytes | 2 | 506089.000 | 506089.000 | 506089.000 | 506089.000 | bytes |
| viberwhisper | history | step_029 | current | searchable_files | 2 | 78.000 | 78.000 | 78.000 | 78.000 | count |
| viberwhisper | history | step_029 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_029 | current | wall | 2 | 8.925 | 8.925 | 10.284 | 10.284 | ms |
| viberwhisper | history | step_029 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_029 | rg | wall | 2 | 5.753 | 5.753 | 6.416 | 6.416 | ms |
| viberwhisper | history | step_030 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_030 | current | index_size | 2 | 2724726.000 | 2724726.000 | 2724726.000 | 2724726.000 | bytes |
| viberwhisper | history | step_030 | current | peak_rss | 2 | 13271040.000 | 13271040.000 | 13320192.000 | 13320192.000 | bytes |
| viberwhisper | history | step_030 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_030 | current | searchable_bytes | 2 | 498027.000 | 498027.000 | 498027.000 | 498027.000 | bytes |
| viberwhisper | history | step_030 | current | searchable_files | 2 | 77.000 | 77.000 | 77.000 | 77.000 | count |
| viberwhisper | history | step_030 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_030 | current | wall | 2 | 7.955 | 7.955 | 8.046 | 8.046 | ms |
| viberwhisper | history | step_030 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_030 | rg | wall | 2 | 6.551 | 6.551 | 6.571 | 6.571 | ms |
| viberwhisper | history | step_031 | current | extracted | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| viberwhisper | history | step_031 | current | index_size | 2 | 2941139.000 | 2941139.000 | 2941139.000 | 2941139.000 | bytes |
| viberwhisper | history | step_031 | current | peak_rss | 2 | 14237696.000 | 14237696.000 | 14450688.000 | 14450688.000 | bytes |
| viberwhisper | history | step_031 | current | reused | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| viberwhisper | history | step_031 | current | searchable_bytes | 2 | 499063.000 | 499063.000 | 499063.000 | 499063.000 | bytes |
| viberwhisper | history | step_031 | current | searchable_files | 2 | 77.000 | 77.000 | 77.000 | 77.000 | count |
| viberwhisper | history | step_031 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_031 | current | wall | 2 | 8.593 | 8.593 | 8.616 | 8.616 | ms |
| viberwhisper | history | step_031 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_031 | rg | wall | 2 | 5.208 | 5.208 | 5.354 | 5.354 | ms |
| viberwhisper | history | step_032 | current | extracted | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_032 | current | index_size | 2 | 3039993.000 | 3039993.000 | 3039993.000 | 3039993.000 | bytes |
| viberwhisper | history | step_032 | current | peak_rss | 2 | 12787712.000 | 12787712.000 | 12845056.000 | 12845056.000 | bytes |
| viberwhisper | history | step_032 | current | reused | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_032 | current | searchable_bytes | 2 | 505569.000 | 505569.000 | 505569.000 | 505569.000 | bytes |
| viberwhisper | history | step_032 | current | searchable_files | 2 | 77.000 | 77.000 | 77.000 | 77.000 | count |
| viberwhisper | history | step_032 | current | segments | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_032 | current | wall | 2 | 8.026 | 8.026 | 8.066 | 8.066 | ms |
| viberwhisper | history | step_032 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_032 | rg | wall | 2 | 5.552 | 5.552 | 6.053 | 6.053 | ms |
| viberwhisper | history | step_033 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_033 | current | index_size | 2 | 3128134.000 | 3128134.000 | 3128134.000 | 3128134.000 | bytes |
| viberwhisper | history | step_033 | current | peak_rss | 2 | 13295616.000 | 13295616.000 | 13336576.000 | 13336576.000 | bytes |
| viberwhisper | history | step_033 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_033 | current | searchable_bytes | 2 | 507240.000 | 507240.000 | 507240.000 | 507240.000 | bytes |
| viberwhisper | history | step_033 | current | searchable_files | 2 | 77.000 | 77.000 | 77.000 | 77.000 | count |
| viberwhisper | history | step_033 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_033 | current | wall | 2 | 8.742 | 8.742 | 9.153 | 9.153 | ms |
| viberwhisper | history | step_033 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_033 | rg | wall | 2 | 6.060 | 6.060 | 6.119 | 6.119 | ms |
| viberwhisper | history | step_034 | current | extracted | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_034 | current | index_size | 2 | 3224518.000 | 3224518.000 | 3224518.000 | 3224518.000 | bytes |
| viberwhisper | history | step_034 | current | peak_rss | 2 | 12664832.000 | 12664832.000 | 12713984.000 | 12713984.000 | bytes |
| viberwhisper | history | step_034 | current | reused | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_034 | current | searchable_bytes | 2 | 511142.000 | 511142.000 | 511142.000 | 511142.000 | bytes |
| viberwhisper | history | step_034 | current | searchable_files | 2 | 77.000 | 77.000 | 77.000 | 77.000 | count |
| viberwhisper | history | step_034 | current | segments | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_034 | current | wall | 2 | 8.347 | 8.347 | 8.525 | 8.525 | ms |
| viberwhisper | history | step_034 | rg | peak_rss | 2 | 6422528.000 | 6422528.000 | 6471680.000 | 6471680.000 | bytes |
| viberwhisper | history | step_034 | rg | wall | 2 | 5.623 | 5.623 | 5.692 | 5.692 | ms |
| viberwhisper | history | step_035 | current | extracted | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_035 | current | index_size | 2 | 3391037.000 | 3391037.000 | 3391037.000 | 3391037.000 | bytes |
| viberwhisper | history | step_035 | current | peak_rss | 2 | 13942784.000 | 13942784.000 | 13991936.000 | 13991936.000 | bytes |
| viberwhisper | history | step_035 | current | reused | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_035 | current | searchable_bytes | 2 | 523744.000 | 523744.000 | 523744.000 | 523744.000 | bytes |
| viberwhisper | history | step_035 | current | searchable_files | 2 | 77.000 | 77.000 | 77.000 | 77.000 | count |
| viberwhisper | history | step_035 | current | segments | 2 | 19.000 | 19.000 | 19.000 | 19.000 | count |
| viberwhisper | history | step_035 | current | wall | 2 | 8.466 | 8.466 | 8.545 | 8.545 | ms |
| viberwhisper | history | step_035 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_035 | rg | wall | 2 | 5.481 | 5.481 | 6.082 | 6.082 | ms |
| viberwhisper | history | step_036 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_036 | current | index_size | 2 | 3472994.000 | 3472994.000 | 3472994.000 | 3472994.000 | bytes |
| viberwhisper | history | step_036 | current | peak_rss | 2 | 12951552.000 | 12951552.000 | 13041664.000 | 13041664.000 | bytes |
| viberwhisper | history | step_036 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_036 | current | searchable_bytes | 2 | 525156.000 | 525156.000 | 525156.000 | 525156.000 | bytes |
| viberwhisper | history | step_036 | current | searchable_files | 2 | 77.000 | 77.000 | 77.000 | 77.000 | count |
| viberwhisper | history | step_036 | current | segments | 2 | 19.000 | 19.000 | 19.000 | 19.000 | count |
| viberwhisper | history | step_036 | current | wall | 2 | 8.086 | 8.086 | 8.304 | 8.304 | ms |
| viberwhisper | history | step_036 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_036 | rg | wall | 2 | 6.529 | 6.529 | 6.642 | 6.642 | ms |
| viberwhisper | history | step_037 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_037 | current | index_size | 2 | 3539976.000 | 3539976.000 | 3539976.000 | 3539976.000 | bytes |
| viberwhisper | history | step_037 | current | peak_rss | 2 | 13189120.000 | 13189120.000 | 13238272.000 | 13238272.000 | bytes |
| viberwhisper | history | step_037 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_037 | current | searchable_bytes | 2 | 529157.000 | 529157.000 | 529157.000 | 529157.000 | bytes |
| viberwhisper | history | step_037 | current | searchable_files | 2 | 77.000 | 77.000 | 77.000 | 77.000 | count |
| viberwhisper | history | step_037 | current | segments | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| viberwhisper | history | step_037 | current | wall | 2 | 7.748 | 7.748 | 7.834 | 7.834 | ms |
| viberwhisper | history | step_037 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_037 | rg | wall | 2 | 5.424 | 5.424 | 5.705 | 5.705 | ms |
| viberwhisper | history | step_038 | current | extracted | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_038 | current | index_size | 2 | 3808689.000 | 3808689.000 | 3808689.000 | 3808689.000 | bytes |
| viberwhisper | history | step_038 | current | peak_rss | 2 | 15712256.000 | 15712256.000 | 15859712.000 | 15859712.000 | bytes |
| viberwhisper | history | step_038 | current | reused | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_038 | current | searchable_bytes | 2 | 577948.000 | 577948.000 | 577948.000 | 577948.000 | bytes |
| viberwhisper | history | step_038 | current | searchable_files | 2 | 75.000 | 75.000 | 75.000 | 75.000 | count |
| viberwhisper | history | step_038 | current | segments | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_038 | current | wall | 2 | 11.076 | 11.076 | 11.411 | 11.411 | ms |
| viberwhisper | history | step_038 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_038 | rg | wall | 2 | 5.863 | 5.863 | 6.701 | 6.701 | ms |
| viberwhisper | history | step_039 | current | extracted | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_039 | current | index_size | 2 | 3852321.000 | 3852321.000 | 3852321.000 | 3852321.000 | bytes |
| viberwhisper | history | step_039 | current | peak_rss | 2 | 11993088.000 | 11993088.000 | 12009472.000 | 12009472.000 | bytes |
| viberwhisper | history | step_039 | current | reused | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_039 | current | searchable_bytes | 2 | 577889.000 | 577889.000 | 577889.000 | 577889.000 | bytes |
| viberwhisper | history | step_039 | current | searchable_files | 2 | 75.000 | 75.000 | 75.000 | 75.000 | count |
| viberwhisper | history | step_039 | current | segments | 2 | 19.000 | 19.000 | 19.000 | 19.000 | count |
| viberwhisper | history | step_039 | current | wall | 2 | 7.266 | 7.266 | 7.338 | 7.338 | ms |
| viberwhisper | history | step_039 | rg | peak_rss | 2 | 6340608.000 | 6340608.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_039 | rg | wall | 2 | 5.072 | 5.072 | 5.270 | 5.270 | ms |
| viberwhisper | history | step_040 | current | extracted | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_040 | current | index_size | 2 | 3772828.000 | 3772828.000 | 3772828.000 | 3772828.000 | bytes |
| viberwhisper | history | step_040 | current | peak_rss | 2 | 13557760.000 | 13557760.000 | 13565952.000 | 13565952.000 | bytes |
| viberwhisper | history | step_040 | current | reused | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_040 | current | searchable_bytes | 2 | 600028.000 | 600028.000 | 600028.000 | 600028.000 | bytes |
| viberwhisper | history | step_040 | current | searchable_files | 2 | 76.000 | 76.000 | 76.000 | 76.000 | count |
| viberwhisper | history | step_040 | current | segments | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| viberwhisper | history | step_040 | current | wall | 2 | 9.598 | 9.598 | 10.239 | 10.239 | ms |
| viberwhisper | history | step_040 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_040 | rg | wall | 2 | 5.540 | 5.540 | 5.934 | 5.934 | ms |
| viberwhisper | history | step_041 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_041 | current | index_size | 2 | 3819173.000 | 3819173.000 | 3819173.000 | 3819173.000 | bytes |
| viberwhisper | history | step_041 | current | peak_rss | 2 | 12173312.000 | 12173312.000 | 12206080.000 | 12206080.000 | bytes |
| viberwhisper | history | step_041 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_041 | current | searchable_bytes | 2 | 601482.000 | 601482.000 | 601482.000 | 601482.000 | bytes |
| viberwhisper | history | step_041 | current | searchable_files | 2 | 76.000 | 76.000 | 76.000 | 76.000 | count |
| viberwhisper | history | step_041 | current | segments | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| viberwhisper | history | step_041 | current | wall | 2 | 7.339 | 7.339 | 7.739 | 7.739 | ms |
| viberwhisper | history | step_041 | rg | peak_rss | 2 | 6406144.000 | 6406144.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_041 | rg | wall | 2 | 6.075 | 6.075 | 6.130 | 6.130 | ms |
| viberwhisper | history | step_042 | current | extracted | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_042 | current | index_size | 2 | 4046848.000 | 4046848.000 | 4046848.000 | 4046848.000 | bytes |
| viberwhisper | history | step_042 | current | peak_rss | 2 | 14852096.000 | 14852096.000 | 14860288.000 | 14860288.000 | bytes |
| viberwhisper | history | step_042 | current | reused | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_042 | current | searchable_bytes | 2 | 605279.000 | 605279.000 | 605279.000 | 605279.000 | bytes |
| viberwhisper | history | step_042 | current | searchable_files | 2 | 78.000 | 78.000 | 78.000 | 78.000 | count |
| viberwhisper | history | step_042 | current | segments | 2 | 22.000 | 22.000 | 22.000 | 22.000 | count |
| viberwhisper | history | step_042 | current | wall | 2 | 10.938 | 10.938 | 11.271 | 11.271 | ms |
| viberwhisper | history | step_042 | rg | peak_rss | 2 | 6373376.000 | 6373376.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_042 | rg | wall | 2 | 5.790 | 5.790 | 6.342 | 6.342 | ms |
| viberwhisper | history | step_043 | current | extracted | 2 | 34.000 | 34.000 | 34.000 | 34.000 | count |
| viberwhisper | history | step_043 | current | index_size | 2 | 4557392.000 | 4557392.000 | 4557392.000 | 4557392.000 | bytes |
| viberwhisper | history | step_043 | current | peak_rss | 2 | 15622144.000 | 15622144.000 | 15859712.000 | 15859712.000 | bytes |
| viberwhisper | history | step_043 | current | reused | 2 | 34.000 | 34.000 | 34.000 | 34.000 | count |
| viberwhisper | history | step_043 | current | searchable_bytes | 2 | 651313.000 | 651313.000 | 651313.000 | 651313.000 | bytes |
| viberwhisper | history | step_043 | current | searchable_files | 2 | 81.000 | 81.000 | 81.000 | 81.000 | count |
| viberwhisper | history | step_043 | current | segments | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_043 | current | wall | 2 | 13.997 | 13.997 | 15.602 | 15.602 | ms |
| viberwhisper | history | step_043 | rg | peak_rss | 2 | 6373376.000 | 6373376.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_043 | rg | wall | 2 | 5.764 | 5.764 | 6.595 | 6.595 | ms |
| viberwhisper | history | step_044 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_044 | current | index_size | 2 | 4566514.000 | 4566514.000 | 4566514.000 | 4566514.000 | bytes |
| viberwhisper | history | step_044 | current | peak_rss | 2 | 11091968.000 | 11091968.000 | 11124736.000 | 11124736.000 | bytes |
| viberwhisper | history | step_044 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_044 | current | searchable_bytes | 2 | 652380.000 | 652380.000 | 652380.000 | 652380.000 | bytes |
| viberwhisper | history | step_044 | current | searchable_files | 2 | 82.000 | 82.000 | 82.000 | 82.000 | count |
| viberwhisper | history | step_044 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_044 | current | wall | 2 | 8.008 | 8.008 | 8.163 | 8.163 | ms |
| viberwhisper | history | step_044 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_044 | rg | wall | 2 | 6.447 | 6.447 | 6.931 | 6.931 | ms |
| viberwhisper | history | step_045 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_045 | current | index_size | 2 | 4540686.000 | 4540686.000 | 4540686.000 | 4540686.000 | bytes |
| viberwhisper | history | step_045 | current | peak_rss | 2 | 14147584.000 | 14147584.000 | 14188544.000 | 14188544.000 | bytes |
| viberwhisper | history | step_045 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_045 | current | searchable_bytes | 2 | 650807.000 | 650807.000 | 650807.000 | 650807.000 | bytes |
| viberwhisper | history | step_045 | current | searchable_files | 2 | 82.000 | 82.000 | 82.000 | 82.000 | count |
| viberwhisper | history | step_045 | current | segments | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_045 | current | wall | 2 | 9.137 | 9.137 | 9.337 | 9.337 | ms |
| viberwhisper | history | step_045 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_045 | rg | wall | 2 | 6.149 | 6.149 | 6.437 | 6.437 | ms |
| viberwhisper | history | step_046 | current | extracted | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| viberwhisper | history | step_046 | current | index_size | 2 | 4867817.000 | 4867817.000 | 4867817.000 | 4867817.000 | bytes |
| viberwhisper | history | step_046 | current | peak_rss | 2 | 14630912.000 | 14630912.000 | 14663680.000 | 14663680.000 | bytes |
| viberwhisper | history | step_046 | current | reused | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| viberwhisper | history | step_046 | current | searchable_bytes | 2 | 648969.000 | 648969.000 | 648969.000 | 648969.000 | bytes |
| viberwhisper | history | step_046 | current | searchable_files | 2 | 84.000 | 84.000 | 84.000 | 84.000 | count |
| viberwhisper | history | step_046 | current | segments | 2 | 19.000 | 19.000 | 19.000 | 19.000 | count |
| viberwhisper | history | step_046 | current | wall | 2 | 10.142 | 10.142 | 10.272 | 10.272 | ms |
| viberwhisper | history | step_046 | rg | peak_rss | 2 | 6430720.000 | 6430720.000 | 6488064.000 | 6488064.000 | bytes |
| viberwhisper | history | step_046 | rg | wall | 2 | 7.570 | 7.570 | 8.202 | 8.202 | ms |
| viberwhisper | history | step_047 | current | extracted | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_047 | current | index_size | 2 | 5196664.000 | 5196664.000 | 5196664.000 | 5196664.000 | bytes |
| viberwhisper | history | step_047 | current | peak_rss | 2 | 14909440.000 | 14909440.000 | 14958592.000 | 14958592.000 | bytes |
| viberwhisper | history | step_047 | current | reused | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_047 | current | searchable_bytes | 2 | 654819.000 | 654819.000 | 654819.000 | 654819.000 | bytes |
| viberwhisper | history | step_047 | current | searchable_files | 2 | 85.000 | 85.000 | 85.000 | 85.000 | count |
| viberwhisper | history | step_047 | current | segments | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| viberwhisper | history | step_047 | current | wall | 2 | 10.844 | 10.844 | 11.074 | 11.074 | ms |
| viberwhisper | history | step_047 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_047 | rg | wall | 2 | 5.298 | 5.298 | 5.618 | 5.618 | ms |
| viberwhisper | history | step_048 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_048 | current | index_size | 2 | 5224679.000 | 5224679.000 | 5224679.000 | 5224679.000 | bytes |
| viberwhisper | history | step_048 | current | peak_rss | 2 | 11878400.000 | 11878400.000 | 11894784.000 | 11894784.000 | bytes |
| viberwhisper | history | step_048 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_048 | current | searchable_bytes | 2 | 649437.000 | 649437.000 | 649437.000 | 649437.000 | bytes |
| viberwhisper | history | step_048 | current | searchable_files | 2 | 85.000 | 85.000 | 85.000 | 85.000 | count |
| viberwhisper | history | step_048 | current | segments | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| viberwhisper | history | step_048 | current | wall | 2 | 8.711 | 8.711 | 8.900 | 8.900 | ms |
| viberwhisper | history | step_048 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_048 | rg | wall | 2 | 6.118 | 6.118 | 7.140 | 7.140 | ms |
| viberwhisper | history | step_049 | current | extracted | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_049 | current | index_size | 2 | 5290458.000 | 5290458.000 | 5290458.000 | 5290458.000 | bytes |
| viberwhisper | history | step_049 | current | peak_rss | 2 | 12656640.000 | 12656640.000 | 12713984.000 | 12713984.000 | bytes |
| viberwhisper | history | step_049 | current | reused | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_049 | current | searchable_bytes | 2 | 649065.000 | 649065.000 | 649065.000 | 649065.000 | bytes |
| viberwhisper | history | step_049 | current | searchable_files | 2 | 85.000 | 85.000 | 85.000 | 85.000 | count |
| viberwhisper | history | step_049 | current | segments | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| viberwhisper | history | step_049 | current | wall | 2 | 8.420 | 8.420 | 9.206 | 9.206 | ms |
| viberwhisper | history | step_049 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_049 | rg | wall | 2 | 5.607 | 5.607 | 5.738 | 5.738 | ms |
| viberwhisper | history | step_050 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_050 | current | index_size | 2 | 4439784.000 | 4439784.000 | 4439784.000 | 4439784.000 | bytes |
| viberwhisper | history | step_050 | current | peak_rss | 2 | 13058048.000 | 13058048.000 | 13074432.000 | 13074432.000 | bytes |
| viberwhisper | history | step_050 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_050 | current | searchable_bytes | 2 | 663098.000 | 663098.000 | 663098.000 | 663098.000 | bytes |
| viberwhisper | history | step_050 | current | searchable_files | 2 | 86.000 | 86.000 | 86.000 | 86.000 | count |
| viberwhisper | history | step_050 | current | segments | 2 | 22.000 | 22.000 | 22.000 | 22.000 | count |
| viberwhisper | history | step_050 | current | wall | 2 | 9.434 | 9.434 | 9.731 | 9.731 | ms |
| viberwhisper | history | step_050 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_050 | rg | wall | 2 | 6.202 | 6.202 | 6.808 | 6.808 | ms |
| viberwhisper | history | step_051 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_051 | current | index_size | 2 | 4487294.000 | 4487294.000 | 4487294.000 | 4487294.000 | bytes |
| viberwhisper | history | step_051 | current | peak_rss | 2 | 12419072.000 | 12419072.000 | 12468224.000 | 12468224.000 | bytes |
| viberwhisper | history | step_051 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_051 | current | searchable_bytes | 2 | 663098.000 | 663098.000 | 663098.000 | 663098.000 | bytes |
| viberwhisper | history | step_051 | current | searchable_files | 2 | 86.000 | 86.000 | 86.000 | 86.000 | count |
| viberwhisper | history | step_051 | current | segments | 2 | 23.000 | 23.000 | 23.000 | 23.000 | count |
| viberwhisper | history | step_051 | current | wall | 2 | 7.959 | 7.959 | 8.143 | 8.143 | ms |
| viberwhisper | history | step_051 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_051 | rg | wall | 2 | 5.677 | 5.677 | 5.835 | 5.835 | ms |
| viberwhisper | history | step_052 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_052 | current | index_size | 2 | 4698382.000 | 4698382.000 | 4698382.000 | 4698382.000 | bytes |
| viberwhisper | history | step_052 | current | peak_rss | 2 | 14843904.000 | 14843904.000 | 14843904.000 | 14843904.000 | bytes |
| viberwhisper | history | step_052 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_052 | current | searchable_bytes | 2 | 669864.000 | 669864.000 | 669864.000 | 669864.000 | bytes |
| viberwhisper | history | step_052 | current | searchable_files | 2 | 87.000 | 87.000 | 87.000 | 87.000 | count |
| viberwhisper | history | step_052 | current | segments | 2 | 23.000 | 23.000 | 23.000 | 23.000 | count |
| viberwhisper | history | step_052 | current | wall | 2 | 9.436 | 9.436 | 9.648 | 9.648 | ms |
| viberwhisper | history | step_052 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_052 | rg | wall | 2 | 5.849 | 5.849 | 6.291 | 6.291 | ms |
| viberwhisper | history | step_053 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_053 | current | index_size | 2 | 4877336.000 | 4877336.000 | 4877336.000 | 4877336.000 | bytes |
| viberwhisper | history | step_053 | current | peak_rss | 2 | 14884864.000 | 14884864.000 | 14893056.000 | 14893056.000 | bytes |
| viberwhisper | history | step_053 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_053 | current | searchable_bytes | 2 | 670139.000 | 670139.000 | 670139.000 | 670139.000 | bytes |
| viberwhisper | history | step_053 | current | searchable_files | 2 | 88.000 | 88.000 | 88.000 | 88.000 | count |
| viberwhisper | history | step_053 | current | segments | 2 | 24.000 | 24.000 | 24.000 | 24.000 | count |
| viberwhisper | history | step_053 | current | wall | 2 | 9.015 | 9.015 | 9.524 | 9.524 | ms |
| viberwhisper | history | step_053 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_053 | rg | wall | 2 | 6.008 | 6.008 | 6.509 | 6.509 | ms |
| viberwhisper | history | step_054 | current | extracted | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_054 | current | index_size | 2 | 5063569.000 | 5063569.000 | 5063569.000 | 5063569.000 | bytes |
| viberwhisper | history | step_054 | current | peak_rss | 2 | 13598720.000 | 13598720.000 | 13664256.000 | 13664256.000 | bytes |
| viberwhisper | history | step_054 | current | reused | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_054 | current | searchable_bytes | 2 | 680657.000 | 680657.000 | 680657.000 | 680657.000 | bytes |
| viberwhisper | history | step_054 | current | searchable_files | 2 | 92.000 | 92.000 | 92.000 | 92.000 | count |
| viberwhisper | history | step_054 | current | segments | 2 | 24.000 | 24.000 | 24.000 | 24.000 | count |
| viberwhisper | history | step_054 | current | wall | 2 | 9.089 | 9.089 | 9.601 | 9.601 | ms |
| viberwhisper | history | step_054 | rg | peak_rss | 2 | 6414336.000 | 6414336.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_054 | rg | wall | 2 | 4.880 | 4.880 | 4.984 | 4.984 | ms |
| viberwhisper | history | step_055 | current | extracted | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| viberwhisper | history | step_055 | current | index_size | 2 | 5208332.000 | 5208332.000 | 5208332.000 | 5208332.000 | bytes |
| viberwhisper | history | step_055 | current | peak_rss | 2 | 13312000.000 | 13312000.000 | 13320192.000 | 13320192.000 | bytes |
| viberwhisper | history | step_055 | current | reused | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| viberwhisper | history | step_055 | current | searchable_bytes | 2 | 685966.000 | 685966.000 | 685966.000 | 685966.000 | bytes |
| viberwhisper | history | step_055 | current | searchable_files | 2 | 93.000 | 93.000 | 93.000 | 93.000 | count |
| viberwhisper | history | step_055 | current | segments | 2 | 25.000 | 25.000 | 25.000 | 25.000 | count |
| viberwhisper | history | step_055 | current | wall | 2 | 10.015 | 10.015 | 11.342 | 11.342 | ms |
| viberwhisper | history | step_055 | rg | peak_rss | 2 | 6373376.000 | 6373376.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_055 | rg | wall | 2 | 7.376 | 7.376 | 8.812 | 8.812 | ms |
| viberwhisper | history | step_056 | current | extracted | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| viberwhisper | history | step_056 | current | index_size | 2 | 5490958.000 | 5490958.000 | 5490958.000 | 5490958.000 | bytes |
| viberwhisper | history | step_056 | current | peak_rss | 2 | 14909440.000 | 14909440.000 | 14909440.000 | 14909440.000 | bytes |
| viberwhisper | history | step_056 | current | reused | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| viberwhisper | history | step_056 | current | searchable_bytes | 2 | 690035.000 | 690035.000 | 690035.000 | 690035.000 | bytes |
| viberwhisper | history | step_056 | current | searchable_files | 2 | 94.000 | 94.000 | 94.000 | 94.000 | count |
| viberwhisper | history | step_056 | current | segments | 2 | 23.000 | 23.000 | 23.000 | 23.000 | count |
| viberwhisper | history | step_056 | current | wall | 2 | 11.139 | 11.139 | 12.234 | 12.234 | ms |
| viberwhisper | history | step_056 | rg | peak_rss | 2 | 6430720.000 | 6430720.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | history | step_056 | rg | wall | 2 | 7.263 | 7.263 | 9.307 | 9.307 | ms |
| viberwhisper | history | step_057 | current | extracted | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| viberwhisper | history | step_057 | current | index_size | 2 | 5656187.000 | 5656187.000 | 5656187.000 | 5656187.000 | bytes |
| viberwhisper | history | step_057 | current | peak_rss | 2 | 13385728.000 | 13385728.000 | 13402112.000 | 13402112.000 | bytes |
| viberwhisper | history | step_057 | current | reused | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| viberwhisper | history | step_057 | current | searchable_bytes | 2 | 703505.000 | 703505.000 | 703505.000 | 703505.000 | bytes |
| viberwhisper | history | step_057 | current | searchable_files | 2 | 96.000 | 96.000 | 96.000 | 96.000 | count |
| viberwhisper | history | step_057 | current | segments | 2 | 24.000 | 24.000 | 24.000 | 24.000 | count |
| viberwhisper | history | step_057 | current | wall | 2 | 8.532 | 8.532 | 8.665 | 8.665 | ms |
| viberwhisper | history | step_057 | rg | peak_rss | 2 | 6406144.000 | 6406144.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_057 | rg | wall | 2 | 8.437 | 8.437 | 9.834 | 9.834 | ms |
| viberwhisper | history | step_058 | current | extracted | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_058 | current | index_size | 2 | 6097432.000 | 6097432.000 | 6097432.000 | 6097432.000 | bytes |
| viberwhisper | history | step_058 | current | peak_rss | 2 | 15187968.000 | 15187968.000 | 15319040.000 | 15319040.000 | bytes |
| viberwhisper | history | step_058 | current | reused | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_058 | current | searchable_bytes | 2 | 849768.000 | 849768.000 | 849768.000 | 849768.000 | bytes |
| viberwhisper | history | step_058 | current | searchable_files | 2 | 102.000 | 102.000 | 102.000 | 102.000 | count |
| viberwhisper | history | step_058 | current | segments | 2 | 25.000 | 25.000 | 25.000 | 25.000 | count |
| viberwhisper | history | step_058 | current | wall | 2 | 12.455 | 12.455 | 13.154 | 13.154 | ms |
| viberwhisper | history | step_058 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_058 | rg | wall | 2 | 5.975 | 5.975 | 6.687 | 6.687 | ms |
| viberwhisper | history | step_059 | current | extracted | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_059 | current | index_size | 2 | 6434258.000 | 6434258.000 | 6434258.000 | 6434258.000 | bytes |
| viberwhisper | history | step_059 | current | peak_rss | 2 | 15351808.000 | 15351808.000 | 15417344.000 | 15417344.000 | bytes |
| viberwhisper | history | step_059 | current | reused | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_059 | current | searchable_bytes | 2 | 891006.000 | 891006.000 | 891006.000 | 891006.000 | bytes |
| viberwhisper | history | step_059 | current | searchable_files | 2 | 103.000 | 103.000 | 103.000 | 103.000 | count |
| viberwhisper | history | step_059 | current | segments | 2 | 26.000 | 26.000 | 26.000 | 26.000 | count |
| viberwhisper | history | step_059 | current | wall | 2 | 11.639 | 11.639 | 11.835 | 11.835 | ms |
| viberwhisper | history | step_059 | rg | peak_rss | 2 | 6406144.000 | 6406144.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_059 | rg | wall | 2 | 6.681 | 6.681 | 7.585 | 7.585 | ms |
| viberwhisper | history | step_060 | current | extracted | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_060 | current | index_size | 2 | 6970842.000 | 6970842.000 | 6970842.000 | 6970842.000 | bytes |
| viberwhisper | history | step_060 | current | peak_rss | 2 | 15835136.000 | 15835136.000 | 15876096.000 | 15876096.000 | bytes |
| viberwhisper | history | step_060 | current | reused | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_060 | current | searchable_bytes | 2 | 945808.000 | 945808.000 | 945808.000 | 945808.000 | bytes |
| viberwhisper | history | step_060 | current | searchable_files | 2 | 105.000 | 105.000 | 105.000 | 105.000 | count |
| viberwhisper | history | step_060 | current | segments | 2 | 27.000 | 27.000 | 27.000 | 27.000 | count |
| viberwhisper | history | step_060 | current | wall | 2 | 14.235 | 14.235 | 14.727 | 14.727 | ms |
| viberwhisper | history | step_060 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_060 | rg | wall | 2 | 6.580 | 6.580 | 7.809 | 7.809 | ms |
| viberwhisper | history | step_061 | current | extracted | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_061 | current | index_size | 2 | 7470621.000 | 7470621.000 | 7470621.000 | 7470621.000 | bytes |
| viberwhisper | history | step_061 | current | peak_rss | 2 | 15581184.000 | 15581184.000 | 15663104.000 | 15663104.000 | bytes |
| viberwhisper | history | step_061 | current | reused | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_061 | current | searchable_bytes | 2 | 996330.000 | 996330.000 | 996330.000 | 996330.000 | bytes |
| viberwhisper | history | step_061 | current | searchable_files | 2 | 109.000 | 109.000 | 109.000 | 109.000 | count |
| viberwhisper | history | step_061 | current | segments | 2 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | history | step_061 | current | wall | 2 | 13.206 | 13.206 | 13.699 | 13.699 | ms |
| viberwhisper | history | step_061 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_061 | rg | wall | 2 | 9.006 | 9.006 | 9.575 | 9.575 | ms |
| viberwhisper | history | step_062 | current | extracted | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_062 | current | index_size | 2 | 7834216.000 | 7834216.000 | 7834216.000 | 7834216.000 | bytes |
| viberwhisper | history | step_062 | current | peak_rss | 2 | 14753792.000 | 14753792.000 | 14958592.000 | 14958592.000 | bytes |
| viberwhisper | history | step_062 | current | reused | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_062 | current | searchable_bytes | 2 | 1037105.000 | 1037105.000 | 1037105.000 | 1037105.000 | bytes |
| viberwhisper | history | step_062 | current | searchable_files | 2 | 113.000 | 113.000 | 113.000 | 113.000 | count |
| viberwhisper | history | step_062 | current | segments | 2 | 29.000 | 29.000 | 29.000 | 29.000 | count |
| viberwhisper | history | step_062 | current | wall | 2 | 11.925 | 11.925 | 12.466 | 12.466 | ms |
| viberwhisper | history | step_062 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_062 | rg | wall | 2 | 6.716 | 6.716 | 7.614 | 7.614 | ms |
| viberwhisper | history | step_063 | current | extracted | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| viberwhisper | history | step_063 | current | index_size | 2 | 7713661.000 | 7713661.000 | 7713661.000 | 7713661.000 | bytes |
| viberwhisper | history | step_063 | current | peak_rss | 2 | 15106048.000 | 15106048.000 | 15122432.000 | 15122432.000 | bytes |
| viberwhisper | history | step_063 | current | reused | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| viberwhisper | history | step_063 | current | searchable_bytes | 2 | 1064529.000 | 1064529.000 | 1064529.000 | 1064529.000 | bytes |
| viberwhisper | history | step_063 | current | searchable_files | 2 | 115.000 | 115.000 | 115.000 | 115.000 | count |
| viberwhisper | history | step_063 | current | segments | 2 | 30.000 | 30.000 | 30.000 | 30.000 | count |
| viberwhisper | history | step_063 | current | wall | 2 | 12.633 | 12.633 | 12.660 | 12.660 | ms |
| viberwhisper | history | step_063 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_063 | rg | wall | 2 | 7.967 | 7.967 | 8.639 | 8.639 | ms |
| viberwhisper | history | step_064 | current | extracted | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| viberwhisper | history | step_064 | current | index_size | 2 | 8158615.000 | 8158615.000 | 8158615.000 | 8158615.000 | bytes |
| viberwhisper | history | step_064 | current | peak_rss | 2 | 15376384.000 | 15376384.000 | 15400960.000 | 15400960.000 | bytes |
| viberwhisper | history | step_064 | current | reused | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| viberwhisper | history | step_064 | current | searchable_bytes | 2 | 1113117.000 | 1113117.000 | 1113117.000 | 1113117.000 | bytes |
| viberwhisper | history | step_064 | current | searchable_files | 2 | 118.000 | 118.000 | 118.000 | 118.000 | count |
| viberwhisper | history | step_064 | current | segments | 2 | 31.000 | 31.000 | 31.000 | 31.000 | count |
| viberwhisper | history | step_064 | current | wall | 2 | 12.876 | 12.876 | 13.917 | 13.917 | ms |
| viberwhisper | history | step_064 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_064 | rg | wall | 2 | 8.431 | 8.431 | 10.666 | 10.666 | ms |
| viberwhisper | history | step_065 | current | extracted | 2 | 25.000 | 25.000 | 25.000 | 25.000 | count |
| viberwhisper | history | step_065 | current | index_size | 2 | 8942320.000 | 8942320.000 | 8942320.000 | 8942320.000 | bytes |
| viberwhisper | history | step_065 | current | peak_rss | 2 | 18317312.000 | 18317312.000 | 18792448.000 | 18792448.000 | bytes |
| viberwhisper | history | step_065 | current | reused | 2 | 25.000 | 25.000 | 25.000 | 25.000 | count |
| viberwhisper | history | step_065 | current | searchable_bytes | 2 | 1322983.000 | 1322983.000 | 1322983.000 | 1322983.000 | bytes |
| viberwhisper | history | step_065 | current | searchable_files | 2 | 127.000 | 127.000 | 127.000 | 127.000 | count |
| viberwhisper | history | step_065 | current | segments | 2 | 32.000 | 32.000 | 32.000 | 32.000 | count |
| viberwhisper | history | step_065 | current | wall | 2 | 16.059 | 16.059 | 16.465 | 16.465 | ms |
| viberwhisper | history | step_065 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_065 | rg | wall | 2 | 6.473 | 6.473 | 7.180 | 7.180 | ms |
| viberwhisper | history | step_066 | current | extracted | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_066 | current | index_size | 2 | 9222840.000 | 9222840.000 | 9222840.000 | 9222840.000 | bytes |
| viberwhisper | history | step_066 | current | peak_rss | 2 | 15802368.000 | 15802368.000 | 15974400.000 | 15974400.000 | bytes |
| viberwhisper | history | step_066 | current | reused | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_066 | current | searchable_bytes | 2 | 1342969.000 | 1342969.000 | 1342969.000 | 1342969.000 | bytes |
| viberwhisper | history | step_066 | current | searchable_files | 2 | 128.000 | 128.000 | 128.000 | 128.000 | count |
| viberwhisper | history | step_066 | current | segments | 2 | 33.000 | 33.000 | 33.000 | 33.000 | count |
| viberwhisper | history | step_066 | current | wall | 2 | 13.676 | 13.676 | 14.083 | 14.083 | ms |
| viberwhisper | history | step_066 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_066 | rg | wall | 2 | 6.069 | 6.069 | 6.741 | 6.741 | ms |
| viberwhisper | history | step_067 | current | extracted | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| viberwhisper | history | step_067 | current | index_size | 2 | 11440646.000 | 11440646.000 | 11440646.000 | 11440646.000 | bytes |
| viberwhisper | history | step_067 | current | peak_rss | 2 | 22986752.000 | 22986752.000 | 23003136.000 | 23003136.000 | bytes |
| viberwhisper | history | step_067 | current | reused | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| viberwhisper | history | step_067 | current | searchable_bytes | 2 | 1365941.000 | 1365941.000 | 1365941.000 | 1365941.000 | bytes |
| viberwhisper | history | step_067 | current | searchable_files | 2 | 130.000 | 130.000 | 130.000 | 130.000 | count |
| viberwhisper | history | step_067 | current | segments | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_067 | current | wall | 2 | 101.433 | 101.433 | 102.555 | 102.555 | ms |
| viberwhisper | history | step_067 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | history | step_067 | rg | wall | 2 | 6.459 | 6.459 | 7.516 | 7.516 | ms |
| viberwhisper | history | step_068 | current | extracted | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_068 | current | index_size | 2 | 11541588.000 | 11541588.000 | 11541588.000 | 11541588.000 | bytes |
| viberwhisper | history | step_068 | current | peak_rss | 2 | 12009472.000 | 12009472.000 | 12075008.000 | 12075008.000 | bytes |
| viberwhisper | history | step_068 | current | reused | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_068 | current | searchable_bytes | 2 | 1366423.000 | 1366423.000 | 1366423.000 | 1366423.000 | bytes |
| viberwhisper | history | step_068 | current | searchable_files | 2 | 130.000 | 130.000 | 130.000 | 130.000 | count |
| viberwhisper | history | step_068 | current | segments | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_068 | current | wall | 2 | 9.549 | 9.549 | 9.780 | 9.780 | ms |
| viberwhisper | history | step_068 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6356992.000 | 6356992.000 | bytes |
| viberwhisper | history | step_068 | rg | wall | 2 | 8.287 | 8.287 | 10.515 | 10.515 | ms |
| viberwhisper | history | step_069 | current | extracted | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_069 | current | index_size | 2 | 11578723.000 | 11578723.000 | 11578723.000 | 11578723.000 | bytes |
| viberwhisper | history | step_069 | current | peak_rss | 2 | 11370496.000 | 11370496.000 | 11419648.000 | 11419648.000 | bytes |
| viberwhisper | history | step_069 | current | reused | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_069 | current | searchable_bytes | 2 | 1366539.000 | 1366539.000 | 1366539.000 | 1366539.000 | bytes |
| viberwhisper | history | step_069 | current | searchable_files | 2 | 130.000 | 130.000 | 130.000 | 130.000 | count |
| viberwhisper | history | step_069 | current | segments | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_069 | current | wall | 2 | 7.878 | 7.878 | 8.611 | 8.611 | ms |
| viberwhisper | history | step_069 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_069 | rg | wall | 2 | 7.915 | 7.915 | 8.809 | 8.809 | ms |
| viberwhisper | history | step_070 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_070 | current | index_size | 2 | 11657818.000 | 11657818.000 | 11657818.000 | 11657818.000 | bytes |
| viberwhisper | history | step_070 | current | peak_rss | 2 | 12050432.000 | 12050432.000 | 12075008.000 | 12075008.000 | bytes |
| viberwhisper | history | step_070 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_070 | current | searchable_bytes | 2 | 1372803.000 | 1372803.000 | 1372803.000 | 1372803.000 | bytes |
| viberwhisper | history | step_070 | current | searchable_files | 2 | 131.000 | 131.000 | 131.000 | 131.000 | count |
| viberwhisper | history | step_070 | current | segments | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_070 | current | wall | 2 | 8.012 | 8.012 | 8.409 | 8.409 | ms |
| viberwhisper | history | step_070 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_070 | rg | wall | 2 | 10.319 | 10.319 | 11.114 | 11.114 | ms |
| viberwhisper | history | step_071 | current | extracted | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| viberwhisper | history | step_071 | current | index_size | 2 | 11831411.000 | 11831411.000 | 11831411.000 | 11831411.000 | bytes |
| viberwhisper | history | step_071 | current | peak_rss | 2 | 12599296.000 | 12599296.000 | 12599296.000 | 12599296.000 | bytes |
| viberwhisper | history | step_071 | current | reused | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| viberwhisper | history | step_071 | current | searchable_bytes | 2 | 1387578.000 | 1387578.000 | 1387578.000 | 1387578.000 | bytes |
| viberwhisper | history | step_071 | current | searchable_files | 2 | 134.000 | 134.000 | 134.000 | 134.000 | count |
| viberwhisper | history | step_071 | current | segments | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_071 | current | wall | 2 | 9.360 | 9.360 | 10.038 | 10.038 | ms |
| viberwhisper | history | step_071 | rg | peak_rss | 2 | 6316032.000 | 6316032.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_071 | rg | wall | 2 | 8.132 | 8.132 | 8.358 | 8.358 | ms |
| viberwhisper | history | step_072 | current | extracted | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_072 | current | index_size | 2 | 11978310.000 | 11978310.000 | 11978310.000 | 11978310.000 | bytes |
| viberwhisper | history | step_072 | current | peak_rss | 2 | 12541952.000 | 12541952.000 | 12566528.000 | 12566528.000 | bytes |
| viberwhisper | history | step_072 | current | reused | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_072 | current | searchable_bytes | 2 | 1396517.000 | 1396517.000 | 1396517.000 | 1396517.000 | bytes |
| viberwhisper | history | step_072 | current | searchable_files | 2 | 135.000 | 135.000 | 135.000 | 135.000 | count |
| viberwhisper | history | step_072 | current | segments | 2 | 5.000 | 5.000 | 5.000 | 5.000 | count |
| viberwhisper | history | step_072 | current | wall | 2 | 8.162 | 8.162 | 8.406 | 8.406 | ms |
| viberwhisper | history | step_072 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | history | step_072 | rg | wall | 2 | 8.013 | 8.013 | 9.706 | 9.706 | ms |
| viberwhisper | history | step_073 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_073 | current | index_size | 2 | 12127378.000 | 12127378.000 | 12127378.000 | 12127378.000 | bytes |
| viberwhisper | history | step_073 | current | peak_rss | 2 | 12541952.000 | 12541952.000 | 12550144.000 | 12550144.000 | bytes |
| viberwhisper | history | step_073 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_073 | current | searchable_bytes | 2 | 1405971.000 | 1405971.000 | 1405971.000 | 1405971.000 | bytes |
| viberwhisper | history | step_073 | current | searchable_files | 2 | 137.000 | 137.000 | 137.000 | 137.000 | count |
| viberwhisper | history | step_073 | current | segments | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_073 | current | wall | 2 | 9.017 | 9.017 | 9.465 | 9.465 | ms |
| viberwhisper | history | step_073 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_073 | rg | wall | 2 | 6.509 | 6.509 | 7.019 | 7.019 | ms |
| viberwhisper | history | step_074 | current | extracted | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_074 | current | index_size | 2 | 3449366.000 | 3449366.000 | 3449366.000 | 3449366.000 | bytes |
| viberwhisper | history | step_074 | current | peak_rss | 2 | 14385152.000 | 14385152.000 | 14401536.000 | 14401536.000 | bytes |
| viberwhisper | history | step_074 | current | reused | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_074 | current | searchable_bytes | 2 | 1426350.000 | 1426350.000 | 1426350.000 | 1426350.000 | bytes |
| viberwhisper | history | step_074 | current | searchable_files | 2 | 139.000 | 139.000 | 139.000 | 139.000 | count |
| viberwhisper | history | step_074 | current | segments | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_074 | current | wall | 2 | 13.075 | 13.075 | 13.219 | 13.219 | ms |
| viberwhisper | history | step_074 | rg | peak_rss | 2 | 6422528.000 | 6422528.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_074 | rg | wall | 2 | 9.162 | 9.162 | 9.347 | 9.347 | ms |
| viberwhisper | history | step_075 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_075 | current | index_size | 2 | 3627547.000 | 3627547.000 | 3627547.000 | 3627547.000 | bytes |
| viberwhisper | history | step_075 | current | peak_rss | 2 | 13148160.000 | 13148160.000 | 13189120.000 | 13189120.000 | bytes |
| viberwhisper | history | step_075 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_075 | current | searchable_bytes | 2 | 1435127.000 | 1435127.000 | 1435127.000 | 1435127.000 | bytes |
| viberwhisper | history | step_075 | current | searchable_files | 2 | 140.000 | 140.000 | 140.000 | 140.000 | count |
| viberwhisper | history | step_075 | current | segments | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_075 | current | wall | 2 | 9.213 | 9.213 | 9.655 | 9.655 | ms |
| viberwhisper | history | step_075 | rg | peak_rss | 2 | 6422528.000 | 6422528.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_075 | rg | wall | 2 | 6.285 | 6.285 | 7.025 | 7.025 | ms |
| viberwhisper | history | step_076 | current | extracted | 2 | 23.000 | 23.000 | 23.000 | 23.000 | count |
| viberwhisper | history | step_076 | current | index_size | 2 | 4317788.000 | 4317788.000 | 4317788.000 | 4317788.000 | bytes |
| viberwhisper | history | step_076 | current | peak_rss | 2 | 15794176.000 | 15794176.000 | 15908864.000 | 15908864.000 | bytes |
| viberwhisper | history | step_076 | current | reused | 2 | 23.000 | 23.000 | 23.000 | 23.000 | count |
| viberwhisper | history | step_076 | current | searchable_bytes | 2 | 1503575.000 | 1503575.000 | 1503575.000 | 1503575.000 | bytes |
| viberwhisper | history | step_076 | current | searchable_files | 2 | 143.000 | 143.000 | 143.000 | 143.000 | count |
| viberwhisper | history | step_076 | current | segments | 2 | 9.000 | 9.000 | 9.000 | 9.000 | count |
| viberwhisper | history | step_076 | current | wall | 2 | 15.705 | 15.705 | 16.264 | 16.264 | ms |
| viberwhisper | history | step_076 | rg | peak_rss | 2 | 6389760.000 | 6389760.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_076 | rg | wall | 2 | 8.145 | 8.145 | 9.091 | 9.091 | ms |
| viberwhisper | history | step_077 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_077 | current | index_size | 2 | 4476069.000 | 4476069.000 | 4476069.000 | 4476069.000 | bytes |
| viberwhisper | history | step_077 | current | peak_rss | 2 | 13426688.000 | 13426688.000 | 13451264.000 | 13451264.000 | bytes |
| viberwhisper | history | step_077 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_077 | current | searchable_bytes | 2 | 1508754.000 | 1508754.000 | 1508754.000 | 1508754.000 | bytes |
| viberwhisper | history | step_077 | current | searchable_files | 2 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | history | step_077 | current | segments | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_077 | current | wall | 2 | 8.984 | 8.984 | 9.003 | 9.003 | ms |
| viberwhisper | history | step_077 | rg | peak_rss | 2 | 6414336.000 | 6414336.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_077 | rg | wall | 2 | 7.871 | 7.871 | 7.931 | 7.931 | ms |
| viberwhisper | history | step_078 | current | extracted | 2 | 23.000 | 23.000 | 23.000 | 23.000 | count |
| viberwhisper | history | step_078 | current | index_size | 2 | 4981392.000 | 4981392.000 | 4981392.000 | 4981392.000 | bytes |
| viberwhisper | history | step_078 | current | peak_rss | 2 | 15900672.000 | 15900672.000 | 16220160.000 | 16220160.000 | bytes |
| viberwhisper | history | step_078 | current | reused | 2 | 23.000 | 23.000 | 23.000 | 23.000 | count |
| viberwhisper | history | step_078 | current | searchable_bytes | 2 | 1513982.000 | 1513982.000 | 1513982.000 | 1513982.000 | bytes |
| viberwhisper | history | step_078 | current | searchable_files | 2 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | history | step_078 | current | segments | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| viberwhisper | history | step_078 | current | wall | 2 | 14.319 | 14.319 | 15.558 | 15.558 | ms |
| viberwhisper | history | step_078 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_078 | rg | wall | 2 | 8.752 | 8.752 | 9.789 | 9.789 | ms |
| viberwhisper | history | step_079 | current | extracted | 2 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | history | step_079 | current | index_size | 2 | 5520810.000 | 5520810.000 | 5520810.000 | 5520810.000 | bytes |
| viberwhisper | history | step_079 | current | peak_rss | 2 | 15048704.000 | 15048704.000 | 15253504.000 | 15253504.000 | bytes |
| viberwhisper | history | step_079 | current | reused | 2 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | history | step_079 | current | searchable_bytes | 2 | 1364289.000 | 1364289.000 | 1364289.000 | 1364289.000 | bytes |
| viberwhisper | history | step_079 | current | searchable_files | 2 | 123.000 | 123.000 | 123.000 | 123.000 | count |
| viberwhisper | history | step_079 | current | segments | 2 | 12.000 | 12.000 | 12.000 | 12.000 | count |
| viberwhisper | history | step_079 | current | wall | 2 | 13.627 | 13.627 | 15.143 | 15.143 | ms |
| viberwhisper | history | step_079 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_079 | rg | wall | 2 | 6.763 | 6.763 | 7.329 | 7.329 | ms |
| viberwhisper | history | step_080 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_080 | current | index_size | 2 | 5562386.000 | 5562386.000 | 5562386.000 | 5562386.000 | bytes |
| viberwhisper | history | step_080 | current | peak_rss | 2 | 11812864.000 | 11812864.000 | 11845632.000 | 11845632.000 | bytes |
| viberwhisper | history | step_080 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_080 | current | searchable_bytes | 2 | 1365002.000 | 1365002.000 | 1365002.000 | 1365002.000 | bytes |
| viberwhisper | history | step_080 | current | searchable_files | 2 | 123.000 | 123.000 | 123.000 | 123.000 | count |
| viberwhisper | history | step_080 | current | segments | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| viberwhisper | history | step_080 | current | wall | 2 | 8.347 | 8.347 | 9.299 | 9.299 | ms |
| viberwhisper | history | step_080 | rg | peak_rss | 2 | 6299648.000 | 6299648.000 | 6307840.000 | 6307840.000 | bytes |
| viberwhisper | history | step_080 | rg | wall | 2 | 6.928 | 6.928 | 7.207 | 7.207 | ms |
| viberwhisper | history | step_081 | current | extracted | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_081 | current | index_size | 2 | 5715684.000 | 5715684.000 | 5715684.000 | 5715684.000 | bytes |
| viberwhisper | history | step_081 | current | peak_rss | 2 | 13164544.000 | 13164544.000 | 13238272.000 | 13238272.000 | bytes |
| viberwhisper | history | step_081 | current | reused | 2 | 4.000 | 4.000 | 4.000 | 4.000 | count |
| viberwhisper | history | step_081 | current | searchable_bytes | 2 | 1367475.000 | 1367475.000 | 1367475.000 | 1367475.000 | bytes |
| viberwhisper | history | step_081 | current | searchable_files | 2 | 124.000 | 124.000 | 124.000 | 124.000 | count |
| viberwhisper | history | step_081 | current | segments | 2 | 13.000 | 13.000 | 13.000 | 13.000 | count |
| viberwhisper | history | step_081 | current | wall | 2 | 10.912 | 10.912 | 13.312 | 13.312 | ms |
| viberwhisper | history | step_081 | rg | peak_rss | 2 | 6397952.000 | 6397952.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_081 | rg | wall | 2 | 7.028 | 7.028 | 8.639 | 8.639 | ms |
| viberwhisper | history | step_082 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_082 | current | index_size | 2 | 5867598.000 | 5867598.000 | 5867598.000 | 5867598.000 | bytes |
| viberwhisper | history | step_082 | current | peak_rss | 2 | 13475840.000 | 13475840.000 | 13500416.000 | 13500416.000 | bytes |
| viberwhisper | history | step_082 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_082 | current | searchable_bytes | 2 | 1367419.000 | 1367419.000 | 1367419.000 | 1367419.000 | bytes |
| viberwhisper | history | step_082 | current | searchable_files | 2 | 124.000 | 124.000 | 124.000 | 124.000 | count |
| viberwhisper | history | step_082 | current | segments | 2 | 14.000 | 14.000 | 14.000 | 14.000 | count |
| viberwhisper | history | step_082 | current | wall | 2 | 8.966 | 8.966 | 9.166 | 9.166 | ms |
| viberwhisper | history | step_082 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_082 | rg | wall | 2 | 6.777 | 6.777 | 7.450 | 7.450 | ms |
| viberwhisper | history | step_083 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_083 | current | index_size | 2 | 6086197.000 | 6086197.000 | 6086197.000 | 6086197.000 | bytes |
| viberwhisper | history | step_083 | current | peak_rss | 2 | 13869056.000 | 13869056.000 | 13959168.000 | 13959168.000 | bytes |
| viberwhisper | history | step_083 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_083 | current | searchable_bytes | 2 | 1381607.000 | 1381607.000 | 1381607.000 | 1381607.000 | bytes |
| viberwhisper | history | step_083 | current | searchable_files | 2 | 125.000 | 125.000 | 125.000 | 125.000 | count |
| viberwhisper | history | step_083 | current | segments | 2 | 15.000 | 15.000 | 15.000 | 15.000 | count |
| viberwhisper | history | step_083 | current | wall | 2 | 9.755 | 9.755 | 10.021 | 10.021 | ms |
| viberwhisper | history | step_083 | rg | peak_rss | 2 | 6373376.000 | 6373376.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_083 | rg | wall | 2 | 5.946 | 5.946 | 6.095 | 6.095 | ms |
| viberwhisper | history | step_084 | current | extracted | 2 | 30.000 | 30.000 | 30.000 | 30.000 | count |
| viberwhisper | history | step_084 | current | index_size | 2 | 6632531.000 | 6632531.000 | 6632531.000 | 6632531.000 | bytes |
| viberwhisper | history | step_084 | current | peak_rss | 2 | 15130624.000 | 15130624.000 | 15171584.000 | 15171584.000 | bytes |
| viberwhisper | history | step_084 | current | reused | 2 | 30.000 | 30.000 | 30.000 | 30.000 | count |
| viberwhisper | history | step_084 | current | searchable_bytes | 2 | 1395718.000 | 1395718.000 | 1395718.000 | 1395718.000 | bytes |
| viberwhisper | history | step_084 | current | searchable_files | 2 | 125.000 | 125.000 | 125.000 | 125.000 | count |
| viberwhisper | history | step_084 | current | segments | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_084 | current | wall | 2 | 14.031 | 14.031 | 14.404 | 14.404 | ms |
| viberwhisper | history | step_084 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_084 | rg | wall | 2 | 8.165 | 8.165 | 8.710 | 8.710 | ms |
| viberwhisper | history | step_085 | current | extracted | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_085 | current | index_size | 2 | 6961348.000 | 6961348.000 | 6961348.000 | 6961348.000 | bytes |
| viberwhisper | history | step_085 | current | peak_rss | 2 | 16056320.000 | 16056320.000 | 16089088.000 | 16089088.000 | bytes |
| viberwhisper | history | step_085 | current | reused | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_085 | current | searchable_bytes | 2 | 1401792.000 | 1401792.000 | 1401792.000 | 1401792.000 | bytes |
| viberwhisper | history | step_085 | current | searchable_files | 2 | 126.000 | 126.000 | 126.000 | 126.000 | count |
| viberwhisper | history | step_085 | current | segments | 2 | 16.000 | 16.000 | 16.000 | 16.000 | count |
| viberwhisper | history | step_085 | current | wall | 2 | 10.457 | 10.457 | 10.467 | 10.467 | ms |
| viberwhisper | history | step_085 | rg | peak_rss | 2 | 6438912.000 | 6438912.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | history | step_085 | rg | wall | 2 | 7.499 | 7.499 | 8.065 | 8.065 | ms |
| viberwhisper | history | step_086 | current | extracted | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_086 | current | index_size | 2 | 7199650.000 | 7199650.000 | 7199650.000 | 7199650.000 | bytes |
| viberwhisper | history | step_086 | current | peak_rss | 2 | 14467072.000 | 14467072.000 | 14516224.000 | 14516224.000 | bytes |
| viberwhisper | history | step_086 | current | reused | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_086 | current | searchable_bytes | 2 | 1404791.000 | 1404791.000 | 1404791.000 | 1404791.000 | bytes |
| viberwhisper | history | step_086 | current | searchable_files | 2 | 126.000 | 126.000 | 126.000 | 126.000 | count |
| viberwhisper | history | step_086 | current | segments | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_086 | current | wall | 2 | 9.719 | 9.719 | 9.887 | 9.887 | ms |
| viberwhisper | history | step_086 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_086 | rg | wall | 2 | 6.544 | 6.544 | 6.913 | 6.913 | ms |
| viberwhisper | history | step_087 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_087 | current | index_size | 2 | 7245603.000 | 7245603.000 | 7245603.000 | 7245603.000 | bytes |
| viberwhisper | history | step_087 | current | peak_rss | 2 | 12312576.000 | 12312576.000 | 12369920.000 | 12369920.000 | bytes |
| viberwhisper | history | step_087 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_087 | current | searchable_bytes | 2 | 1404120.000 | 1404120.000 | 1404120.000 | 1404120.000 | bytes |
| viberwhisper | history | step_087 | current | searchable_files | 2 | 126.000 | 126.000 | 126.000 | 126.000 | count |
| viberwhisper | history | step_087 | current | segments | 2 | 18.000 | 18.000 | 18.000 | 18.000 | count |
| viberwhisper | history | step_087 | current | wall | 2 | 8.478 | 8.478 | 9.350 | 9.350 | ms |
| viberwhisper | history | step_087 | rg | peak_rss | 2 | 6414336.000 | 6414336.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_087 | rg | wall | 2 | 7.633 | 7.633 | 9.768 | 9.768 | ms |
| viberwhisper | history | step_088 | current | extracted | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_088 | current | index_size | 2 | 7418199.000 | 7418199.000 | 7418199.000 | 7418199.000 | bytes |
| viberwhisper | history | step_088 | current | peak_rss | 2 | 14065664.000 | 14065664.000 | 14204928.000 | 14204928.000 | bytes |
| viberwhisper | history | step_088 | current | reused | 2 | 7.000 | 7.000 | 7.000 | 7.000 | count |
| viberwhisper | history | step_088 | current | searchable_bytes | 2 | 1411705.000 | 1411705.000 | 1411705.000 | 1411705.000 | bytes |
| viberwhisper | history | step_088 | current | searchable_files | 2 | 127.000 | 127.000 | 127.000 | 127.000 | count |
| viberwhisper | history | step_088 | current | segments | 2 | 19.000 | 19.000 | 19.000 | 19.000 | count |
| viberwhisper | history | step_088 | current | wall | 2 | 10.095 | 10.095 | 11.233 | 11.233 | ms |
| viberwhisper | history | step_088 | rg | peak_rss | 2 | 6348800.000 | 6348800.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_088 | rg | wall | 2 | 6.775 | 6.775 | 6.805 | 6.805 | ms |
| viberwhisper | history | step_089 | current | extracted | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_089 | current | index_size | 2 | 7660634.000 | 7660634.000 | 7660634.000 | 7660634.000 | bytes |
| viberwhisper | history | step_089 | current | peak_rss | 2 | 14753792.000 | 14753792.000 | 14843904.000 | 14843904.000 | bytes |
| viberwhisper | history | step_089 | current | reused | 2 | 8.000 | 8.000 | 8.000 | 8.000 | count |
| viberwhisper | history | step_089 | current | searchable_bytes | 2 | 1418470.000 | 1418470.000 | 1418470.000 | 1418470.000 | bytes |
| viberwhisper | history | step_089 | current | searchable_files | 2 | 128.000 | 128.000 | 128.000 | 128.000 | count |
| viberwhisper | history | step_089 | current | segments | 2 | 20.000 | 20.000 | 20.000 | 20.000 | count |
| viberwhisper | history | step_089 | current | wall | 2 | 9.855 | 9.855 | 9.989 | 9.989 | ms |
| viberwhisper | history | step_089 | rg | peak_rss | 2 | 6389760.000 | 6389760.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_089 | rg | wall | 2 | 6.022 | 6.022 | 6.191 | 6.191 | ms |
| viberwhisper | history | step_090 | current | extracted | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_090 | current | index_size | 2 | 7909455.000 | 7909455.000 | 7909455.000 | 7909455.000 | bytes |
| viberwhisper | history | step_090 | current | peak_rss | 2 | 14753792.000 | 14753792.000 | 14811136.000 | 14811136.000 | bytes |
| viberwhisper | history | step_090 | current | reused | 2 | 10.000 | 10.000 | 10.000 | 10.000 | count |
| viberwhisper | history | step_090 | current | searchable_bytes | 2 | 1418586.000 | 1418586.000 | 1418586.000 | 1418586.000 | bytes |
| viberwhisper | history | step_090 | current | searchable_files | 2 | 129.000 | 129.000 | 129.000 | 129.000 | count |
| viberwhisper | history | step_090 | current | segments | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| viberwhisper | history | step_090 | current | wall | 2 | 10.932 | 10.932 | 11.923 | 11.923 | ms |
| viberwhisper | history | step_090 | rg | peak_rss | 2 | 6381568.000 | 6381568.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_090 | rg | wall | 2 | 9.791 | 9.791 | 11.218 | 11.218 | ms |
| viberwhisper | history | step_091 | current | extracted | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_091 | current | index_size | 2 | 8022416.000 | 8022416.000 | 8022416.000 | 8022416.000 | bytes |
| viberwhisper | history | step_091 | current | peak_rss | 2 | 13180928.000 | 13180928.000 | 13287424.000 | 13287424.000 | bytes |
| viberwhisper | history | step_091 | current | reused | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_091 | current | searchable_bytes | 2 | 1418646.000 | 1418646.000 | 1418646.000 | 1418646.000 | bytes |
| viberwhisper | history | step_091 | current | searchable_files | 2 | 129.000 | 129.000 | 129.000 | 129.000 | count |
| viberwhisper | history | step_091 | current | segments | 2 | 22.000 | 22.000 | 22.000 | 22.000 | count |
| viberwhisper | history | step_091 | current | wall | 2 | 9.786 | 9.786 | 10.982 | 10.982 | ms |
| viberwhisper | history | step_091 | rg | peak_rss | 2 | 6414336.000 | 6414336.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_091 | rg | wall | 2 | 7.065 | 7.065 | 8.002 | 8.002 | ms |
| viberwhisper | history | step_092 | current | extracted | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_092 | current | index_size | 2 | 8272069.000 | 8272069.000 | 8272069.000 | 8272069.000 | bytes |
| viberwhisper | history | step_092 | current | peak_rss | 2 | 14974976.000 | 14974976.000 | 15106048.000 | 15106048.000 | bytes |
| viberwhisper | history | step_092 | current | reused | 2 | 17.000 | 17.000 | 17.000 | 17.000 | count |
| viberwhisper | history | step_092 | current | searchable_bytes | 2 | 1430534.000 | 1430534.000 | 1430534.000 | 1430534.000 | bytes |
| viberwhisper | history | step_092 | current | searchable_files | 2 | 132.000 | 132.000 | 132.000 | 132.000 | count |
| viberwhisper | history | step_092 | current | segments | 2 | 23.000 | 23.000 | 23.000 | 23.000 | count |
| viberwhisper | history | step_092 | current | wall | 2 | 12.589 | 12.589 | 13.253 | 13.253 | ms |
| viberwhisper | history | step_092 | rg | peak_rss | 2 | 6365184.000 | 6365184.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_092 | rg | wall | 2 | 8.856 | 8.856 | 9.541 | 9.541 | ms |
| viberwhisper | history | step_093 | current | extracted | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_093 | current | index_size | 2 | 8363239.000 | 8363239.000 | 8363239.000 | 8363239.000 | bytes |
| viberwhisper | history | step_093 | current | peak_rss | 2 | 13017088.000 | 13017088.000 | 13058048.000 | 13058048.000 | bytes |
| viberwhisper | history | step_093 | current | reused | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_093 | current | searchable_bytes | 2 | 1429347.000 | 1429347.000 | 1429347.000 | 1429347.000 | bytes |
| viberwhisper | history | step_093 | current | searchable_files | 2 | 132.000 | 132.000 | 132.000 | 132.000 | count |
| viberwhisper | history | step_093 | current | segments | 2 | 24.000 | 24.000 | 24.000 | 24.000 | count |
| viberwhisper | history | step_093 | current | wall | 2 | 8.378 | 8.378 | 8.489 | 8.489 | ms |
| viberwhisper | history | step_093 | rg | peak_rss | 2 | 6356992.000 | 6356992.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | history | step_093 | rg | wall | 2 | 6.549 | 6.549 | 6.733 | 6.733 | ms |
| viberwhisper | history | step_094 | current | extracted | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_094 | current | index_size | 2 | 8638670.000 | 8638670.000 | 8638670.000 | 8638670.000 | bytes |
| viberwhisper | history | step_094 | current | peak_rss | 2 | 13975552.000 | 13975552.000 | 14008320.000 | 14008320.000 | bytes |
| viberwhisper | history | step_094 | current | reused | 2 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | history | step_094 | current | searchable_bytes | 2 | 1429585.000 | 1429585.000 | 1429585.000 | 1429585.000 | bytes |
| viberwhisper | history | step_094 | current | searchable_files | 2 | 132.000 | 132.000 | 132.000 | 132.000 | count |
| viberwhisper | history | step_094 | current | segments | 2 | 25.000 | 25.000 | 25.000 | 25.000 | count |
| viberwhisper | history | step_094 | current | wall | 2 | 11.606 | 11.606 | 12.462 | 12.462 | ms |
| viberwhisper | history | step_094 | rg | peak_rss | 2 | 6373376.000 | 6373376.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | history | step_094 | rg | wall | 2 | 8.035 | 8.035 | 10.620 | 10.620 | ms |
| viberwhisper | history | step_095 | current | extracted | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| viberwhisper | history | step_095 | current | index_size | 2 | 9165359.000 | 9165359.000 | 9165359.000 | 9165359.000 | bytes |
| viberwhisper | history | step_095 | current | peak_rss | 2 | 16392192.000 | 16392192.000 | 16416768.000 | 16416768.000 | bytes |
| viberwhisper | history | step_095 | current | reused | 2 | 11.000 | 11.000 | 11.000 | 11.000 | count |
| viberwhisper | history | step_095 | current | searchable_bytes | 2 | 1480423.000 | 1480423.000 | 1480423.000 | 1480423.000 | bytes |
| viberwhisper | history | step_095 | current | searchable_files | 2 | 136.000 | 136.000 | 136.000 | 136.000 | count |
| viberwhisper | history | step_095 | current | segments | 2 | 25.000 | 25.000 | 25.000 | 25.000 | count |
| viberwhisper | history | step_095 | current | wall | 2 | 15.226 | 15.226 | 17.641 | 17.641 | ms |
| viberwhisper | history | step_095 | rg | peak_rss | 2 | 6406144.000 | 6406144.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | history | step_095 | rg | wall | 2 | 6.183 | 6.183 | 6.953 | 6.953 | ms |
| viberwhisper | history | step_096 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_096 | current | index_size | 2 | 9211001.000 | 9211001.000 | 9211001.000 | 9211001.000 | bytes |
| viberwhisper | history | step_096 | current | peak_rss | 2 | 12640256.000 | 12640256.000 | 12648448.000 | 12648448.000 | bytes |
| viberwhisper | history | step_096 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_096 | current | searchable_bytes | 2 | 1489262.000 | 1489262.000 | 1489262.000 | 1489262.000 | bytes |
| viberwhisper | history | step_096 | current | searchable_files | 2 | 137.000 | 137.000 | 137.000 | 137.000 | count |
| viberwhisper | history | step_096 | current | segments | 2 | 26.000 | 26.000 | 26.000 | 26.000 | count |
| viberwhisper | history | step_096 | current | wall | 2 | 9.031 | 9.031 | 9.402 | 9.402 | ms |
| viberwhisper | history | step_096 | rg | peak_rss | 2 | 6406144.000 | 6406144.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | history | step_096 | rg | wall | 2 | 7.459 | 7.459 | 8.326 | 8.326 | ms |
| viberwhisper | history | step_097 | current | extracted | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| viberwhisper | history | step_097 | current | index_size | 2 | 9941716.000 | 9941716.000 | 9941716.000 | 9941716.000 | bytes |
| viberwhisper | history | step_097 | current | peak_rss | 2 | 17883136.000 | 17883136.000 | 18219008.000 | 18219008.000 | bytes |
| viberwhisper | history | step_097 | current | reused | 2 | 21.000 | 21.000 | 21.000 | 21.000 | count |
| viberwhisper | history | step_097 | current | searchable_bytes | 2 | 1870543.000 | 1870543.000 | 1870543.000 | 1870543.000 | bytes |
| viberwhisper | history | step_097 | current | searchable_files | 2 | 143.000 | 143.000 | 143.000 | 143.000 | count |
| viberwhisper | history | step_097 | current | segments | 2 | 25.000 | 25.000 | 25.000 | 25.000 | count |
| viberwhisper | history | step_097 | current | wall | 2 | 22.020 | 22.020 | 22.097 | 22.097 | ms |
| viberwhisper | history | step_097 | rg | peak_rss | 2 | 6504448.000 | 6504448.000 | 6520832.000 | 6520832.000 | bytes |
| viberwhisper | history | step_097 | rg | wall | 2 | 8.166 | 8.166 | 10.386 | 10.386 | ms |
| viberwhisper | history | step_098 | current | extracted | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_098 | current | index_size | 2 | 10200929.000 | 10200929.000 | 10200929.000 | 10200929.000 | bytes |
| viberwhisper | history | step_098 | current | peak_rss | 2 | 14802944.000 | 14802944.000 | 14811136.000 | 14811136.000 | bytes |
| viberwhisper | history | step_098 | current | reused | 2 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | history | step_098 | current | searchable_bytes | 2 | 1870736.000 | 1870736.000 | 1870736.000 | 1870736.000 | bytes |
| viberwhisper | history | step_098 | current | searchable_files | 2 | 143.000 | 143.000 | 143.000 | 143.000 | count |
| viberwhisper | history | step_098 | current | segments | 2 | 26.000 | 26.000 | 26.000 | 26.000 | count |
| viberwhisper | history | step_098 | current | wall | 2 | 15.461 | 15.461 | 15.840 | 15.840 | ms |
| viberwhisper | history | step_098 | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | history | step_098 | rg | wall | 2 | 8.130 | 8.130 | 10.375 | 10.375 | ms |
| viberwhisper | history | step_099 | current | extracted | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_099 | current | index_size | 2 | 10380118.000 | 10380118.000 | 10380118.000 | 10380118.000 | bytes |
| viberwhisper | history | step_099 | current | peak_rss | 2 | 14270464.000 | 14270464.000 | 14319616.000 | 14319616.000 | bytes |
| viberwhisper | history | step_099 | current | reused | 2 | 6.000 | 6.000 | 6.000 | 6.000 | count |
| viberwhisper | history | step_099 | current | searchable_bytes | 2 | 1874213.000 | 1874213.000 | 1874213.000 | 1874213.000 | bytes |
| viberwhisper | history | step_099 | current | searchable_files | 2 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | history | step_099 | current | segments | 2 | 27.000 | 27.000 | 27.000 | 27.000 | count |
| viberwhisper | history | step_099 | current | wall | 2 | 9.767 | 9.767 | 10.229 | 10.229 | ms |
| viberwhisper | history | step_099 | rg | peak_rss | 2 | 6578176.000 | 6578176.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | history | step_099 | rg | wall | 2 | 8.535 | 8.535 | 10.209 | 10.209 | ms |
| viberwhisper | history | step_100 | current | extracted | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_100 | current | index_size | 2 | 10398187.000 | 10398187.000 | 10398187.000 | 10398187.000 | bytes |
| viberwhisper | history | step_100 | current | peak_rss | 2 | 12009472.000 | 12009472.000 | 12042240.000 | 12042240.000 | bytes |
| viberwhisper | history | step_100 | current | reused | 2 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | history | step_100 | current | searchable_bytes | 2 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | history | step_100 | current | searchable_files | 2 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | history | step_100 | current | segments | 2 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | history | step_100 | current | wall | 2 | 8.187 | 8.187 | 8.388 | 8.388 | ms |
| viberwhisper | history | step_100 | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6537216.000 | 6537216.000 | bytes |
| viberwhisper | history | step_100 | rg | wall | 2 | 6.439 | 6.439 | 7.006 | 7.006 | ms |
| viberwhisper | manifest | search_view | current | load | 16 | 0.013 | 0.014 | 0.021 | 0.021 | ms |
| viberwhisper | search | history_0-absent | current | peak_rss | 4 | 9453568.000 | 9457664.000 | 9469952.000 | 9469952.000 | bytes |
| viberwhisper | search | history_0-absent | current | wall | 32 | 3.915 | 4.053 | 5.007 | 6.276 | ms |
| viberwhisper | search | history_0-absent | rg | peak_rss | 4 | 6356992.000 | 6348800.000 | 6373376.000 | 6373376.000 | bytes |
| viberwhisper | search | history_0-absent | rg | wall | 32 | 4.664 | 4.352 | 5.188 | 6.283 | ms |
| viberwhisper | search | history_0-anchor | current | peak_rss | 4 | 10665984.000 | 10682368.000 | 10747904.000 | 10747904.000 | bytes |
| viberwhisper | search | history_0-anchor | current | wall | 32 | 4.710 | 4.781 | 5.595 | 5.631 | ms |
| viberwhisper | search | history_0-anchor | rg | peak_rss | 4 | 7012352.000 | 7004160.000 | 7094272.000 | 7094272.000 | bytes |
| viberwhisper | search | history_0-anchor | rg | wall | 32 | 5.757 | 5.644 | 7.386 | 7.918 | ms |
| viberwhisper | search | history_0-blank | current | peak_rss | 4 | 9723904.000 | 9715712.000 | 9732096.000 | 9732096.000 | bytes |
| viberwhisper | search | history_0-blank | current | wall | 32 | 4.479 | 4.662 | 5.276 | 5.419 | ms |
| viberwhisper | search | history_0-blank | rg | peak_rss | 4 | 6635520.000 | 6615040.000 | 6668288.000 | 6668288.000 | bytes |
| viberwhisper | search | history_0-blank | rg | wall | 32 | 4.795 | 4.827 | 6.014 | 6.251 | ms |
| viberwhisper | search | history_0-broad | current | peak_rss | 4 | 9535488.000 | 9543680.000 | 9617408.000 | 9617408.000 | bytes |
| viberwhisper | search | history_0-broad | current | wall | 32 | 4.147 | 4.255 | 5.165 | 5.718 | ms |
| viberwhisper | search | history_0-broad | rg | peak_rss | 4 | 6389760.000 | 6369280.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | search | history_0-broad | rg | wall | 32 | 4.641 | 4.571 | 5.820 | 7.241 | ms |
| viberwhisper | search | history_0-count | current | peak_rss | 4 | 9781248.000 | 9781248.000 | 9830400.000 | 9830400.000 | bytes |
| viberwhisper | search | history_0-count | current | wall | 32 | 4.564 | 4.598 | 5.110 | 5.270 | ms |
| viberwhisper | search | history_0-count | rg | peak_rss | 4 | 6668288.000 | 6676480.000 | 6766592.000 | 6766592.000 | bytes |
| viberwhisper | search | history_0-count | rg | wall | 32 | 4.783 | 4.472 | 5.292 | 5.299 | ms |
| viberwhisper | search | history_0-icase | current | peak_rss | 4 | 9723904.000 | 9728000.000 | 9764864.000 | 9764864.000 | bytes |
| viberwhisper | search | history_0-icase | current | wall | 32 | 4.132 | 4.186 | 4.857 | 5.689 | ms |
| viberwhisper | search | history_0-icase | rg | peak_rss | 4 | 6660096.000 | 6656000.000 | 6684672.000 | 6684672.000 | bytes |
| viberwhisper | search | history_0-icase | rg | wall | 32 | 4.840 | 4.599 | 6.113 | 6.185 | ms |
| viberwhisper | search | history_0-icase_literal | current | peak_rss | 4 | 9625600.000 | 9641984.000 | 9715712.000 | 9715712.000 | bytes |
| viberwhisper | search | history_0-icase_literal | current | wall | 32 | 3.924 | 4.071 | 5.197 | 5.643 | ms |
| viberwhisper | search | history_0-icase_literal | rg | peak_rss | 4 | 6422528.000 | 6418432.000 | 6455296.000 | 6455296.000 | bytes |
| viberwhisper | search | history_0-icase_literal | rg | wall | 32 | 4.734 | 4.765 | 6.161 | 6.531 | ms |
| viberwhisper | search | history_0-literal | current | peak_rss | 4 | 9527296.000 | 9515008.000 | 9535488.000 | 9535488.000 | bytes |
| viberwhisper | search | history_0-literal | current | wall | 32 | 3.866 | 3.947 | 4.438 | 4.923 | ms |
| viberwhisper | search | history_0-literal | rg | peak_rss | 4 | 6389760.000 | 6381568.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | search | history_0-literal | rg | wall | 32 | 4.673 | 4.412 | 5.086 | 5.690 | ms |
| viberwhisper | search | history_0-literal_lines | current | peak_rss | 4 | 9535488.000 | 9535488.000 | 9568256.000 | 9568256.000 | bytes |
| viberwhisper | search | history_0-literal_lines | current | wall | 32 | 4.177 | 4.178 | 4.774 | 4.791 | ms |
| viberwhisper | search | history_0-literal_lines | rg | peak_rss | 4 | 6414336.000 | 6414336.000 | 6455296.000 | 6455296.000 | bytes |
| viberwhisper | search | history_0-literal_lines | rg | wall | 32 | 4.674 | 4.485 | 5.508 | 5.930 | ms |
| viberwhisper | search | history_0-or | current | peak_rss | 4 | 9854976.000 | 9846784.000 | 9879552.000 | 9879552.000 | bytes |
| viberwhisper | search | history_0-or | current | wall | 32 | 4.215 | 4.355 | 5.341 | 5.775 | ms |
| viberwhisper | search | history_0-or | rg | peak_rss | 4 | 6684672.000 | 6672384.000 | 6717440.000 | 6717440.000 | bytes |
| viberwhisper | search | history_0-or | rg | wall | 32 | 4.840 | 4.619 | 5.356 | 6.189 | ms |
| viberwhisper | search | history_0-short | current | peak_rss | 4 | 9494528.000 | 9519104.000 | 9650176.000 | 9650176.000 | bytes |
| viberwhisper | search | history_0-short | current | wall | 32 | 4.295 | 4.378 | 5.178 | 5.264 | ms |
| viberwhisper | search | history_0-short | rg | peak_rss | 4 | 6381568.000 | 6377472.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | search | history_0-short | rg | wall | 32 | 4.594 | 4.367 | 5.833 | 5.956 | ms |
| viberwhisper | search | history_0-word | current | peak_rss | 4 | 9625600.000 | 9637888.000 | 9682944.000 | 9682944.000 | bytes |
| viberwhisper | search | history_0-word | current | wall | 32 | 4.102 | 4.144 | 4.874 | 4.937 | ms |
| viberwhisper | search | history_0-word | rg | peak_rss | 4 | 6602752.000 | 6594560.000 | 6619136.000 | 6619136.000 | bytes |
| viberwhisper | search | history_0-word | rg | wall | 32 | 4.767 | 4.576 | 6.052 | 6.401 | ms |
| viberwhisper | search | history_100-absent | current | peak_rss | 4 | 11042816.000 | 11046912.000 | 11059200.000 | 11059200.000 | bytes |
| viberwhisper | search | history_100-absent | current | wall | 32 | 5.155 | 5.226 | 5.657 | 5.956 | ms |
| viberwhisper | search | history_100-absent | rg | peak_rss | 4 | 6537216.000 | 6537216.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | search | history_100-absent | rg | wall | 32 | 5.797 | 6.146 | 7.667 | 9.570 | ms |
| viberwhisper | search | history_100-anchor | current | peak_rss | 4 | 19177472.000 | 19136512.000 | 19218432.000 | 19218432.000 | bytes |
| viberwhisper | search | history_100-anchor | current | wall | 32 | 8.324 | 8.283 | 8.991 | 9.177 | ms |
| viberwhisper | search | history_100-anchor | rg | peak_rss | 4 | 7987200.000 | 7999488.000 | 8093696.000 | 8093696.000 | bytes |
| viberwhisper | search | history_100-anchor | rg | wall | 32 | 7.618 | 8.033 | 9.466 | 10.698 | ms |
| viberwhisper | search | history_100-blank | current | peak_rss | 4 | 12328960.000 | 12333056.000 | 12386304.000 | 12386304.000 | bytes |
| viberwhisper | search | history_100-blank | current | wall | 32 | 9.787 | 9.875 | 10.724 | 11.626 | ms |
| viberwhisper | search | history_100-blank | rg | peak_rss | 4 | 7135232.000 | 7135232.000 | 7192576.000 | 7192576.000 | bytes |
| viberwhisper | search | history_100-blank | rg | wall | 32 | 7.258 | 7.584 | 9.097 | 9.126 | ms |
| viberwhisper | search | history_100-broad | current | peak_rss | 4 | 12263424.000 | 12288000.000 | 12402688.000 | 12402688.000 | bytes |
| viberwhisper | search | history_100-broad | current | wall | 32 | 6.364 | 6.577 | 7.824 | 8.158 | ms |
| viberwhisper | search | history_100-broad | rg | peak_rss | 4 | 6750208.000 | 6742016.000 | 6782976.000 | 6782976.000 | bytes |
| viberwhisper | search | history_100-broad | rg | wall | 32 | 7.089 | 7.106 | 8.211 | 8.294 | ms |
| viberwhisper | search | history_100-count | current | peak_rss | 4 | 11673600.000 | 11640832.000 | 11730944.000 | 11730944.000 | bytes |
| viberwhisper | search | history_100-count | current | wall | 32 | 7.013 | 7.076 | 7.537 | 8.801 | ms |
| viberwhisper | search | history_100-count | rg | peak_rss | 4 | 7274496.000 | 7307264.000 | 7454720.000 | 7454720.000 | bytes |
| viberwhisper | search | history_100-count | rg | wall | 32 | 7.009 | 7.288 | 8.604 | 8.705 | ms |
| viberwhisper | search | history_100-icase | current | peak_rss | 4 | 15122432.000 | 15126528.000 | 15155200.000 | 15155200.000 | bytes |
| viberwhisper | search | history_100-icase | current | wall | 32 | 6.058 | 6.039 | 6.949 | 7.078 | ms |
| viberwhisper | search | history_100-icase | rg | peak_rss | 4 | 6864896.000 | 6877184.000 | 6930432.000 | 6930432.000 | bytes |
| viberwhisper | search | history_100-icase | rg | wall | 32 | 5.885 | 6.142 | 7.185 | 7.272 | ms |
| viberwhisper | search | history_100-icase_literal | current | peak_rss | 4 | 14188544.000 | 14204928.000 | 14303232.000 | 14303232.000 | bytes |
| viberwhisper | search | history_100-icase_literal | current | wall | 32 | 5.587 | 5.728 | 6.998 | 8.782 | ms |
| viberwhisper | search | history_100-icase_literal | rg | peak_rss | 4 | 6619136.000 | 6631424.000 | 6701056.000 | 6701056.000 | bytes |
| viberwhisper | search | history_100-icase_literal | rg | wall | 32 | 6.133 | 6.449 | 7.743 | 10.412 | ms |
| viberwhisper | search | history_100-literal | current | peak_rss | 4 | 11993088.000 | 11980800.000 | 12009472.000 | 12009472.000 | bytes |
| viberwhisper | search | history_100-literal | current | wall | 32 | 5.495 | 5.748 | 8.515 | 8.537 | ms |
| viberwhisper | search | history_100-literal | rg | peak_rss | 4 | 6553600.000 | 6557696.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | search | history_100-literal | rg | wall | 32 | 5.805 | 6.083 | 7.123 | 7.541 | ms |
| viberwhisper | search | history_100-literal_lines | current | peak_rss | 4 | 12509184.000 | 12513280.000 | 12566528.000 | 12566528.000 | bytes |
| viberwhisper | search | history_100-literal_lines | current | wall | 32 | 6.213 | 6.303 | 7.209 | 7.460 | ms |
| viberwhisper | search | history_100-literal_lines | rg | peak_rss | 4 | 6774784.000 | 6787072.000 | 6848512.000 | 6848512.000 | bytes |
| viberwhisper | search | history_100-literal_lines | rg | wall | 32 | 6.504 | 6.835 | 8.064 | 8.972 | ms |
| viberwhisper | search | history_100-or | current | peak_rss | 4 | 14843904.000 | 14843904.000 | 14893056.000 | 14893056.000 | bytes |
| viberwhisper | search | history_100-or | current | wall | 32 | 6.711 | 6.787 | 7.957 | 8.247 | ms |
| viberwhisper | search | history_100-or | rg | peak_rss | 4 | 6930432.000 | 6934528.000 | 6979584.000 | 6979584.000 | bytes |
| viberwhisper | search | history_100-or | rg | wall | 32 | 6.713 | 7.026 | 8.444 | 9.142 | ms |
| viberwhisper | search | history_100-short | current | peak_rss | 4 | 11354112.000 | 11341824.000 | 11386880.000 | 11386880.000 | bytes |
| viberwhisper | search | history_100-short | current | wall | 32 | 6.442 | 6.513 | 7.414 | 7.955 | ms |
| viberwhisper | search | history_100-short | rg | peak_rss | 4 | 6766592.000 | 6754304.000 | 6782976.000 | 6782976.000 | bytes |
| viberwhisper | search | history_100-short | rg | wall | 32 | 6.899 | 7.139 | 8.004 | 8.969 | ms |
| viberwhisper | search | history_100-word | current | peak_rss | 4 | 12632064.000 | 12619776.000 | 12681216.000 | 12681216.000 | bytes |
| viberwhisper | search | history_100-word | current | wall | 32 | 6.249 | 6.418 | 7.608 | 7.645 | ms |
| viberwhisper | search | history_100-word | rg | peak_rss | 4 | 6905856.000 | 6914048.000 | 6995968.000 | 6995968.000 | bytes |
| viberwhisper | search | history_100-word | rg | wall | 32 | 6.728 | 6.919 | 8.183 | 9.000 | ms |
| viberwhisper | search | history_25-absent | current | peak_rss | 4 | 10240000.000 | 10248192.000 | 10289152.000 | 10289152.000 | bytes |
| viberwhisper | search | history_25-absent | current | wall | 32 | 4.553 | 4.689 | 5.415 | 5.941 | ms |
| viberwhisper | search | history_25-absent | rg | peak_rss | 4 | 6340608.000 | 6356992.000 | 6422528.000 | 6422528.000 | bytes |
| viberwhisper | search | history_25-absent | rg | wall | 32 | 5.009 | 4.948 | 6.315 | 6.494 | ms |
| viberwhisper | search | history_25-anchor | current | peak_rss | 4 | 12976128.000 | 12980224.000 | 13041664.000 | 13041664.000 | bytes |
| viberwhisper | search | history_25-anchor | current | wall | 32 | 6.114 | 6.190 | 6.877 | 7.293 | ms |
| viberwhisper | search | history_25-anchor | rg | peak_rss | 4 | 7692288.000 | 7696384.000 | 7749632.000 | 7749632.000 | bytes |
| viberwhisper | search | history_25-anchor | rg | wall | 32 | 6.694 | 6.930 | 8.006 | 8.323 | ms |
| viberwhisper | search | history_25-blank | current | peak_rss | 4 | 10747904.000 | 10756096.000 | 10813440.000 | 10813440.000 | bytes |
| viberwhisper | search | history_25-blank | current | wall | 32 | 6.340 | 6.331 | 6.948 | 7.189 | ms |
| viberwhisper | search | history_25-blank | rg | peak_rss | 4 | 6807552.000 | 6807552.000 | 6848512.000 | 6848512.000 | bytes |
| viberwhisper | search | history_25-blank | rg | wall | 32 | 5.784 | 6.143 | 8.013 | 8.077 | ms |
| viberwhisper | search | history_25-broad | current | peak_rss | 4 | 10682368.000 | 10674176.000 | 10698752.000 | 10698752.000 | bytes |
| viberwhisper | search | history_25-broad | current | wall | 32 | 5.150 | 5.213 | 5.891 | 6.336 | ms |
| viberwhisper | search | history_25-broad | rg | peak_rss | 4 | 6406144.000 | 6418432.000 | 6520832.000 | 6520832.000 | bytes |
| viberwhisper | search | history_25-broad | rg | wall | 32 | 5.680 | 5.778 | 7.156 | 7.481 | ms |
| viberwhisper | search | history_25-count | current | peak_rss | 4 | 10641408.000 | 10641408.000 | 10665984.000 | 10665984.000 | bytes |
| viberwhisper | search | history_25-count | current | wall | 32 | 5.341 | 5.535 | 6.382 | 7.124 | ms |
| viberwhisper | search | history_25-count | rg | peak_rss | 4 | 6840320.000 | 6864896.000 | 6979584.000 | 6979584.000 | bytes |
| viberwhisper | search | history_25-count | rg | wall | 32 | 5.434 | 5.733 | 6.748 | 6.764 | ms |
| viberwhisper | search | history_25-icase | current | peak_rss | 4 | 11730944.000 | 11726848.000 | 11763712.000 | 11763712.000 | bytes |
| viberwhisper | search | history_25-icase | current | wall | 32 | 5.065 | 5.234 | 6.236 | 6.351 | ms |
| viberwhisper | search | history_25-icase | rg | peak_rss | 4 | 6758400.000 | 6758400.000 | 6799360.000 | 6799360.000 | bytes |
| viberwhisper | search | history_25-icase | rg | wall | 32 | 5.045 | 5.248 | 6.557 | 6.731 | ms |
| viberwhisper | search | history_25-icase_literal | current | peak_rss | 4 | 11501568.000 | 11497472.000 | 11534336.000 | 11534336.000 | bytes |
| viberwhisper | search | history_25-icase_literal | current | wall | 32 | 4.899 | 5.003 | 5.883 | 5.895 | ms |
| viberwhisper | search | history_25-icase_literal | rg | peak_rss | 4 | 6471680.000 | 6471680.000 | 6504448.000 | 6504448.000 | bytes |
| viberwhisper | search | history_25-icase_literal | rg | wall | 32 | 5.032 | 5.178 | 6.627 | 7.203 | ms |
| viberwhisper | search | history_25-literal | current | peak_rss | 4 | 10608640.000 | 10612736.000 | 10633216.000 | 10633216.000 | bytes |
| viberwhisper | search | history_25-literal | current | wall | 32 | 4.633 | 4.771 | 5.566 | 5.862 | ms |
| viberwhisper | search | history_25-literal | rg | peak_rss | 4 | 6373376.000 | 6369280.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | search | history_25-literal | rg | wall | 32 | 4.760 | 4.727 | 6.170 | 6.635 | ms |
| viberwhisper | search | history_25-literal_lines | current | peak_rss | 4 | 10870784.000 | 10870784.000 | 10928128.000 | 10928128.000 | bytes |
| viberwhisper | search | history_25-literal_lines | current | wall | 32 | 4.997 | 5.184 | 6.344 | 6.413 | ms |
| viberwhisper | search | history_25-literal_lines | rg | peak_rss | 4 | 6496256.000 | 6488064.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | search | history_25-literal_lines | rg | wall | 32 | 5.382 | 5.694 | 7.220 | 7.449 | ms |
| viberwhisper | search | history_25-or | current | peak_rss | 4 | 11673600.000 | 11689984.000 | 11763712.000 | 11763712.000 | bytes |
| viberwhisper | search | history_25-or | current | wall | 32 | 5.237 | 5.313 | 6.207 | 6.301 | ms |
| viberwhisper | search | history_25-or | rg | peak_rss | 4 | 6758400.000 | 6762496.000 | 6815744.000 | 6815744.000 | bytes |
| viberwhisper | search | history_25-or | rg | wall | 32 | 5.417 | 5.683 | 6.830 | 6.956 | ms |
| viberwhisper | search | history_25-short | current | peak_rss | 4 | 10436608.000 | 10432512.000 | 10502144.000 | 10502144.000 | bytes |
| viberwhisper | search | history_25-short | current | wall | 32 | 5.124 | 5.277 | 6.074 | 7.364 | ms |
| viberwhisper | search | history_25-short | rg | peak_rss | 4 | 6488064.000 | 6500352.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | search | history_25-short | rg | wall | 32 | 5.521 | 5.666 | 6.904 | 8.280 | ms |
| viberwhisper | search | history_25-word | current | peak_rss | 4 | 11042816.000 | 11042816.000 | 11059200.000 | 11059200.000 | bytes |
| viberwhisper | search | history_25-word | current | wall | 32 | 5.276 | 5.293 | 6.101 | 6.300 | ms |
| viberwhisper | search | history_25-word | rg | peak_rss | 4 | 6668288.000 | 6668288.000 | 6701056.000 | 6701056.000 | bytes |
| viberwhisper | search | history_25-word | rg | wall | 32 | 5.501 | 5.720 | 6.816 | 8.290 | ms |
| viberwhisper | search | history_50-absent | current | peak_rss | 4 | 10551296.000 | 10559488.000 | 10616832.000 | 10616832.000 | bytes |
| viberwhisper | search | history_50-absent | current | wall | 32 | 4.850 | 4.978 | 5.962 | 6.430 | ms |
| viberwhisper | search | history_50-absent | rg | peak_rss | 4 | 6397952.000 | 6385664.000 | 6406144.000 | 6406144.000 | bytes |
| viberwhisper | search | history_50-absent | rg | wall | 32 | 5.517 | 5.485 | 6.423 | 6.822 | ms |
| viberwhisper | search | history_50-anchor | current | peak_rss | 4 | 15187968.000 | 15183872.000 | 15204352.000 | 15204352.000 | bytes |
| viberwhisper | search | history_50-anchor | current | wall | 32 | 7.000 | 7.058 | 7.765 | 8.311 | ms |
| viberwhisper | search | history_50-anchor | rg | peak_rss | 4 | 7831552.000 | 7831552.000 | 7897088.000 | 7897088.000 | bytes |
| viberwhisper | search | history_50-anchor | rg | wall | 32 | 7.685 | 7.524 | 9.041 | 9.130 | ms |
| viberwhisper | search | history_50-blank | current | peak_rss | 4 | 11051008.000 | 11055104.000 | 11108352.000 | 11108352.000 | bytes |
| viberwhisper | search | history_50-blank | current | wall | 32 | 7.079 | 7.116 | 8.067 | 8.515 | ms |
| viberwhisper | search | history_50-blank | rg | peak_rss | 4 | 6807552.000 | 6815744.000 | 6864896.000 | 6864896.000 | bytes |
| viberwhisper | search | history_50-blank | rg | wall | 32 | 6.628 | 6.622 | 7.349 | 7.734 | ms |
| viberwhisper | search | history_50-broad | current | peak_rss | 4 | 11182080.000 | 11182080.000 | 11223040.000 | 11223040.000 | bytes |
| viberwhisper | search | history_50-broad | current | wall | 32 | 5.332 | 5.392 | 6.208 | 6.273 | ms |
| viberwhisper | search | history_50-broad | rg | peak_rss | 4 | 6447104.000 | 6455296.000 | 6504448.000 | 6504448.000 | bytes |
| viberwhisper | search | history_50-broad | rg | wall | 32 | 5.745 | 5.964 | 7.666 | 7.781 | ms |
| viberwhisper | search | history_50-count | current | peak_rss | 4 | 10928128.000 | 10928128.000 | 10944512.000 | 10944512.000 | bytes |
| viberwhisper | search | history_50-count | current | wall | 32 | 6.053 | 6.151 | 7.134 | 7.440 | ms |
| viberwhisper | search | history_50-count | rg | peak_rss | 4 | 6979584.000 | 6995968.000 | 7094272.000 | 7094272.000 | bytes |
| viberwhisper | search | history_50-count | rg | wall | 32 | 5.912 | 6.139 | 7.205 | 7.300 | ms |
| viberwhisper | search | history_50-icase | current | peak_rss | 4 | 13041664.000 | 13029376.000 | 13058048.000 | 13058048.000 | bytes |
| viberwhisper | search | history_50-icase | current | wall | 32 | 5.311 | 5.416 | 6.585 | 6.685 | ms |
| viberwhisper | search | history_50-icase | rg | peak_rss | 4 | 6774784.000 | 6770688.000 | 6815744.000 | 6815744.000 | bytes |
| viberwhisper | search | history_50-icase | rg | wall | 32 | 5.339 | 5.620 | 6.524 | 6.538 | ms |
| viberwhisper | search | history_50-icase_literal | current | peak_rss | 4 | 12615680.000 | 12595200.000 | 12615680.000 | 12615680.000 | bytes |
| viberwhisper | search | history_50-icase_literal | current | wall | 32 | 5.054 | 5.202 | 6.238 | 6.467 | ms |
| viberwhisper | search | history_50-icase_literal | rg | peak_rss | 4 | 6414336.000 | 6414336.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | search | history_50-icase_literal | rg | wall | 32 | 6.002 | 5.851 | 7.565 | 7.719 | ms |
| viberwhisper | search | history_50-literal | current | peak_rss | 4 | 11231232.000 | 11227136.000 | 11255808.000 | 11255808.000 | bytes |
| viberwhisper | search | history_50-literal | current | wall | 32 | 4.991 | 5.131 | 5.978 | 6.336 | ms |
| viberwhisper | search | history_50-literal | rg | peak_rss | 4 | 6365184.000 | 6361088.000 | 6389760.000 | 6389760.000 | bytes |
| viberwhisper | search | history_50-literal | rg | wall | 32 | 5.526 | 5.566 | 6.529 | 6.571 | ms |
| viberwhisper | search | history_50-literal_lines | current | peak_rss | 4 | 11550720.000 | 11546624.000 | 11583488.000 | 11583488.000 | bytes |
| viberwhisper | search | history_50-literal_lines | current | wall | 32 | 5.328 | 5.545 | 6.651 | 6.913 | ms |
| viberwhisper | search | history_50-literal_lines | rg | peak_rss | 4 | 6586368.000 | 6598656.000 | 6651904.000 | 6651904.000 | bytes |
| viberwhisper | search | history_50-literal_lines | rg | wall | 32 | 5.618 | 5.851 | 6.745 | 7.588 | ms |
| viberwhisper | search | history_50-or | current | peak_rss | 4 | 12877824.000 | 12877824.000 | 12910592.000 | 12910592.000 | bytes |
| viberwhisper | search | history_50-or | current | wall | 32 | 5.891 | 5.978 | 6.698 | 7.168 | ms |
| viberwhisper | search | history_50-or | rg | peak_rss | 4 | 6766592.000 | 6750208.000 | 6782976.000 | 6782976.000 | bytes |
| viberwhisper | search | history_50-or | rg | wall | 32 | 6.168 | 6.195 | 7.816 | 8.126 | ms |
| viberwhisper | search | history_50-short | current | peak_rss | 4 | 10616832.000 | 10625024.000 | 10698752.000 | 10698752.000 | bytes |
| viberwhisper | search | history_50-short | current | wall | 32 | 5.625 | 5.816 | 6.848 | 7.388 | ms |
| viberwhisper | search | history_50-short | rg | peak_rss | 4 | 6504448.000 | 6512640.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | search | history_50-short | rg | wall | 32 | 6.051 | 6.175 | 7.723 | 7.941 | ms |
| viberwhisper | search | history_50-word | current | peak_rss | 4 | 11665408.000 | 11657216.000 | 11681792.000 | 11681792.000 | bytes |
| viberwhisper | search | history_50-word | current | wall | 32 | 5.591 | 5.746 | 7.159 | 7.184 | ms |
| viberwhisper | search | history_50-word | rg | peak_rss | 4 | 6676480.000 | 6680576.000 | 6733824.000 | 6733824.000 | bytes |
| viberwhisper | search | history_50-word | rg | wall | 32 | 5.724 | 5.915 | 6.951 | 7.282 | ms |
| viberwhisper | search | history_75-absent | current | peak_rss | 4 | 9945088.000 | 9945088.000 | 9961472.000 | 9961472.000 | bytes |
| viberwhisper | search | history_75-absent | current | wall | 32 | 4.752 | 4.841 | 5.637 | 6.975 | ms |
| viberwhisper | search | history_75-absent | rg | peak_rss | 4 | 6381568.000 | 6385664.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | search | history_75-absent | rg | wall | 32 | 6.074 | 6.323 | 8.354 | 8.886 | ms |
| viberwhisper | search | history_75-anchor | current | peak_rss | 4 | 13058048.000 | 13053952.000 | 13074432.000 | 13074432.000 | bytes |
| viberwhisper | search | history_75-anchor | current | wall | 32 | 6.737 | 6.765 | 7.391 | 8.048 | ms |
| viberwhisper | search | history_75-anchor | rg | peak_rss | 4 | 7954432.000 | 7946240.000 | 7979008.000 | 7979008.000 | bytes |
| viberwhisper | search | history_75-anchor | rg | wall | 32 | 7.590 | 8.180 | 10.412 | 13.551 | ms |
| viberwhisper | search | history_75-blank | current | peak_rss | 4 | 11141120.000 | 11137024.000 | 11206656.000 | 11206656.000 | bytes |
| viberwhisper | search | history_75-blank | current | wall | 32 | 8.645 | 8.833 | 10.430 | 10.622 | ms |
| viberwhisper | search | history_75-blank | rg | peak_rss | 4 | 6955008.000 | 6955008.000 | 7012352.000 | 7012352.000 | bytes |
| viberwhisper | search | history_75-blank | rg | wall | 32 | 7.374 | 7.686 | 9.646 | 9.743 | ms |
| viberwhisper | search | history_75-broad | current | peak_rss | 4 | 10502144.000 | 10518528.000 | 10600448.000 | 10600448.000 | bytes |
| viberwhisper | search | history_75-broad | current | wall | 32 | 5.765 | 5.914 | 7.626 | 8.044 | ms |
| viberwhisper | search | history_75-broad | rg | peak_rss | 4 | 6537216.000 | 6549504.000 | 6602752.000 | 6602752.000 | bytes |
| viberwhisper | search | history_75-broad | rg | wall | 32 | 7.252 | 7.121 | 8.412 | 8.442 | ms |
| viberwhisper | search | history_75-count | current | peak_rss | 4 | 10641408.000 | 10657792.000 | 10715136.000 | 10715136.000 | bytes |
| viberwhisper | search | history_75-count | current | wall | 32 | 6.499 | 6.550 | 7.405 | 7.806 | ms |
| viberwhisper | search | history_75-count | rg | peak_rss | 4 | 7217152.000 | 7200768.000 | 7241728.000 | 7241728.000 | bytes |
| viberwhisper | search | history_75-count | rg | wall | 32 | 7.646 | 7.405 | 8.332 | 8.805 | ms |
| viberwhisper | search | history_75-icase | current | peak_rss | 4 | 11141120.000 | 11149312.000 | 11190272.000 | 11190272.000 | bytes |
| viberwhisper | search | history_75-icase | current | wall | 32 | 4.998 | 5.147 | 6.359 | 7.818 | ms |
| viberwhisper | search | history_75-icase | rg | peak_rss | 4 | 6815744.000 | 6823936.000 | 6864896.000 | 6864896.000 | bytes |
| viberwhisper | search | history_75-icase | rg | wall | 32 | 6.187 | 6.431 | 8.366 | 8.925 | ms |
| viberwhisper | search | history_75-icase_literal | current | peak_rss | 4 | 10772480.000 | 10792960.000 | 10878976.000 | 10878976.000 | bytes |
| viberwhisper | search | history_75-icase_literal | current | wall | 32 | 4.909 | 4.997 | 5.506 | 7.185 | ms |
| viberwhisper | search | history_75-icase_literal | rg | peak_rss | 4 | 6455296.000 | 6455296.000 | 6488064.000 | 6488064.000 | bytes |
| viberwhisper | search | history_75-icase_literal | rg | wall | 32 | 5.808 | 6.366 | 8.453 | 9.752 | ms |
| viberwhisper | search | history_75-literal | current | peak_rss | 4 | 10264576.000 | 10256384.000 | 10272768.000 | 10272768.000 | bytes |
| viberwhisper | search | history_75-literal | current | wall | 32 | 4.694 | 4.871 | 5.968 | 6.136 | ms |
| viberwhisper | search | history_75-literal | rg | peak_rss | 4 | 6381568.000 | 6385664.000 | 6438912.000 | 6438912.000 | bytes |
| viberwhisper | search | history_75-literal | rg | wall | 32 | 6.450 | 6.337 | 7.620 | 8.422 | ms |
| viberwhisper | search | history_75-literal_lines | current | peak_rss | 4 | 10731520.000 | 10727424.000 | 10731520.000 | 10731520.000 | bytes |
| viberwhisper | search | history_75-literal_lines | current | wall | 32 | 5.664 | 5.679 | 6.564 | 6.676 | ms |
| viberwhisper | search | history_75-literal_lines | rg | peak_rss | 4 | 6643712.000 | 6639616.000 | 6668288.000 | 6668288.000 | bytes |
| viberwhisper | search | history_75-literal_lines | rg | wall | 32 | 6.288 | 6.670 | 7.710 | 8.391 | ms |
| viberwhisper | search | history_75-or | current | peak_rss | 4 | 11337728.000 | 11341824.000 | 11403264.000 | 11403264.000 | bytes |
| viberwhisper | search | history_75-or | current | wall | 32 | 5.717 | 5.804 | 6.648 | 6.894 | ms |
| viberwhisper | search | history_75-or | rg | peak_rss | 4 | 6856704.000 | 6856704.000 | 6897664.000 | 6897664.000 | bytes |
| viberwhisper | search | history_75-or | rg | wall | 32 | 6.714 | 6.963 | 8.374 | 8.472 | ms |
| viberwhisper | search | history_75-short | current | peak_rss | 4 | 10346496.000 | 10358784.000 | 10452992.000 | 10452992.000 | bytes |
| viberwhisper | search | history_75-short | current | wall | 32 | 5.947 | 6.167 | 6.969 | 9.254 | ms |
| viberwhisper | search | history_75-short | rg | peak_rss | 4 | 6569984.000 | 6574080.000 | 6602752.000 | 6602752.000 | bytes |
| viberwhisper | search | history_75-short | rg | wall | 32 | 7.202 | 7.348 | 9.091 | 9.361 | ms |
| viberwhisper | search | history_75-word | current | peak_rss | 4 | 10846208.000 | 10846208.000 | 10895360.000 | 10895360.000 | bytes |
| viberwhisper | search | history_75-word | current | wall | 32 | 5.617 | 5.888 | 7.707 | 7.766 | ms |
| viberwhisper | search | history_75-word | rg | peak_rss | 4 | 6815744.000 | 6803456.000 | 6832128.000 | 6832128.000 | bytes |
| viberwhisper | search | history_75-word | rg | wall | 32 | 6.452 | 6.814 | 8.806 | 9.042 | ms |
| viberwhisper | search | initial-absent | current | peak_rss | 2 | 9691136.000 | 9691136.000 | 9699328.000 | 9699328.000 | bytes |
| viberwhisper | search | initial-absent | current | wall | 16 | 4.936 | 5.081 | 6.569 | 6.569 | ms |
| viberwhisper | search | initial-absent | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | search | initial-absent | rg | wall | 16 | 6.065 | 6.221 | 6.846 | 6.846 | ms |
| viberwhisper | search | initial-anchor | current | peak_rss | 2 | 12140544.000 | 12140544.000 | 12189696.000 | 12189696.000 | bytes |
| viberwhisper | search | initial-anchor | current | wall | 16 | 6.749 | 6.993 | 8.067 | 8.067 | ms |
| viberwhisper | search | initial-anchor | rg | peak_rss | 2 | 7970816.000 | 7970816.000 | 7995392.000 | 7995392.000 | bytes |
| viberwhisper | search | initial-anchor | rg | wall | 16 | 8.251 | 8.515 | 9.990 | 9.990 | ms |
| viberwhisper | search | initial-blank | current | peak_rss | 2 | 11411456.000 | 11411456.000 | 11419648.000 | 11419648.000 | bytes |
| viberwhisper | search | initial-blank | current | wall | 16 | 10.125 | 10.227 | 11.637 | 11.637 | ms |
| viberwhisper | search | initial-blank | rg | peak_rss | 2 | 7086080.000 | 7086080.000 | 7110656.000 | 7110656.000 | bytes |
| viberwhisper | search | initial-blank | rg | wall | 16 | 7.957 | 8.061 | 10.288 | 10.288 | ms |
| viberwhisper | search | initial-broad | current | peak_rss | 2 | 10510336.000 | 10510336.000 | 10518528.000 | 10518528.000 | bytes |
| viberwhisper | search | initial-broad | current | wall | 16 | 6.050 | 6.278 | 8.260 | 8.260 | ms |
| viberwhisper | search | initial-broad | rg | peak_rss | 2 | 6676480.000 | 6676480.000 | 6684672.000 | 6684672.000 | bytes |
| viberwhisper | search | initial-broad | rg | wall | 16 | 7.750 | 7.664 | 9.411 | 9.411 | ms |
| viberwhisper | search | initial-count | current | peak_rss | 2 | 10674176.000 | 10674176.000 | 10715136.000 | 10715136.000 | bytes |
| viberwhisper | search | initial-count | current | wall | 16 | 6.700 | 6.787 | 8.588 | 8.588 | ms |
| viberwhisper | search | initial-count | rg | peak_rss | 2 | 7372800.000 | 7372800.000 | 7421952.000 | 7421952.000 | bytes |
| viberwhisper | search | initial-count | rg | wall | 16 | 7.456 | 7.555 | 9.613 | 9.613 | ms |
| viberwhisper | search | initial-icase | current | peak_rss | 2 | 10117120.000 | 10117120.000 | 10158080.000 | 10158080.000 | bytes |
| viberwhisper | search | initial-icase | current | wall | 16 | 4.985 | 5.263 | 6.929 | 6.929 | ms |
| viberwhisper | search | initial-icase | rg | peak_rss | 2 | 6873088.000 | 6873088.000 | 6897664.000 | 6897664.000 | bytes |
| viberwhisper | search | initial-icase | rg | wall | 16 | 6.611 | 6.830 | 8.725 | 8.725 | ms |
| viberwhisper | search | initial-icase_literal | current | peak_rss | 2 | 9977856.000 | 9977856.000 | 10010624.000 | 10010624.000 | bytes |
| viberwhisper | search | initial-icase_literal | current | wall | 16 | 5.170 | 5.137 | 5.949 | 5.949 | ms |
| viberwhisper | search | initial-icase_literal | rg | peak_rss | 2 | 6619136.000 | 6619136.000 | 6635520.000 | 6635520.000 | bytes |
| viberwhisper | search | initial-icase_literal | rg | wall | 16 | 6.626 | 6.902 | 11.262 | 11.262 | ms |
| viberwhisper | search | initial-literal | current | peak_rss | 2 | 9887744.000 | 9887744.000 | 9895936.000 | 9895936.000 | bytes |
| viberwhisper | search | initial-literal | current | wall | 16 | 5.081 | 5.235 | 6.374 | 6.374 | ms |
| viberwhisper | search | initial-literal | rg | peak_rss | 2 | 6594560.000 | 6594560.000 | 6619136.000 | 6619136.000 | bytes |
| viberwhisper | search | initial-literal | rg | wall | 16 | 6.748 | 6.911 | 9.127 | 9.127 | ms |
| viberwhisper | search | initial-literal_lines | current | peak_rss | 2 | 10321920.000 | 10321920.000 | 10371072.000 | 10371072.000 | bytes |
| viberwhisper | search | initial-literal_lines | current | wall | 16 | 6.007 | 6.118 | 8.571 | 8.571 | ms |
| viberwhisper | search | initial-literal_lines | rg | peak_rss | 2 | 6782976.000 | 6782976.000 | 6815744.000 | 6815744.000 | bytes |
| viberwhisper | search | initial-literal_lines | rg | wall | 16 | 7.492 | 7.419 | 8.482 | 8.482 | ms |
| viberwhisper | search | initial-or | current | peak_rss | 2 | 10936320.000 | 10936320.000 | 10944512.000 | 10944512.000 | bytes |
| viberwhisper | search | initial-or | current | wall | 16 | 6.440 | 6.392 | 8.185 | 8.185 | ms |
| viberwhisper | search | initial-or | rg | peak_rss | 2 | 6938624.000 | 6938624.000 | 6963200.000 | 6963200.000 | bytes |
| viberwhisper | search | initial-or | rg | wall | 16 | 7.971 | 7.513 | 8.294 | 8.294 | ms |
| viberwhisper | search | initial-short | current | peak_rss | 2 | 10428416.000 | 10428416.000 | 10518528.000 | 10518528.000 | bytes |
| viberwhisper | search | initial-short | current | wall | 16 | 6.346 | 6.310 | 7.340 | 7.340 | ms |
| viberwhisper | search | initial-short | rg | peak_rss | 2 | 6733824.000 | 6733824.000 | 6750208.000 | 6750208.000 | bytes |
| viberwhisper | search | initial-short | rg | wall | 16 | 7.342 | 7.711 | 10.229 | 10.229 | ms |
| viberwhisper | search | initial-word | current | peak_rss | 2 | 10461184.000 | 10461184.000 | 10469376.000 | 10469376.000 | bytes |
| viberwhisper | search | initial-word | current | wall | 16 | 5.993 | 6.079 | 7.135 | 7.135 | ms |
| viberwhisper | search | initial-word | rg | peak_rss | 2 | 6840320.000 | 6840320.000 | 6864896.000 | 6864896.000 | bytes |
| viberwhisper | search | initial-word | rg | wall | 16 | 7.093 | 7.252 | 9.232 | 9.232 | ms |
| viberwhisper | workflow | edit_1pct-add_commit | current | index_size | 3 | 2603754.000 | 2603754.000 | 2603754.000 | 2603754.000 | bytes |
| viberwhisper | workflow | edit_1pct-add_commit | current | peak_rss | 2 | 10403840.000 | 10403840.000 | 10403840.000 | 10403840.000 | bytes |
| viberwhisper | workflow | edit_1pct-add_commit | current | searchable_bytes | 3 | 1876246.000 | 1876246.000 | 1876246.000 | 1876246.000 | bytes |
| viberwhisper | workflow | edit_1pct-add_commit | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_1pct-add_commit | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_1pct-add_commit | current | wall | 3 | 6.511 | 6.586 | 6.840 | 6.840 | ms |
| viberwhisper | workflow | edit_1pct-add_commit | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | workflow | edit_1pct-add_commit | rg | wall | 3 | 6.140 | 6.494 | 7.768 | 7.768 | ms |
| viberwhisper | workflow | edit_1pct-before_ignore | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-before_ignore | current | index_size | 3 | 2604152.000 | 2604152.000 | 2604152.000 | 2604152.000 | bytes |
| viberwhisper | workflow | edit_1pct-before_ignore | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10403840.000 | 10403840.000 | bytes |
| viberwhisper | workflow | edit_1pct-before_ignore | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-before_ignore | current | searchable_bytes | 3 | 1876248.000 | 1876248.000 | 1876248.000 | 1876248.000 | bytes |
| viberwhisper | workflow | edit_1pct-before_ignore | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_1pct-before_ignore | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_1pct-before_ignore | current | wall | 3 | 6.313 | 6.486 | 7.270 | 7.270 | ms |
| viberwhisper | workflow | edit_1pct-before_ignore | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | workflow | edit_1pct-before_ignore | rg | wall | 3 | 7.760 | 8.598 | 10.778 | 10.778 | ms |
| viberwhisper | workflow | edit_1pct-delete | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-delete | current | index_size | 3 | 2603670.000 | 2603670.000 | 2603670.000 | 2603670.000 | bytes |
| viberwhisper | workflow | edit_1pct-delete | current | peak_rss | 2 | 10133504.000 | 10133504.000 | 10141696.000 | 10141696.000 | bytes |
| viberwhisper | workflow | edit_1pct-delete | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-delete | current | searchable_bytes | 3 | 1876221.000 | 1876221.000 | 1876221.000 | 1876221.000 | bytes |
| viberwhisper | workflow | edit_1pct-delete | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_1pct-delete | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_1pct-delete | current | wall | 3 | 5.761 | 5.703 | 5.801 | 5.801 | ms |
| viberwhisper | workflow | edit_1pct-delete | rg | peak_rss | 2 | 6479872.000 | 6479872.000 | 6504448.000 | 6504448.000 | bytes |
| viberwhisper | workflow | edit_1pct-delete | rg | wall | 3 | 7.182 | 7.444 | 8.725 | 8.725 | ms |
| viberwhisper | workflow | edit_1pct-dirty | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-dirty | current | index_size | 3 | 2618519.000 | 2618519.000 | 2618519.000 | 2618519.000 | bytes |
| viberwhisper | workflow | edit_1pct-dirty | current | peak_rss | 2 | 11255808.000 | 11255808.000 | 11255808.000 | 11255808.000 | bytes |
| viberwhisper | workflow | edit_1pct-dirty | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-dirty | current | searchable_bytes | 3 | 1877950.000 | 1877950.000 | 1877950.000 | 1877950.000 | bytes |
| viberwhisper | workflow | edit_1pct-dirty | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_1pct-dirty | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_1pct-dirty | current | wall | 3 | 6.919 | 6.866 | 7.017 | 7.017 | ms |
| viberwhisper | workflow | edit_1pct-dirty | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6602752.000 | 6602752.000 | bytes |
| viberwhisper | workflow | edit_1pct-dirty | rg | wall | 3 | 7.715 | 7.569 | 7.950 | 7.950 | ms |
| viberwhisper | workflow | edit_1pct-discard | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-discard | current | index_size | 3 | 2588354.000 | 2588354.000 | 2588354.000 | 2588354.000 | bytes |
| viberwhisper | workflow | edit_1pct-discard | current | peak_rss | 2 | 10240000.000 | 10240000.000 | 10272768.000 | 10272768.000 | bytes |
| viberwhisper | workflow | edit_1pct-discard | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-discard | current | searchable_bytes | 3 | 1876221.000 | 1876221.000 | 1876221.000 | 1876221.000 | bytes |
| viberwhisper | workflow | edit_1pct-discard | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_1pct-discard | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_1pct-discard | current | wall | 3 | 5.777 | 6.200 | 7.155 | 7.155 | ms |
| viberwhisper | workflow | edit_1pct-discard | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | workflow | edit_1pct-discard | rg | wall | 3 | 5.953 | 6.175 | 6.856 | 6.856 | ms |
| viberwhisper | workflow | edit_1pct-edit | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-edit | current | index_size | 3 | 2573792.000 | 2573792.000 | 2573792.000 | 2573792.000 | bytes |
| viberwhisper | workflow | edit_1pct-edit | current | peak_rss | 2 | 11223040.000 | 11223040.000 | 11239424.000 | 11239424.000 | bytes |
| viberwhisper | workflow | edit_1pct-edit | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-edit | current | searchable_bytes | 3 | 1876221.000 | 1876221.000 | 1876221.000 | 1876221.000 | bytes |
| viberwhisper | workflow | edit_1pct-edit | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_1pct-edit | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_1pct-edit | current | wall | 3 | 6.544 | 7.406 | 9.216 | 9.216 | ms |
| viberwhisper | workflow | edit_1pct-edit | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | workflow | edit_1pct-edit | rg | wall | 3 | 7.307 | 8.212 | 10.788 | 10.788 | ms |
| viberwhisper | workflow | edit_1pct-ignored | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-ignored | current | index_size | 3 | 2604006.000 | 2604006.000 | 2604006.000 | 2604006.000 | bytes |
| viberwhisper | workflow | edit_1pct-ignored | current | peak_rss | 2 | 10477568.000 | 10477568.000 | 10534912.000 | 10534912.000 | bytes |
| viberwhisper | workflow | edit_1pct-ignored | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-ignored | current | searchable_bytes | 3 | 1876226.000 | 1876226.000 | 1876226.000 | 1876226.000 | bytes |
| viberwhisper | workflow | edit_1pct-ignored | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_1pct-ignored | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_1pct-ignored | current | wall | 3 | 6.295 | 6.713 | 7.653 | 7.653 | ms |
| viberwhisper | workflow | edit_1pct-ignored | rg | peak_rss | 2 | 6578176.000 | 6578176.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | workflow | edit_1pct-ignored | rg | wall | 3 | 5.859 | 5.985 | 6.462 | 6.462 | ms |
| viberwhisper | workflow | edit_1pct-promotion | current | index_size | 3 | 2588354.000 | 2588354.000 | 2588354.000 | 2588354.000 | bytes |
| viberwhisper | workflow | edit_1pct-promotion | current | peak_rss | 2 | 10371072.000 | 10371072.000 | 10420224.000 | 10420224.000 | bytes |
| viberwhisper | workflow | edit_1pct-promotion | current | searchable_bytes | 3 | 1876221.000 | 1876221.000 | 1876221.000 | 1876221.000 | bytes |
| viberwhisper | workflow | edit_1pct-promotion | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_1pct-promotion | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_1pct-promotion | current | wall | 3 | 8.892 | 9.787 | 13.741 | 13.741 | ms |
| viberwhisper | workflow | edit_1pct-promotion | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6553600.000 | 6553600.000 | bytes |
| viberwhisper | workflow | edit_1pct-promotion | rg | wall | 3 | 6.587 | 6.832 | 8.070 | 8.070 | ms |
| viberwhisper | workflow | edit_1pct-rename | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-rename | current | index_size | 3 | 2604131.000 | 2604131.000 | 2604131.000 | 2604131.000 | bytes |
| viberwhisper | workflow | edit_1pct-rename | current | peak_rss | 2 | 10395648.000 | 10395648.000 | 10403840.000 | 10403840.000 | bytes |
| viberwhisper | workflow | edit_1pct-rename | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-rename | current | searchable_bytes | 3 | 1876246.000 | 1876246.000 | 1876246.000 | 1876246.000 | bytes |
| viberwhisper | workflow | edit_1pct-rename | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_1pct-rename | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_1pct-rename | current | wall | 3 | 6.322 | 6.430 | 6.717 | 6.717 | ms |
| viberwhisper | workflow | edit_1pct-rename | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6520832.000 | 6520832.000 | bytes |
| viberwhisper | workflow | edit_1pct-rename | rg | wall | 3 | 5.826 | 6.167 | 6.864 | 6.864 | ms |
| viberwhisper | workflow | edit_1pct-revisit | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-revisit | current | index_size | 3 | 2588354.000 | 2588354.000 | 2588354.000 | 2588354.000 | bytes |
| viberwhisper | workflow | edit_1pct-revisit | current | peak_rss | 2 | 10231808.000 | 10231808.000 | 10240000.000 | 10240000.000 | bytes |
| viberwhisper | workflow | edit_1pct-revisit | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-revisit | current | searchable_bytes | 3 | 1876221.000 | 1876221.000 | 1876221.000 | 1876221.000 | bytes |
| viberwhisper | workflow | edit_1pct-revisit | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_1pct-revisit | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_1pct-revisit | current | wall | 3 | 5.684 | 5.597 | 5.792 | 5.792 | ms |
| viberwhisper | workflow | edit_1pct-revisit | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6635520.000 | 6635520.000 | bytes |
| viberwhisper | workflow | edit_1pct-revisit | rg | wall | 3 | 6.478 | 6.631 | 6.939 | 6.939 | ms |
| viberwhisper | workflow | edit_1pct-rollback | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-rollback | current | index_size | 3 | 2588248.000 | 2588248.000 | 2588248.000 | 2588248.000 | bytes |
| viberwhisper | workflow | edit_1pct-rollback | current | peak_rss | 2 | 10182656.000 | 10182656.000 | 10190848.000 | 10190848.000 | bytes |
| viberwhisper | workflow | edit_1pct-rollback | current | reused | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-rollback | current | searchable_bytes | 3 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | workflow | edit_1pct-rollback | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_1pct-rollback | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-rollback | current | wall | 3 | 5.840 | 5.899 | 6.042 | 6.042 | ms |
| viberwhisper | workflow | edit_1pct-rollback | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6569984.000 | 6569984.000 | bytes |
| viberwhisper | workflow | edit_1pct-rollback | rg | wall | 3 | 5.888 | 5.745 | 5.938 | 5.938 | ms |
| viberwhisper | workflow | edit_1pct-untracked | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_1pct-untracked | current | index_size | 3 | 2588909.000 | 2588909.000 | 2588909.000 | 2588909.000 | bytes |
| viberwhisper | workflow | edit_1pct-untracked | current | peak_rss | 2 | 10387456.000 | 10387456.000 | 10387456.000 | 10387456.000 | bytes |
| viberwhisper | workflow | edit_1pct-untracked | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_1pct-untracked | current | searchable_bytes | 3 | 1876246.000 | 1876246.000 | 1876246.000 | 1876246.000 | bytes |
| viberwhisper | workflow | edit_1pct-untracked | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_1pct-untracked | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_1pct-untracked | current | wall | 3 | 6.150 | 6.314 | 6.923 | 6.923 | ms |
| viberwhisper | workflow | edit_1pct-untracked | rg | peak_rss | 2 | 6512640.000 | 6512640.000 | 6520832.000 | 6520832.000 | bytes |
| viberwhisper | workflow | edit_1pct-untracked | rg | wall | 3 | 8.534 | 7.714 | 8.597 | 8.597 | ms |
| viberwhisper | workflow | edit_50pct-add_commit | current | index_size | 3 | 3094527.000 | 3094527.000 | 3094527.000 | 3094527.000 | bytes |
| viberwhisper | workflow | edit_50pct-add_commit | current | peak_rss | 2 | 10420224.000 | 10420224.000 | 10436608.000 | 10436608.000 | bytes |
| viberwhisper | workflow | edit_50pct-add_commit | current | searchable_bytes | 3 | 1926385.000 | 1926385.000 | 1926385.000 | 1926385.000 | bytes |
| viberwhisper | workflow | edit_50pct-add_commit | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_50pct-add_commit | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_50pct-add_commit | current | wall | 3 | 6.090 | 6.276 | 6.713 | 6.713 | ms |
| viberwhisper | workflow | edit_50pct-add_commit | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | workflow | edit_50pct-add_commit | rg | wall | 3 | 5.757 | 5.893 | 6.324 | 6.324 | ms |
| viberwhisper | workflow | edit_50pct-before_ignore | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_50pct-before_ignore | current | index_size | 3 | 3094925.000 | 3094925.000 | 3094925.000 | 3094925.000 | bytes |
| viberwhisper | workflow | edit_50pct-before_ignore | current | peak_rss | 2 | 10354688.000 | 10354688.000 | 10371072.000 | 10371072.000 | bytes |
| viberwhisper | workflow | edit_50pct-before_ignore | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-before_ignore | current | searchable_bytes | 3 | 1926387.000 | 1926387.000 | 1926387.000 | 1926387.000 | bytes |
| viberwhisper | workflow | edit_50pct-before_ignore | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_50pct-before_ignore | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_50pct-before_ignore | current | wall | 3 | 5.923 | 6.098 | 6.494 | 6.494 | ms |
| viberwhisper | workflow | edit_50pct-before_ignore | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6537216.000 | 6537216.000 | bytes |
| viberwhisper | workflow | edit_50pct-before_ignore | rg | wall | 3 | 6.595 | 6.774 | 8.281 | 8.281 | ms |
| viberwhisper | workflow | edit_50pct-delete | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-delete | current | index_size | 3 | 3094443.000 | 3094443.000 | 3094443.000 | 3094443.000 | bytes |
| viberwhisper | workflow | edit_50pct-delete | current | peak_rss | 2 | 10215424.000 | 10215424.000 | 10240000.000 | 10240000.000 | bytes |
| viberwhisper | workflow | edit_50pct-delete | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-delete | current | searchable_bytes | 3 | 1926360.000 | 1926360.000 | 1926360.000 | 1926360.000 | bytes |
| viberwhisper | workflow | edit_50pct-delete | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_50pct-delete | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_50pct-delete | current | wall | 3 | 5.753 | 5.830 | 5.985 | 5.985 | ms |
| viberwhisper | workflow | edit_50pct-delete | rg | peak_rss | 2 | 6586368.000 | 6586368.000 | 6602752.000 | 6602752.000 | bytes |
| viberwhisper | workflow | edit_50pct-delete | rg | wall | 3 | 6.838 | 6.595 | 7.453 | 7.453 | ms |
| viberwhisper | workflow | edit_50pct-dirty | current | extracted | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | workflow | edit_50pct-dirty | current | index_size | 3 | 3606765.000 | 3606765.000 | 3606765.000 | 3606765.000 | bytes |
| viberwhisper | workflow | edit_50pct-dirty | current | peak_rss | 2 | 15867904.000 | 15867904.000 | 16482304.000 | 16482304.000 | bytes |
| viberwhisper | workflow | edit_50pct-dirty | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-dirty | current | searchable_bytes | 3 | 1974772.000 | 1974772.000 | 1974772.000 | 1974772.000 | bytes |
| viberwhisper | workflow | edit_50pct-dirty | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_50pct-dirty | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_50pct-dirty | current | wall | 3 | 11.188 | 11.169 | 11.331 | 11.331 | ms |
| viberwhisper | workflow | edit_50pct-dirty | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6520832.000 | 6520832.000 | bytes |
| viberwhisper | workflow | edit_50pct-dirty | rg | wall | 3 | 9.546 | 9.428 | 9.886 | 9.886 | ms |
| viberwhisper | workflow | edit_50pct-discard | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-discard | current | index_size | 3 | 3079127.000 | 3079127.000 | 3079127.000 | 3079127.000 | bytes |
| viberwhisper | workflow | edit_50pct-discard | current | peak_rss | 2 | 10502144.000 | 10502144.000 | 10518528.000 | 10518528.000 | bytes |
| viberwhisper | workflow | edit_50pct-discard | current | reused | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | workflow | edit_50pct-discard | current | searchable_bytes | 3 | 1926360.000 | 1926360.000 | 1926360.000 | 1926360.000 | bytes |
| viberwhisper | workflow | edit_50pct-discard | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_50pct-discard | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_50pct-discard | current | wall | 3 | 6.408 | 6.410 | 6.737 | 6.737 | ms |
| viberwhisper | workflow | edit_50pct-discard | rg | peak_rss | 2 | 6537216.000 | 6537216.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | workflow | edit_50pct-discard | rg | wall | 3 | 5.927 | 6.194 | 7.131 | 7.131 | ms |
| viberwhisper | workflow | edit_50pct-edit | current | extracted | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | workflow | edit_50pct-edit | current | index_size | 3 | 3064565.000 | 3064565.000 | 3064565.000 | 3064565.000 | bytes |
| viberwhisper | workflow | edit_50pct-edit | current | peak_rss | 2 | 15310848.000 | 15310848.000 | 15745024.000 | 15745024.000 | bytes |
| viberwhisper | workflow | edit_50pct-edit | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-edit | current | searchable_bytes | 3 | 1926360.000 | 1926360.000 | 1926360.000 | 1926360.000 | bytes |
| viberwhisper | workflow | edit_50pct-edit | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_50pct-edit | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_50pct-edit | current | wall | 3 | 12.589 | 15.208 | 21.854 | 21.854 | ms |
| viberwhisper | workflow | edit_50pct-edit | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6619136.000 | 6619136.000 | bytes |
| viberwhisper | workflow | edit_50pct-edit | rg | wall | 3 | 7.239 | 7.304 | 7.712 | 7.712 | ms |
| viberwhisper | workflow | edit_50pct-ignored | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_50pct-ignored | current | index_size | 3 | 3094779.000 | 3094779.000 | 3094779.000 | 3094779.000 | bytes |
| viberwhisper | workflow | edit_50pct-ignored | current | peak_rss | 2 | 10461184.000 | 10461184.000 | 10469376.000 | 10469376.000 | bytes |
| viberwhisper | workflow | edit_50pct-ignored | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-ignored | current | searchable_bytes | 3 | 1926365.000 | 1926365.000 | 1926365.000 | 1926365.000 | bytes |
| viberwhisper | workflow | edit_50pct-ignored | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_50pct-ignored | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_50pct-ignored | current | wall | 3 | 6.647 | 6.564 | 6.901 | 6.901 | ms |
| viberwhisper | workflow | edit_50pct-ignored | rg | peak_rss | 2 | 6561792.000 | 6561792.000 | 6602752.000 | 6602752.000 | bytes |
| viberwhisper | workflow | edit_50pct-ignored | rg | wall | 3 | 7.022 | 8.207 | 11.369 | 11.369 | ms |
| viberwhisper | workflow | edit_50pct-promotion | current | index_size | 3 | 3079127.000 | 3079127.000 | 3079127.000 | 3079127.000 | bytes |
| viberwhisper | workflow | edit_50pct-promotion | current | peak_rss | 2 | 10395648.000 | 10395648.000 | 10420224.000 | 10420224.000 | bytes |
| viberwhisper | workflow | edit_50pct-promotion | current | searchable_bytes | 3 | 1926360.000 | 1926360.000 | 1926360.000 | 1926360.000 | bytes |
| viberwhisper | workflow | edit_50pct-promotion | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_50pct-promotion | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_50pct-promotion | current | wall | 3 | 6.623 | 6.663 | 6.754 | 6.754 | ms |
| viberwhisper | workflow | edit_50pct-promotion | rg | peak_rss | 2 | 6578176.000 | 6578176.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | workflow | edit_50pct-promotion | rg | wall | 3 | 7.023 | 7.296 | 9.059 | 9.059 | ms |
| viberwhisper | workflow | edit_50pct-rename | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_50pct-rename | current | index_size | 3 | 3094904.000 | 3094904.000 | 3094904.000 | 3094904.000 | bytes |
| viberwhisper | workflow | edit_50pct-rename | current | peak_rss | 2 | 10395648.000 | 10395648.000 | 10403840.000 | 10403840.000 | bytes |
| viberwhisper | workflow | edit_50pct-rename | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-rename | current | searchable_bytes | 3 | 1926385.000 | 1926385.000 | 1926385.000 | 1926385.000 | bytes |
| viberwhisper | workflow | edit_50pct-rename | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_50pct-rename | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_50pct-rename | current | wall | 3 | 6.397 | 6.444 | 6.843 | 6.843 | ms |
| viberwhisper | workflow | edit_50pct-rename | rg | peak_rss | 2 | 6520832.000 | 6520832.000 | 6537216.000 | 6537216.000 | bytes |
| viberwhisper | workflow | edit_50pct-rename | rg | wall | 3 | 7.951 | 8.649 | 12.442 | 12.442 | ms |
| viberwhisper | workflow | edit_50pct-revisit | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-revisit | current | index_size | 3 | 3079127.000 | 3079127.000 | 3079127.000 | 3079127.000 | bytes |
| viberwhisper | workflow | edit_50pct-revisit | current | peak_rss | 2 | 10428416.000 | 10428416.000 | 10452992.000 | 10452992.000 | bytes |
| viberwhisper | workflow | edit_50pct-revisit | current | reused | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | workflow | edit_50pct-revisit | current | searchable_bytes | 3 | 1926360.000 | 1926360.000 | 1926360.000 | 1926360.000 | bytes |
| viberwhisper | workflow | edit_50pct-revisit | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_50pct-revisit | current | segments | 3 | 2.000 | 2.000 | 2.000 | 2.000 | count |
| viberwhisper | workflow | edit_50pct-revisit | current | wall | 3 | 5.938 | 5.921 | 5.946 | 5.946 | ms |
| viberwhisper | workflow | edit_50pct-revisit | rg | peak_rss | 2 | 6545408.000 | 6545408.000 | 6602752.000 | 6602752.000 | bytes |
| viberwhisper | workflow | edit_50pct-revisit | rg | wall | 3 | 7.173 | 6.789 | 7.417 | 7.417 | ms |
| viberwhisper | workflow | edit_50pct-rollback | current | extracted | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-rollback | current | index_size | 3 | 3079021.000 | 3079021.000 | 3079021.000 | 3079021.000 | bytes |
| viberwhisper | workflow | edit_50pct-rollback | current | peak_rss | 2 | 10420224.000 | 10420224.000 | 10436608.000 | 10436608.000 | bytes |
| viberwhisper | workflow | edit_50pct-rollback | current | reused | 3 | 28.000 | 28.000 | 28.000 | 28.000 | count |
| viberwhisper | workflow | edit_50pct-rollback | current | searchable_bytes | 3 | 1874364.000 | 1874364.000 | 1874364.000 | 1874364.000 | bytes |
| viberwhisper | workflow | edit_50pct-rollback | current | searchable_files | 3 | 144.000 | 144.000 | 144.000 | 144.000 | count |
| viberwhisper | workflow | edit_50pct-rollback | current | segments | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_50pct-rollback | current | wall | 3 | 6.316 | 6.417 | 6.825 | 6.825 | ms |
| viberwhisper | workflow | edit_50pct-rollback | rg | peak_rss | 2 | 6529024.000 | 6529024.000 | 6537216.000 | 6537216.000 | bytes |
| viberwhisper | workflow | edit_50pct-rollback | rg | wall | 3 | 6.298 | 6.147 | 6.414 | 6.414 | ms |
| viberwhisper | workflow | edit_50pct-untracked | current | extracted | 3 | 1.000 | 1.000 | 1.000 | 1.000 | count |
| viberwhisper | workflow | edit_50pct-untracked | current | index_size | 3 | 3079682.000 | 3079682.000 | 3079682.000 | 3079682.000 | bytes |
| viberwhisper | workflow | edit_50pct-untracked | current | peak_rss | 2 | 10444800.000 | 10444800.000 | 10452992.000 | 10452992.000 | bytes |
| viberwhisper | workflow | edit_50pct-untracked | current | reused | 3 | 0.000 | 0.000 | 0.000 | 0.000 | count |
| viberwhisper | workflow | edit_50pct-untracked | current | searchable_bytes | 3 | 1926385.000 | 1926385.000 | 1926385.000 | 1926385.000 | bytes |
| viberwhisper | workflow | edit_50pct-untracked | current | searchable_files | 3 | 145.000 | 145.000 | 145.000 | 145.000 | count |
| viberwhisper | workflow | edit_50pct-untracked | current | segments | 3 | 3.000 | 3.000 | 3.000 | 3.000 | count |
| viberwhisper | workflow | edit_50pct-untracked | current | wall | 3 | 6.842 | 6.677 | 7.047 | 7.047 | ms |
| viberwhisper | workflow | edit_50pct-untracked | rg | peak_rss | 2 | 6569984.000 | 6569984.000 | 6586368.000 | 6586368.000 | bytes |
| viberwhisper | workflow | edit_50pct-untracked | rg | wall | 3 | 7.805 | 7.893 | 10.016 | 10.016 | ms |

## Same-run comparisons

Negative delta means lower cost. Differences of medians, without confidence intervals.

| Corpus | Scenario | Case | Variant | Reference | Metric | Delta | Change % |
|---|---|---|---|---|---|---:|---:|
| agentflow | branches | edit_1pct-commit_A1 | current | rg | peak_rss | 3858432.000 | 59.39 |
| agentflow | branches | edit_1pct-commit_A1 | current | rg | wall | -0.029 | -0.49 |
| agentflow | branches | edit_1pct-commit_A2 | current | rg | peak_rss | 3940352.000 | 60.58 |
| agentflow | branches | edit_1pct-commit_A2 | current | rg | wall | 0.680 | 11.70 |
| agentflow | branches | edit_1pct-commit_B1 | current | rg | peak_rss | 3923968.000 | 60.71 |
| agentflow | branches | edit_1pct-commit_B1 | current | rg | wall | 0.257 | 4.02 |
| agentflow | branches | edit_1pct-commit_B2 | current | rg | peak_rss | 3981312.000 | 61.75 |
| agentflow | branches | edit_1pct-commit_B2 | current | rg | wall | -1.126 | -14.88 |
| agentflow | branches | edit_1pct-edit_A1 | current | rg | peak_rss | 4882432.000 | 75.73 |
| agentflow | branches | edit_1pct-edit_A1 | current | rg | wall | -0.501 | -6.42 |
| agentflow | branches | edit_1pct-edit_A2 | current | rg | peak_rss | 4898816.000 | 75.98 |
| agentflow | branches | edit_1pct-edit_A2 | current | rg | wall | -0.474 | -6.63 |
| agentflow | branches | edit_1pct-edit_B1 | current | rg | peak_rss | 4784128.000 | 73.64 |
| agentflow | branches | edit_1pct-edit_B1 | current | rg | wall | -0.884 | -11.19 |
| agentflow | branches | edit_1pct-edit_B2 | current | rg | peak_rss | 4841472.000 | 74.53 |
| agentflow | branches | edit_1pct-edit_B2 | current | rg | wall | 0.434 | 6.52 |
| agentflow | branches | edit_1pct-revisit_A | current | rg | peak_rss | 3719168.000 | 57.32 |
| agentflow | branches | edit_1pct-revisit_A | current | rg | wall | -1.233 | -17.75 |
| agentflow | branches | edit_1pct-revisit_B | current | rg | peak_rss | 3694592.000 | 56.30 |
| agentflow | branches | edit_1pct-revisit_B | current | rg | wall | -0.524 | -8.74 |
| agentflow | branches | edit_1pct-switch_A1 | current | rg | peak_rss | 3162112.000 | 48.74 |
| agentflow | branches | edit_1pct-switch_A1 | current | rg | wall | -3.195 | -38.56 |
| agentflow | branches | edit_1pct-switch_A2 | current | rg | peak_rss | 3850240.000 | 59.57 |
| agentflow | branches | edit_1pct-switch_A2 | current | rg | wall | -2.253 | -28.10 |
| agentflow | branches | edit_1pct-switch_B1 | current | rg | peak_rss | 3686400.000 | 56.53 |
| agentflow | branches | edit_1pct-switch_B1 | current | rg | wall | -0.592 | -9.26 |
| agentflow | branches | edit_1pct-switch_B2 | current | rg | peak_rss | 3743744.000 | 57.48 |
| agentflow | branches | edit_1pct-switch_B2 | current | rg | wall | -1.200 | -15.81 |
| agentflow | branches | edit_50pct-commit_A1 | current | rg | peak_rss | 3883008.000 | 59.70 |
| agentflow | branches | edit_50pct-commit_A1 | current | rg | wall | -0.677 | -10.21 |
| agentflow | branches | edit_50pct-commit_A2 | current | rg | peak_rss | 3907584.000 | 60.23 |
| agentflow | branches | edit_50pct-commit_A2 | current | rg | wall | 0.780 | 12.75 |
| agentflow | branches | edit_50pct-commit_B1 | current | rg | peak_rss | 3948544.000 | 61.09 |
| agentflow | branches | edit_50pct-commit_B1 | current | rg | wall | -0.134 | -2.01 |
| agentflow | branches | edit_50pct-commit_B2 | current | rg | peak_rss | 3997696.000 | 62.24 |
| agentflow | branches | edit_50pct-commit_B2 | current | rg | wall | -0.105 | -1.46 |
| agentflow | branches | edit_50pct-edit_A1 | current | rg | peak_rss | 9814016.000 | 149.38 |
| agentflow | branches | edit_50pct-edit_A1 | current | rg | wall | 8.386 | 103.04 |
| agentflow | branches | edit_50pct-edit_A2 | current | rg | peak_rss | 11091968.000 | 171.83 |
| agentflow | branches | edit_50pct-edit_A2 | current | rg | wall | 9.041 | 139.11 |
| agentflow | branches | edit_50pct-edit_B1 | current | rg | peak_rss | 10117120.000 | 155.35 |
| agentflow | branches | edit_50pct-edit_B1 | current | rg | wall | 7.400 | 96.15 |
| agentflow | branches | edit_50pct-edit_B2 | current | rg | peak_rss | 10018816.000 | 153.84 |
| agentflow | branches | edit_50pct-edit_B2 | current | rg | wall | 7.733 | 97.37 |
| agentflow | branches | edit_50pct-revisit_A | current | rg | peak_rss | 4063232.000 | 63.10 |
| agentflow | branches | edit_50pct-revisit_A | current | rg | wall | 0.256 | 3.58 |
| agentflow | branches | edit_50pct-revisit_B | current | rg | peak_rss | 4046848.000 | 62.37 |
| agentflow | branches | edit_50pct-revisit_B | current | rg | wall | -0.763 | -10.60 |
| agentflow | branches | edit_50pct-switch_A1 | current | rg | peak_rss | 3112960.000 | 47.50 |
| agentflow | branches | edit_50pct-switch_A1 | current | rg | wall | -2.937 | -39.24 |
| agentflow | branches | edit_50pct-switch_A2 | current | rg | peak_rss | 3989504.000 | 61.34 |
| agentflow | branches | edit_50pct-switch_A2 | current | rg | wall | -0.636 | -8.54 |
| agentflow | branches | edit_50pct-switch_B1 | current | rg | peak_rss | 3989504.000 | 61.49 |
| agentflow | branches | edit_50pct-switch_B1 | current | rg | wall | -0.829 | -12.26 |
| agentflow | branches | edit_50pct-switch_B2 | current | rg | peak_rss | 4022272.000 | 61.84 |
| agentflow | branches | edit_50pct-switch_B2 | current | rg | wall | 0.045 | 0.70 |
| agentflow | history | revisit_0 | current | rg | peak_rss | 10756096.000 | 166.20 |
| agentflow | history | revisit_0 | current | rg | wall | 16.426 | 297.24 |
| agentflow | history | revisit_28 | current | rg | peak_rss | 17203200.000 | 263.49 |
| agentflow | history | revisit_28 | current | rg | wall | 27.112 | 399.91 |
| agentflow | history | revisit_56 | current | rg | peak_rss | 4153344.000 | 63.30 |
| agentflow | history | revisit_56 | current | rg | wall | 0.518 | 8.12 |
| agentflow | history | step_001 | current | rg | peak_rss | 7962624.000 | 123.19 |
| agentflow | history | step_001 | current | rg | wall | 4.721 | 96.90 |
| agentflow | history | step_002 | current | rg | peak_rss | 7872512.000 | 121.65 |
| agentflow | history | step_002 | current | rg | wall | 4.813 | 86.29 |
| agentflow | history | step_003 | current | rg | peak_rss | 4620288.000 | 71.30 |
| agentflow | history | step_003 | current | rg | wall | 1.610 | 33.64 |
| agentflow | history | step_004 | current | rg | peak_rss | 4808704.000 | 73.37 |
| agentflow | history | step_004 | current | rg | wall | 3.042 | 81.71 |
| agentflow | history | step_005 | current | rg | peak_rss | 10182656.000 | 158.34 |
| agentflow | history | step_005 | current | rg | wall | 15.209 | 327.84 |
| agentflow | history | step_006 | current | rg | peak_rss | 11116544.000 | 170.48 |
| agentflow | history | step_006 | current | rg | wall | 15.053 | 246.74 |
| agentflow | history | step_007 | current | rg | peak_rss | 6897664.000 | 105.12 |
| agentflow | history | step_007 | current | rg | wall | 3.778 | 62.13 |
| agentflow | history | step_008 | current | rg | peak_rss | 8151040.000 | 125.31 |
| agentflow | history | step_008 | current | rg | wall | 5.343 | 92.14 |
| agentflow | history | step_009 | current | rg | peak_rss | 5791744.000 | 88.93 |
| agentflow | history | step_009 | current | rg | wall | 1.257 | 20.44 |
| agentflow | history | step_010 | current | rg | peak_rss | 8085504.000 | 123.53 |
| agentflow | history | step_010 | current | rg | wall | 5.680 | 109.52 |
| agentflow | history | step_011 | current | rg | peak_rss | 9846784.000 | 151.58 |
| agentflow | history | step_011 | current | rg | wall | 12.929 | 211.40 |
| agentflow | history | step_012 | current | rg | peak_rss | 9306112.000 | 142.36 |
| agentflow | history | step_012 | current | rg | wall | 6.995 | 111.87 |
| agentflow | history | step_013 | current | rg | peak_rss | 7512064.000 | 116.22 |
| agentflow | history | step_013 | current | rg | wall | 2.856 | 49.40 |
| agentflow | history | step_014 | current | rg | peak_rss | 9019392.000 | 137.97 |
| agentflow | history | step_014 | current | rg | wall | 6.689 | 124.32 |
| agentflow | history | step_015 | current | rg | peak_rss | 5365760.000 | 82.49 |
| agentflow | history | step_015 | current | rg | wall | 1.015 | 17.15 |
| agentflow | history | step_016 | current | rg | peak_rss | 8675328.000 | 133.04 |
| agentflow | history | step_016 | current | rg | wall | 6.822 | 124.91 |
| agentflow | history | step_017 | current | rg | peak_rss | 12468224.000 | 190.25 |
| agentflow | history | step_017 | current | rg | wall | 15.570 | 244.30 |
| agentflow | history | step_018 | current | rg | peak_rss | 9904128.000 | 152.84 |
| agentflow | history | step_018 | current | rg | wall | 8.142 | 130.47 |
| agentflow | history | step_019 | current | rg | peak_rss | 19292160.000 | 295.85 |
| agentflow | history | step_019 | current | rg | wall | 69.337 | 1152.79 |
| agentflow | history | step_020 | current | rg | peak_rss | 7708672.000 | 118.22 |
| agentflow | history | step_020 | current | rg | wall | 4.787 | 94.43 |
| agentflow | history | step_021 | current | rg | peak_rss | 8970240.000 | 137.05 |
| agentflow | history | step_021 | current | rg | wall | 6.470 | 112.64 |
| agentflow | history | step_022 | current | rg | peak_rss | 9150464.000 | 142.11 |
| agentflow | history | step_022 | current | rg | wall | 6.711 | 104.46 |
| agentflow | history | step_023 | current | rg | peak_rss | 8028160.000 | 123.58 |
| agentflow | history | step_023 | current | rg | wall | 5.604 | 101.13 |
| agentflow | history | step_024 | current | rg | peak_rss | 8372224.000 | 128.07 |
| agentflow | history | step_024 | current | rg | wall | 4.679 | 73.58 |
| agentflow | history | step_025 | current | rg | peak_rss | 5652480.000 | 86.90 |
| agentflow | history | step_025 | current | rg | wall | 1.214 | 19.11 |
| agentflow | history | step_026 | current | rg | peak_rss | 5046272.000 | 77.19 |
| agentflow | history | step_026 | current | rg | wall | 1.945 | 29.53 |
| agentflow | history | step_027 | current | rg | peak_rss | 8192000.000 | 126.10 |
| agentflow | history | step_027 | current | rg | wall | 3.777 | 54.61 |
| agentflow | history | step_028 | current | rg | peak_rss | 6266880.000 | 96.35 |
| agentflow | history | step_028 | current | rg | wall | 1.690 | 28.85 |
| agentflow | history | step_029 | current | rg | peak_rss | 4415488.000 | 67.88 |
| agentflow | history | step_029 | current | rg | wall | -0.512 | -6.75 |
| agentflow | history | step_030 | current | rg | peak_rss | 8036352.000 | 122.93 |
| agentflow | history | step_030 | current | rg | wall | 4.758 | 91.37 |
| agentflow | history | step_031 | current | rg | peak_rss | 5062656.000 | 78.03 |
| agentflow | history | step_031 | current | rg | wall | 1.642 | 28.99 |
| agentflow | history | step_032 | current | rg | peak_rss | 8642560.000 | 132.37 |
| agentflow | history | step_032 | current | rg | wall | 4.471 | 65.59 |
| agentflow | history | step_033 | current | rg | peak_rss | 6471680.000 | 98.87 |
| agentflow | history | step_033 | current | rg | wall | 2.016 | 32.97 |
| agentflow | history | step_034 | current | rg | peak_rss | 5136384.000 | 79.17 |
| agentflow | history | step_034 | current | rg | wall | 0.101 | 1.36 |
| agentflow | history | step_035 | current | rg | peak_rss | 8167424.000 | 125.09 |
| agentflow | history | step_035 | current | rg | wall | 3.535 | 53.72 |
| agentflow | history | step_036 | current | rg | peak_rss | 7004160.000 | 108.09 |
| agentflow | history | step_036 | current | rg | wall | 2.883 | 50.39 |
| agentflow | history | step_037 | current | rg | peak_rss | 7086080.000 | 108.81 |
| agentflow | history | step_037 | current | rg | wall | 1.920 | 29.19 |
| agentflow | history | step_038 | current | rg | peak_rss | 5382144.000 | 82.54 |
| agentflow | history | step_038 | current | rg | wall | 0.553 | 7.56 |
| agentflow | history | step_039 | current | rg | peak_rss | 8536064.000 | 131.40 |
| agentflow | history | step_039 | current | rg | wall | 2.726 | 36.56 |
| agentflow | history | step_040 | current | rg | peak_rss | 5398528.000 | 83.00 |
| agentflow | history | step_040 | current | rg | wall | -0.283 | -3.75 |
| agentflow | history | step_041 | current | rg | peak_rss | 9871360.000 | 150.62 |
| agentflow | history | step_041 | current | rg | wall | 6.568 | 109.37 |
| agentflow | history | step_042 | current | rg | peak_rss | 7389184.000 | 114.18 |
| agentflow | history | step_042 | current | rg | wall | 0.714 | 9.70 |
| agentflow | history | step_043 | current | rg | peak_rss | 5627904.000 | 86.52 |
| agentflow | history | step_043 | current | rg | wall | 2.042 | 30.23 |
| agentflow | history | step_044 | current | rg | peak_rss | 10936320.000 | 168.14 |
| agentflow | history | step_044 | current | rg | wall | 6.672 | 91.04 |
| agentflow | history | step_045 | current | rg | peak_rss | 8224768.000 | 126.45 |
| agentflow | history | step_045 | current | rg | wall | 2.602 | 37.56 |
| agentflow | history | step_046 | current | rg | peak_rss | 5603328.000 | 85.61 |
| agentflow | history | step_046 | current | rg | wall | -2.041 | -22.08 |
| agentflow | history | step_047 | current | rg | peak_rss | 8445952.000 | 129.85 |
| agentflow | history | step_047 | current | rg | wall | 4.024 | 63.71 |
| agentflow | history | step_048 | current | rg | peak_rss | 6660096.000 | 102.65 |
| agentflow | history | step_048 | current | rg | wall | 1.849 | 23.42 |
| agentflow | history | step_049 | current | rg | peak_rss | 9199616.000 | 141.08 |
| agentflow | history | step_049 | current | rg | wall | 5.623 | 98.99 |
| agentflow | history | step_050 | current | rg | peak_rss | 18374656.000 | 281.43 |
| agentflow | history | step_050 | current | rg | wall | 82.495 | 884.16 |
| agentflow | history | step_051 | current | rg | peak_rss | 6209536.000 | 95.23 |
| agentflow | history | step_051 | current | rg | wall | 0.465 | 6.39 |
| agentflow | history | step_052 | current | rg | peak_rss | 4767744.000 | 73.48 |
| agentflow | history | step_052 | current | rg | wall | 0.280 | 4.15 |
| agentflow | history | step_053 | current | rg | peak_rss | 7340032.000 | 112.00 |
| agentflow | history | step_053 | current | rg | wall | 3.162 | 44.47 |
| agentflow | history | step_054 | current | rg | peak_rss | 6004736.000 | 92.20 |
| agentflow | history | step_054 | current | rg | wall | 2.043 | 30.66 |
| agentflow | history | step_055 | current | rg | peak_rss | 4833280.000 | 73.66 |
| agentflow | history | step_055 | current | rg | wall | -1.334 | -13.32 |
| agentflow | history | step_056 | current | rg | peak_rss | 6168576.000 | 94.12 |
| agentflow | history | step_056 | current | rg | wall | -0.336 | -3.98 |
| agentflow | search | history_0-absent | current | rg | peak_rss | 3145728.000 | 48.92 |
| agentflow | search | history_0-absent | current | rg | wall | -0.858 | -18.43 |
| agentflow | search | history_0-anchor | current | rg | peak_rss | 3661824.000 | 48.32 |
| agentflow | search | history_0-anchor | current | rg | wall | -2.279 | -33.31 |
| agentflow | search | history_0-blank | current | rg | peak_rss | 3661824.000 | 55.12 |
| agentflow | search | history_0-blank | current | rg | wall | 0.030 | 0.65 |
| agentflow | search | history_0-broad | current | rg | peak_rss | 3407872.000 | 52.93 |
| agentflow | search | history_0-broad | current | rg | wall | -0.683 | -14.85 |
| agentflow | search | history_0-count | current | rg | peak_rss | 3457024.000 | 49.82 |
| agentflow | search | history_0-count | current | rg | wall | -0.481 | -8.96 |
| agentflow | search | history_0-icase | current | rg | peak_rss | 3203072.000 | 46.83 |
| agentflow | search | history_0-icase | current | rg | wall | -0.784 | -16.07 |
| agentflow | search | history_0-icase_literal | current | rg | peak_rss | 3235840.000 | 49.69 |
| agentflow | search | history_0-icase_literal | current | rg | wall | -0.645 | -13.82 |
| agentflow | search | history_0-literal | current | rg | peak_rss | 3252224.000 | 50.38 |
| agentflow | search | history_0-literal | current | rg | wall | -0.699 | -15.38 |
| agentflow | search | history_0-literal_lines | current | rg | peak_rss | 3227648.000 | 49.44 |
| agentflow | search | history_0-literal_lines | current | rg | wall | -0.603 | -12.99 |
| agentflow | search | history_0-or | current | rg | peak_rss | 3317760.000 | 49.33 |
| agentflow | search | history_0-or | current | rg | wall | -0.838 | -17.16 |
| agentflow | search | history_0-short | current | rg | peak_rss | 3612672.000 | 55.68 |
| agentflow | search | history_0-short | current | rg | wall | -0.599 | -12.79 |
| agentflow | search | history_0-word | current | rg | peak_rss | 3211264.000 | 48.22 |
| agentflow | search | history_0-word | current | rg | wall | -0.646 | -13.84 |
| agentflow | search | history_14-absent | current | rg | peak_rss | 3776512.000 | 58.21 |
| agentflow | search | history_14-absent | current | rg | wall | -0.910 | -17.40 |
| agentflow | search | history_14-anchor | current | rg | peak_rss | 8265728.000 | 104.24 |
| agentflow | search | history_14-anchor | current | rg | wall | -0.477 | -6.83 |
| agentflow | search | history_14-blank | current | rg | peak_rss | 4374528.000 | 63.72 |
| agentflow | search | history_14-blank | current | rg | wall | 0.575 | 9.54 |
| agentflow | search | history_14-broad | current | rg | peak_rss | 5210112.000 | 78.91 |
| agentflow | search | history_14-broad | current | rg | wall | -0.435 | -7.75 |
| agentflow | search | history_14-count | current | rg | peak_rss | 3940352.000 | 55.48 |
| agentflow | search | history_14-count | current | rg | wall | -0.168 | -2.87 |
| agentflow | search | history_14-icase | current | rg | peak_rss | 6103040.000 | 89.11 |
| agentflow | search | history_14-icase | current | rg | wall | -1.356 | -21.69 |
| agentflow | search | history_14-icase_literal | current | rg | peak_rss | 5218304.000 | 78.35 |
| agentflow | search | history_14-icase_literal | current | rg | wall | -1.369 | -22.76 |
| agentflow | search | history_14-literal | current | rg | peak_rss | 4915200.000 | 74.81 |
| agentflow | search | history_14-literal | current | rg | wall | -0.612 | -11.52 |
| agentflow | search | history_14-literal_lines | current | rg | peak_rss | 4644864.000 | 69.40 |
| agentflow | search | history_14-literal_lines | current | rg | wall | -0.284 | -5.30 |
| agentflow | search | history_14-or | current | rg | peak_rss | 5922816.000 | 86.69 |
| agentflow | search | history_14-or | current | rg | wall | -0.324 | -5.77 |
| agentflow | search | history_14-short | current | rg | peak_rss | 4194304.000 | 63.29 |
| agentflow | search | history_14-short | current | rg | wall | -0.364 | -6.59 |
| agentflow | search | history_14-word | current | rg | peak_rss | 4669440.000 | 69.01 |
| agentflow | search | history_14-word | current | rg | wall | -0.454 | -8.00 |
| agentflow | search | history_28-absent | current | rg | peak_rss | 3440640.000 | 52.63 |
| agentflow | search | history_28-absent | current | rg | wall | -1.162 | -21.25 |
| agentflow | search | history_28-anchor | current | rg | peak_rss | 6758400.000 | 84.62 |
| agentflow | search | history_28-anchor | current | rg | wall | -0.925 | -11.89 |
| agentflow | search | history_28-blank | current | rg | peak_rss | 4349952.000 | 63.14 |
| agentflow | search | history_28-blank | current | rg | wall | 0.757 | 10.93 |
| agentflow | search | history_28-broad | current | rg | peak_rss | 4636672.000 | 69.62 |
| agentflow | search | history_28-broad | current | rg | wall | -0.769 | -12.43 |
| agentflow | search | history_28-count | current | rg | peak_rss | 3629056.000 | 49.50 |
| agentflow | search | history_28-count | current | rg | wall | -0.713 | -10.86 |
| agentflow | search | history_28-icase | current | rg | peak_rss | 4964352.000 | 72.23 |
| agentflow | search | history_28-icase | current | rg | wall | -1.387 | -22.38 |
| agentflow | search | history_28-icase_literal | current | rg | peak_rss | 4440064.000 | 67.33 |
| agentflow | search | history_28-icase_literal | current | rg | wall | -0.975 | -17.42 |
| agentflow | search | history_28-literal | current | rg | peak_rss | 4374528.000 | 65.93 |
| agentflow | search | history_28-literal | current | rg | wall | -0.774 | -13.31 |
| agentflow | search | history_28-literal_lines | current | rg | peak_rss | 4300800.000 | 63.33 |
| agentflow | search | history_28-literal_lines | current | rg | wall | -0.589 | -9.47 |
| agentflow | search | history_28-or | current | rg | peak_rss | 5152768.000 | 75.24 |
| agentflow | search | history_28-or | current | rg | wall | -0.622 | -10.22 |
| agentflow | search | history_28-short | current | rg | peak_rss | 4022272.000 | 60.47 |
| agentflow | search | history_28-short | current | rg | wall | -0.899 | -13.96 |
| agentflow | search | history_28-word | current | rg | peak_rss | 4317184.000 | 62.66 |
| agentflow | search | history_28-word | current | rg | wall | -0.528 | -8.47 |
| agentflow | search | history_42-absent | current | rg | peak_rss | 3915776.000 | 60.05 |
| agentflow | search | history_42-absent | current | rg | wall | -1.092 | -18.82 |
| agentflow | search | history_42-anchor | current | rg | peak_rss | 9347072.000 | 116.19 |
| agentflow | search | history_42-anchor | current | rg | wall | -0.339 | -4.20 |
| agentflow | search | history_42-blank | current | rg | peak_rss | 4669440.000 | 67.62 |
| agentflow | search | history_42-blank | current | rg | wall | 0.916 | 12.24 |
| agentflow | search | history_42-broad | current | rg | peak_rss | 5939200.000 | 89.73 |
| agentflow | search | history_42-broad | current | rg | wall | -0.927 | -13.83 |
| agentflow | search | history_42-count | current | rg | peak_rss | 3858432.000 | 52.04 |
| agentflow | search | history_42-count | current | rg | wall | -0.708 | -9.76 |
| agentflow | search | history_42-icase | current | rg | peak_rss | 6717440.000 | 97.50 |
| agentflow | search | history_42-icase | current | rg | wall | -0.962 | -15.29 |
| agentflow | search | history_42-icase_literal | current | rg | peak_rss | 5980160.000 | 90.80 |
| agentflow | search | history_42-icase_literal | current | rg | wall | -1.125 | -18.18 |
| agentflow | search | history_42-literal | current | rg | peak_rss | 5554176.000 | 83.39 |
| agentflow | search | history_42-literal | current | rg | wall | -0.763 | -12.31 |
| agentflow | search | history_42-literal_lines | current | rg | peak_rss | 5087232.000 | 74.91 |
| agentflow | search | history_42-literal_lines | current | rg | wall | -0.460 | -6.95 |
| agentflow | search | history_42-or | current | rg | peak_rss | 6561792.000 | 95.13 |
| agentflow | search | history_42-or | current | rg | wall | -0.609 | -9.00 |
| agentflow | search | history_42-short | current | rg | peak_rss | 4358144.000 | 65.36 |
| agentflow | search | history_42-short | current | rg | wall | -0.486 | -7.28 |
| agentflow | search | history_42-word | current | rg | peak_rss | 5029888.000 | 72.75 |
| agentflow | search | history_42-word | current | rg | wall | -0.490 | -7.26 |
| agentflow | search | history_56-absent | current | rg | peak_rss | 3407872.000 | 52.53 |
| agentflow | search | history_56-absent | current | rg | wall | -1.608 | -26.41 |
| agentflow | search | history_56-anchor | current | rg | peak_rss | 4939776.000 | 61.59 |
| agentflow | search | history_56-anchor | current | rg | wall | -1.337 | -15.49 |
| agentflow | search | history_56-blank | current | rg | peak_rss | 4325376.000 | 62.19 |
| agentflow | search | history_56-blank | current | rg | wall | 0.751 | 9.16 |
| agentflow | search | history_56-broad | current | rg | peak_rss | 4087808.000 | 61.45 |
| agentflow | search | history_56-broad | current | rg | wall | -1.908 | -24.94 |
| agentflow | search | history_56-count | current | rg | peak_rss | 3465216.000 | 46.74 |
| agentflow | search | history_56-count | current | rg | wall | -1.303 | -16.33 |
| agentflow | search | history_56-icase | current | rg | peak_rss | 4128768.000 | 59.57 |
| agentflow | search | history_56-icase | current | rg | wall | -1.540 | -23.54 |
| agentflow | search | history_56-icase_literal | current | rg | peak_rss | 4079616.000 | 61.25 |
| agentflow | search | history_56-icase_literal | current | rg | wall | -1.314 | -21.67 |
| agentflow | search | history_56-literal | current | rg | peak_rss | 3956736.000 | 59.56 |
| agentflow | search | history_56-literal | current | rg | wall | -1.684 | -24.42 |
| agentflow | search | history_56-literal_lines | current | rg | peak_rss | 3891200.000 | 56.89 |
| agentflow | search | history_56-literal_lines | current | rg | wall | -0.682 | -9.67 |
| agentflow | search | history_56-or | current | rg | peak_rss | 4317184.000 | 62.37 |
| agentflow | search | history_56-or | current | rg | wall | -1.206 | -17.06 |
| agentflow | search | history_56-short | current | rg | peak_rss | 3973120.000 | 59.58 |
| agentflow | search | history_56-short | current | rg | wall | -1.725 | -21.58 |
| agentflow | search | history_56-word | current | rg | peak_rss | 3907584.000 | 56.32 |
| agentflow | search | history_56-word | current | rg | wall | -0.759 | -10.78 |
| agentflow | search | initial-absent | current | rg | peak_rss | 3088384.000 | 47.36 |
| agentflow | search | initial-absent | current | rg | wall | -2.036 | -30.87 |
| agentflow | search | initial-anchor | current | rg | peak_rss | 4030464.000 | 49.95 |
| agentflow | search | initial-anchor | current | rg | wall | -1.418 | -15.44 |
| agentflow | search | initial-blank | current | rg | peak_rss | 4325376.000 | 62.86 |
| agentflow | search | initial-blank | current | rg | wall | 0.891 | 10.29 |
| agentflow | search | initial-broad | current | rg | peak_rss | 3678208.000 | 55.30 |
| agentflow | search | initial-broad | current | rg | wall | -1.484 | -19.98 |
| agentflow | search | initial-count | current | rg | peak_rss | 3440640.000 | 47.24 |
| agentflow | search | initial-count | current | rg | wall | -1.643 | -18.42 |
| agentflow | search | initial-icase | current | rg | peak_rss | 3399680.000 | 49.00 |
| agentflow | search | initial-icase | current | rg | wall | -1.274 | -20.13 |
| agentflow | search | initial-icase_literal | current | rg | peak_rss | 3432448.000 | 51.73 |
| agentflow | search | initial-icase_literal | current | rg | wall | -1.177 | -18.29 |
| agentflow | search | initial-literal | current | rg | peak_rss | 3481600.000 | 52.40 |
| agentflow | search | initial-literal | current | rg | wall | -2.359 | -30.58 |
| agentflow | search | initial-literal_lines | current | rg | peak_rss | 3571712.000 | 52.28 |
| agentflow | search | initial-literal_lines | current | rg | wall | -1.133 | -14.35 |
| agentflow | search | initial-or | current | rg | peak_rss | 3817472.000 | 54.89 |
| agentflow | search | initial-or | current | rg | wall | -1.024 | -13.91 |
| agentflow | search | initial-short | current | rg | peak_rss | 3801088.000 | 57.07 |
| agentflow | search | initial-short | current | rg | wall | -1.969 | -23.28 |
| agentflow | search | initial-word | current | rg | peak_rss | 3620864.000 | 51.88 |
| agentflow | search | initial-word | current | rg | wall | -1.656 | -20.51 |
| agentflow | workflow | edit_1pct-add_commit | current | rg | peak_rss | 3817472.000 | 58.10 |
| agentflow | workflow | edit_1pct-add_commit | current | rg | wall | -0.978 | -13.22 |
| agentflow | workflow | edit_1pct-before_ignore | current | rg | peak_rss | 3932160.000 | 60.68 |
| agentflow | workflow | edit_1pct-before_ignore | current | rg | wall | -1.267 | -18.12 |
| agentflow | workflow | edit_1pct-delete | current | rg | peak_rss | 3661824.000 | 56.23 |
| agentflow | workflow | edit_1pct-delete | current | rg | wall | 0.006 | 0.10 |
| agentflow | workflow | edit_1pct-dirty | current | rg | peak_rss | 4825088.000 | 74.56 |
| agentflow | workflow | edit_1pct-dirty | current | rg | wall | 0.463 | 6.13 |
| agentflow | workflow | edit_1pct-discard | current | rg | peak_rss | 3727360.000 | 57.52 |
| agentflow | workflow | edit_1pct-discard | current | rg | wall | -1.045 | -15.46 |
| agentflow | workflow | edit_1pct-edit | current | rg | peak_rss | 4792320.000 | 73.40 |
| agentflow | workflow | edit_1pct-edit | current | rg | wall | -0.923 | -11.22 |
| agentflow | workflow | edit_1pct-ignored | current | rg | peak_rss | 3940352.000 | 60.28 |
| agentflow | workflow | edit_1pct-ignored | current | rg | wall | 0.691 | 11.36 |
| agentflow | workflow | edit_1pct-promotion | current | rg | peak_rss | 3850240.000 | 58.82 |
| agentflow | workflow | edit_1pct-promotion | current | rg | wall | -0.064 | -1.06 |
| agentflow | workflow | edit_1pct-rename | current | rg | peak_rss | 3891200.000 | 59.60 |
| agentflow | workflow | edit_1pct-rename | current | rg | wall | -0.395 | -6.04 |
| agentflow | workflow | edit_1pct-revisit | current | rg | peak_rss | 3776512.000 | 57.99 |
| agentflow | workflow | edit_1pct-revisit | current | rg | wall | -1.481 | -20.17 |
| agentflow | workflow | edit_1pct-rollback | current | rg | peak_rss | 3751936.000 | 58.05 |
| agentflow | workflow | edit_1pct-rollback | current | rg | wall | -0.417 | -6.90 |
| agentflow | workflow | edit_1pct-untracked | current | rg | peak_rss | 3973120.000 | 61.01 |
| agentflow | workflow | edit_1pct-untracked | current | rg | wall | -1.678 | -21.16 |
| agentflow | workflow | edit_50pct-add_commit | current | rg | peak_rss | 3989504.000 | 61.88 |
| agentflow | workflow | edit_50pct-add_commit | current | rg | wall | -0.299 | -4.75 |
| agentflow | workflow | edit_50pct-before_ignore | current | rg | peak_rss | 3850240.000 | 58.90 |
| agentflow | workflow | edit_50pct-before_ignore | current | rg | wall | -0.137 | -2.15 |
| agentflow | workflow | edit_50pct-delete | current | rg | peak_rss | 3751936.000 | 57.97 |
| agentflow | workflow | edit_50pct-delete | current | rg | wall | -0.111 | -1.97 |
| agentflow | workflow | edit_50pct-dirty | current | rg | peak_rss | 10215424.000 | 156.86 |
| agentflow | workflow | edit_50pct-dirty | current | rg | wall | 6.187 | 61.35 |
| agentflow | workflow | edit_50pct-discard | current | rg | peak_rss | 4030464.000 | 62.28 |
| agentflow | workflow | edit_50pct-discard | current | rg | wall | 0.189 | 2.86 |
| agentflow | workflow | edit_50pct-edit | current | rg | peak_rss | 9936896.000 | 153.54 |
| agentflow | workflow | edit_50pct-edit | current | rg | wall | 7.516 | 90.74 |
| agentflow | workflow | edit_50pct-ignored | current | rg | peak_rss | 3989504.000 | 61.10 |
| agentflow | workflow | edit_50pct-ignored | current | rg | wall | -1.131 | -15.30 |
| agentflow | workflow | edit_50pct-promotion | current | rg | peak_rss | 3940352.000 | 60.81 |
| agentflow | workflow | edit_50pct-promotion | current | rg | wall | 0.853 | 14.17 |
| agentflow | workflow | edit_50pct-rename | current | rg | peak_rss | 3874816.000 | 58.98 |
| agentflow | workflow | edit_50pct-rename | current | rg | wall | -2.324 | -28.52 |
| agentflow | workflow | edit_50pct-revisit | current | rg | peak_rss | 3956736.000 | 60.53 |
| agentflow | workflow | edit_50pct-revisit | current | rg | wall | -0.845 | -11.76 |
| agentflow | workflow | edit_50pct-rollback | current | rg | peak_rss | 3940352.000 | 60.58 |
| agentflow | workflow | edit_50pct-rollback | current | rg | wall | 0.612 | 10.15 |
| agentflow | workflow | edit_50pct-untracked | current | rg | peak_rss | 4005888.000 | 61.98 |
| agentflow | workflow | edit_50pct-untracked | current | rg | wall | -1.936 | -23.40 |
| viberwhisper | branches | edit_1pct-commit_A1 | current | rg | peak_rss | 3760128.000 | 57.38 |
| viberwhisper | branches | edit_1pct-commit_A1 | current | rg | wall | 0.187 | 3.21 |
| viberwhisper | branches | edit_1pct-commit_A2 | current | rg | peak_rss | 3784704.000 | 57.89 |
| viberwhisper | branches | edit_1pct-commit_A2 | current | rg | wall | -0.700 | -9.04 |
| viberwhisper | branches | edit_1pct-commit_B1 | current | rg | peak_rss | 3792896.000 | 57.88 |
| viberwhisper | branches | edit_1pct-commit_B1 | current | rg | wall | -0.676 | -8.66 |
| viberwhisper | branches | edit_1pct-commit_B2 | current | rg | peak_rss | 3940352.000 | 60.58 |
| viberwhisper | branches | edit_1pct-commit_B2 | current | rg | wall | 0.795 | 12.54 |
| viberwhisper | branches | edit_1pct-edit_A1 | current | rg | peak_rss | 4685824.000 | 71.77 |
| viberwhisper | branches | edit_1pct-edit_A1 | current | rg | wall | 0.259 | 3.49 |
| viberwhisper | branches | edit_1pct-edit_A2 | current | rg | peak_rss | 4718592.000 | 72.36 |
| viberwhisper | branches | edit_1pct-edit_A2 | current | rg | wall | 1.316 | 21.67 |
| viberwhisper | branches | edit_1pct-edit_B1 | current | rg | peak_rss | 4620288.000 | 70.32 |
| viberwhisper | branches | edit_1pct-edit_B1 | current | rg | wall | -0.612 | -7.42 |
| viberwhisper | branches | edit_1pct-edit_B2 | current | rg | peak_rss | 4710400.000 | 72.06 |
| viberwhisper | branches | edit_1pct-edit_B2 | current | rg | wall | -0.325 | -4.25 |
| viberwhisper | branches | edit_1pct-revisit_A | current | rg | peak_rss | 3661824.000 | 55.94 |
| viberwhisper | branches | edit_1pct-revisit_A | current | rg | wall | -0.864 | -12.12 |
| viberwhisper | branches | edit_1pct-revisit_B | current | rg | peak_rss | 3694592.000 | 56.45 |
| viberwhisper | branches | edit_1pct-revisit_B | current | rg | wall | 0.984 | 16.77 |
| viberwhisper | branches | edit_1pct-switch_A1 | current | rg | peak_rss | 3104768.000 | 47.37 |
| viberwhisper | branches | edit_1pct-switch_A1 | current | rg | wall | -1.594 | -23.44 |
| viberwhisper | branches | edit_1pct-switch_A2 | current | rg | peak_rss | 3629056.000 | 55.10 |
| viberwhisper | branches | edit_1pct-switch_A2 | current | rg | wall | -0.221 | -3.78 |
| viberwhisper | branches | edit_1pct-switch_B1 | current | rg | peak_rss | 3702784.000 | 57.14 |
| viberwhisper | branches | edit_1pct-switch_B1 | current | rg | wall | -0.386 | -5.86 |
| viberwhisper | branches | edit_1pct-switch_B2 | current | rg | peak_rss | 3719168.000 | 56.82 |
| viberwhisper | branches | edit_1pct-switch_B2 | current | rg | wall | -0.464 | -6.59 |
| viberwhisper | branches | edit_50pct-commit_A1 | current | rg | peak_rss | 3850240.000 | 58.90 |
| viberwhisper | branches | edit_50pct-commit_A1 | current | rg | wall | 0.504 | 7.86 |
| viberwhisper | branches | edit_50pct-commit_A2 | current | rg | peak_rss | 3915776.000 | 59.97 |
| viberwhisper | branches | edit_50pct-commit_A2 | current | rg | wall | 1.000 | 17.00 |
| viberwhisper | branches | edit_50pct-commit_B1 | current | rg | peak_rss | 3858432.000 | 59.32 |
| viberwhisper | branches | edit_50pct-commit_B1 | current | rg | wall | 2.845 | 48.37 |
| viberwhisper | branches | edit_50pct-commit_B2 | current | rg | peak_rss | 3825664.000 | 58.38 |
| viberwhisper | branches | edit_50pct-commit_B2 | current | rg | wall | 0.914 | 13.24 |
| viberwhisper | branches | edit_50pct-edit_A1 | current | rg | peak_rss | 8601600.000 | 131.91 |
| viberwhisper | branches | edit_50pct-edit_A1 | current | rg | wall | 4.754 | 70.43 |
| viberwhisper | branches | edit_50pct-edit_A2 | current | rg | peak_rss | 8937472.000 | 136.03 |
| viberwhisper | branches | edit_50pct-edit_A2 | current | rg | wall | 5.715 | 84.47 |
| viberwhisper | branches | edit_50pct-edit_B1 | current | rg | peak_rss | 8953856.000 | 137.14 |
| viberwhisper | branches | edit_50pct-edit_B1 | current | rg | wall | 2.646 | 32.98 |
| viberwhisper | branches | edit_50pct-edit_B2 | current | rg | peak_rss | 9068544.000 | 138.55 |
| viberwhisper | branches | edit_50pct-edit_B2 | current | rg | wall | 4.075 | 57.71 |
| viberwhisper | branches | edit_50pct-revisit_A | current | rg | peak_rss | 3866624.000 | 58.78 |
| viberwhisper | branches | edit_50pct-revisit_A | current | rg | wall | -0.349 | -4.95 |
| viberwhisper | branches | edit_50pct-revisit_B | current | rg | peak_rss | 3858432.000 | 58.73 |
| viberwhisper | branches | edit_50pct-revisit_B | current | rg | wall | 0.353 | 5.61 |
| viberwhisper | branches | edit_50pct-switch_A1 | current | rg | peak_rss | 3145728.000 | 48.12 |
| viberwhisper | branches | edit_50pct-switch_A1 | current | rg | wall | -1.115 | -18.63 |
| viberwhisper | branches | edit_50pct-switch_A2 | current | rg | peak_rss | 3915776.000 | 59.90 |
| viberwhisper | branches | edit_50pct-switch_A2 | current | rg | wall | 0.375 | 6.15 |
| viberwhisper | branches | edit_50pct-switch_B1 | current | rg | peak_rss | 3842048.000 | 58.70 |
| viberwhisper | branches | edit_50pct-switch_B1 | current | rg | wall | 0.020 | 0.30 |
| viberwhisper | branches | edit_50pct-switch_B2 | current | rg | peak_rss | 3973120.000 | 60.93 |
| viberwhisper | branches | edit_50pct-switch_B2 | current | rg | wall | 0.980 | 17.08 |
| viberwhisper | history | revisit_0 | current | rg | peak_rss | 6955008.000 | 109.27 |
| viberwhisper | history | revisit_0 | current | rg | wall | 4.119 | 69.62 |
| viberwhisper | history | revisit_100 | current | rg | peak_rss | 5431296.000 | 82.26 |
| viberwhisper | history | revisit_100 | current | rg | wall | 5.415 | 77.71 |
| viberwhisper | history | revisit_50 | current | rg | peak_rss | 11051008.000 | 172.95 |
| viberwhisper | history | revisit_50 | current | rg | wall | 11.511 | 183.99 |
| viberwhisper | history | step_001 | current | rg | peak_rss | 4571136.000 | 72.56 |
| viberwhisper | history | step_001 | current | rg | wall | 0.498 | 9.35 |
| viberwhisper | history | step_002 | current | rg | peak_rss | 6471680.000 | 102.07 |
| viberwhisper | history | step_002 | current | rg | wall | 2.352 | 49.51 |
| viberwhisper | history | step_003 | current | rg | peak_rss | 4759552.000 | 74.77 |
| viberwhisper | history | step_003 | current | rg | wall | 0.901 | 17.91 |
| viberwhisper | history | step_004 | current | rg | peak_rss | 5062656.000 | 80.16 |
| viberwhisper | history | step_004 | current | rg | wall | 2.063 | 49.63 |
| viberwhisper | history | step_005 | current | rg | peak_rss | 3989504.000 | 62.92 |
| viberwhisper | history | step_005 | current | rg | wall | 0.746 | 15.29 |
| viberwhisper | history | step_006 | current | rg | peak_rss | 4005888.000 | 62.77 |
| viberwhisper | history | step_006 | current | rg | wall | 1.561 | 38.10 |
| viberwhisper | history | step_007 | current | rg | peak_rss | 5169152.000 | 81.42 |
| viberwhisper | history | step_007 | current | rg | wall | 2.026 | 47.51 |
| viberwhisper | history | step_008 | current | rg | peak_rss | 5939200.000 | 93.43 |
| viberwhisper | history | step_008 | current | rg | wall | 1.623 | 29.75 |
| viberwhisper | history | step_009 | current | rg | peak_rss | 5267456.000 | 83.40 |
| viberwhisper | history | step_009 | current | rg | wall | 2.414 | 62.06 |
| viberwhisper | history | step_010 | current | rg | peak_rss | 5758976.000 | 90.13 |
| viberwhisper | history | step_010 | current | rg | wall | 2.744 | 65.84 |
| viberwhisper | history | step_011 | current | rg | peak_rss | 7200768.000 | 114.16 |
| viberwhisper | history | step_011 | current | rg | wall | 3.510 | 73.50 |
| viberwhisper | history | step_012 | current | rg | peak_rss | 5472256.000 | 86.75 |
| viberwhisper | history | step_012 | current | rg | wall | 1.841 | 38.99 |
| viberwhisper | history | step_013 | current | rg | peak_rss | 4243456.000 | 66.93 |
| viberwhisper | history | step_013 | current | rg | wall | 0.529 | 9.91 |
| viberwhisper | history | step_014 | current | rg | peak_rss | 4841472.000 | 76.06 |
| viberwhisper | history | step_014 | current | rg | wall | 1.928 | 40.47 |
| viberwhisper | history | step_015 | current | rg | peak_rss | 5873664.000 | 93.00 |
| viberwhisper | history | step_015 | current | rg | wall | 2.977 | 65.43 |
| viberwhisper | history | step_016 | current | rg | peak_rss | 5390336.000 | 85.01 |
| viberwhisper | history | step_016 | current | rg | wall | 1.899 | 38.86 |
| viberwhisper | history | step_017 | current | rg | peak_rss | 5144576.000 | 80.41 |
| viberwhisper | history | step_017 | current | rg | wall | 2.707 | 61.71 |
| viberwhisper | history | step_018 | current | rg | peak_rss | 4931584.000 | 77.78 |
| viberwhisper | history | step_018 | current | rg | wall | 2.972 | 80.24 |
| viberwhisper | history | step_019 | current | rg | peak_rss | 7454720.000 | 118.03 |
| viberwhisper | history | step_019 | current | rg | wall | 3.349 | 59.08 |
| viberwhisper | history | step_020 | current | rg | peak_rss | 4890624.000 | 76.64 |
| viberwhisper | history | step_020 | current | rg | wall | 0.105 | 1.73 |
| viberwhisper | history | step_021 | current | rg | peak_rss | 5513216.000 | 86.95 |
| viberwhisper | history | step_021 | current | rg | wall | 3.613 | 103.11 |
| viberwhisper | history | step_022 | current | rg | peak_rss | 5357568.000 | 84.50 |
| viberwhisper | history | step_022 | current | rg | wall | 1.102 | 19.14 |
| viberwhisper | history | step_023 | current | rg | peak_rss | 4562944.000 | 71.87 |
| viberwhisper | history | step_023 | current | rg | wall | 2.033 | 43.13 |
| viberwhisper | history | step_024 | current | rg | peak_rss | 8101888.000 | 128.44 |
| viberwhisper | history | step_024 | current | rg | wall | 4.284 | 74.65 |
| viberwhisper | history | step_025 | current | rg | peak_rss | 6283264.000 | 98.97 |
| viberwhisper | history | step_025 | current | rg | wall | 1.757 | 26.99 |
| viberwhisper | history | step_026 | current | rg | peak_rss | 7733248.000 | 121.34 |
| viberwhisper | history | step_026 | current | rg | wall | 3.948 | 63.80 |
| viberwhisper | history | step_027 | current | rg | peak_rss | 5578752.000 | 88.10 |
| viberwhisper | history | step_027 | current | rg | wall | 1.345 | 23.14 |
| viberwhisper | history | step_028 | current | rg | peak_rss | 6184960.000 | 96.79 |
| viberwhisper | history | step_028 | current | rg | wall | 2.657 | 46.23 |
| viberwhisper | history | step_029 | current | rg | peak_rss | 6676480.000 | 105.16 |
| viberwhisper | history | step_029 | current | rg | wall | 3.173 | 55.15 |
| viberwhisper | history | step_030 | current | rg | peak_rss | 6930432.000 | 109.30 |
| viberwhisper | history | step_030 | current | rg | wall | 1.403 | 21.42 |
| viberwhisper | history | step_031 | current | rg | peak_rss | 7872512.000 | 123.68 |
| viberwhisper | history | step_031 | current | rg | wall | 3.384 | 64.98 |
| viberwhisper | history | step_032 | current | rg | peak_rss | 6422528.000 | 100.90 |
| viberwhisper | history | step_032 | current | rg | wall | 2.474 | 44.57 |
| viberwhisper | history | step_033 | current | rg | peak_rss | 6930432.000 | 108.88 |
| viberwhisper | history | step_033 | current | rg | wall | 2.682 | 44.26 |
| viberwhisper | history | step_034 | current | rg | peak_rss | 6242304.000 | 97.19 |
| viberwhisper | history | step_034 | current | rg | wall | 2.724 | 48.44 |
| viberwhisper | history | step_035 | current | rg | peak_rss | 7561216.000 | 118.49 |
| viberwhisper | history | step_035 | current | rg | wall | 2.985 | 54.46 |
| viberwhisper | history | step_036 | current | rg | peak_rss | 6602752.000 | 104.00 |
| viberwhisper | history | step_036 | current | rg | wall | 1.557 | 23.84 |
| viberwhisper | history | step_037 | current | rg | peak_rss | 6832128.000 | 107.47 |
| viberwhisper | history | step_037 | current | rg | wall | 2.324 | 42.84 |
| viberwhisper | history | step_038 | current | rg | peak_rss | 9330688.000 | 146.21 |
| viberwhisper | history | step_038 | current | rg | wall | 5.213 | 88.91 |
| viberwhisper | history | step_039 | current | rg | peak_rss | 5652480.000 | 89.15 |
| viberwhisper | history | step_039 | current | rg | wall | 2.194 | 43.26 |
| viberwhisper | history | step_040 | current | rg | peak_rss | 7208960.000 | 113.55 |
| viberwhisper | history | step_040 | current | rg | wall | 4.058 | 73.25 |
| viberwhisper | history | step_041 | current | rg | peak_rss | 5767168.000 | 90.03 |
| viberwhisper | history | step_041 | current | rg | wall | 1.264 | 20.80 |
| viberwhisper | history | step_042 | current | rg | peak_rss | 8478720.000 | 133.03 |
| viberwhisper | history | step_042 | current | rg | wall | 5.149 | 88.93 |
| viberwhisper | history | step_043 | current | rg | peak_rss | 9248768.000 | 145.12 |
| viberwhisper | history | step_043 | current | rg | wall | 8.233 | 142.82 |
| viberwhisper | history | step_044 | current | rg | peak_rss | 4694016.000 | 73.37 |
| viberwhisper | history | step_044 | current | rg | wall | 1.561 | 24.21 |
| viberwhisper | history | step_045 | current | rg | peak_rss | 7790592.000 | 122.55 |
| viberwhisper | history | step_045 | current | rg | wall | 2.987 | 48.58 |
| viberwhisper | history | step_046 | current | rg | peak_rss | 8200192.000 | 127.52 |
| viberwhisper | history | step_046 | current | rg | wall | 2.573 | 33.99 |
| viberwhisper | history | step_047 | current | rg | peak_rss | 8511488.000 | 133.03 |
| viberwhisper | history | step_047 | current | rg | wall | 5.546 | 104.69 |
| viberwhisper | history | step_048 | current | rg | peak_rss | 5496832.000 | 86.14 |
| viberwhisper | history | step_048 | current | rg | wall | 2.593 | 42.39 |
| viberwhisper | history | step_049 | current | rg | peak_rss | 6299648.000 | 99.10 |
| viberwhisper | history | step_049 | current | rg | wall | 2.813 | 50.18 |
| viberwhisper | history | step_050 | current | rg | peak_rss | 6692864.000 | 105.15 |
| viberwhisper | history | step_050 | current | rg | wall | 3.232 | 52.12 |
| viberwhisper | history | step_051 | current | rg | peak_rss | 6021120.000 | 94.11 |
| viberwhisper | history | step_051 | current | rg | wall | 2.282 | 40.20 |
| viberwhisper | history | step_052 | current | rg | peak_rss | 8486912.000 | 133.51 |
| viberwhisper | history | step_052 | current | rg | wall | 3.586 | 61.31 |
| viberwhisper | history | step_053 | current | rg | peak_rss | 8527872.000 | 134.15 |
| viberwhisper | history | step_053 | current | rg | wall | 3.006 | 50.04 |
| viberwhisper | history | step_054 | current | rg | peak_rss | 7184384.000 | 112.01 |
| viberwhisper | history | step_054 | current | rg | wall | 4.209 | 86.23 |
| viberwhisper | history | step_055 | current | rg | peak_rss | 6938624.000 | 108.87 |
| viberwhisper | history | step_055 | current | rg | wall | 2.639 | 35.78 |
| viberwhisper | history | step_056 | current | rg | peak_rss | 8478720.000 | 131.85 |
| viberwhisper | history | step_056 | current | rg | wall | 3.876 | 53.37 |
| viberwhisper | history | step_057 | current | rg | peak_rss | 6979584.000 | 108.95 |
| viberwhisper | history | step_057 | current | rg | wall | 0.095 | 1.13 |
| viberwhisper | history | step_058 | current | rg | peak_rss | 8806400.000 | 138.00 |
| viberwhisper | history | step_058 | current | rg | wall | 6.480 | 108.44 |
| viberwhisper | history | step_059 | current | rg | peak_rss | 8945664.000 | 139.64 |
| viberwhisper | history | step_059 | current | rg | wall | 4.958 | 74.20 |
| viberwhisper | history | step_060 | current | rg | peak_rss | 9486336.000 | 149.42 |
| viberwhisper | history | step_060 | current | rg | wall | 7.655 | 116.35 |
| viberwhisper | history | step_061 | current | rg | peak_rss | 9199616.000 | 144.16 |
| viberwhisper | history | step_061 | current | rg | wall | 4.200 | 46.64 |
| viberwhisper | history | step_062 | current | rg | peak_rss | 8404992.000 | 132.39 |
| viberwhisper | history | step_062 | current | rg | wall | 5.209 | 77.56 |
| viberwhisper | history | step_063 | current | rg | peak_rss | 8724480.000 | 136.71 |
| viberwhisper | history | step_063 | current | rg | wall | 4.665 | 58.56 |
| viberwhisper | history | step_064 | current | rg | peak_rss | 8978432.000 | 140.33 |
| viberwhisper | history | step_064 | current | rg | wall | 4.445 | 52.72 |
| viberwhisper | history | step_065 | current | rg | peak_rss | 11935744.000 | 187.03 |
| viberwhisper | history | step_065 | current | rg | wall | 9.587 | 148.11 |
| viberwhisper | history | step_066 | current | rg | peak_rss | 9420800.000 | 147.63 |
| viberwhisper | history | step_066 | current | rg | wall | 7.608 | 125.35 |
| viberwhisper | history | step_067 | current | rg | peak_rss | 16629760.000 | 261.60 |
| viberwhisper | history | step_067 | current | rg | wall | 94.974 | 1470.36 |
| viberwhisper | history | step_068 | current | rg | peak_rss | 5652480.000 | 88.92 |
| viberwhisper | history | step_068 | current | rg | wall | 1.262 | 15.23 |
| viberwhisper | history | step_069 | current | rg | peak_rss | 4972544.000 | 77.72 |
| viberwhisper | history | step_069 | current | rg | wall | -0.037 | -0.47 |
| viberwhisper | history | step_070 | current | rg | peak_rss | 5652480.000 | 88.35 |
| viberwhisper | history | step_070 | current | rg | wall | -2.307 | -22.35 |
| viberwhisper | history | step_071 | current | rg | peak_rss | 6283264.000 | 99.48 |
| viberwhisper | history | step_071 | current | rg | wall | 1.228 | 15.10 |
| viberwhisper | history | step_072 | current | rg | peak_rss | 6184960.000 | 97.29 |
| viberwhisper | history | step_072 | current | rg | wall | 0.149 | 1.85 |
| viberwhisper | history | step_073 | current | rg | peak_rss | 6176768.000 | 97.04 |
| viberwhisper | history | step_073 | current | rg | wall | 2.508 | 38.53 |
| viberwhisper | history | step_074 | current | rg | peak_rss | 7962624.000 | 123.98 |
| viberwhisper | history | step_074 | current | rg | wall | 3.913 | 42.71 |
| viberwhisper | history | step_075 | current | rg | peak_rss | 6725632.000 | 104.72 |
| viberwhisper | history | step_075 | current | rg | wall | 2.928 | 46.59 |
| viberwhisper | history | step_076 | current | rg | peak_rss | 9404416.000 | 147.18 |
| viberwhisper | history | step_076 | current | rg | wall | 7.560 | 92.82 |
| viberwhisper | history | step_077 | current | rg | peak_rss | 7012352.000 | 109.32 |
| viberwhisper | history | step_077 | current | rg | wall | 1.113 | 14.14 |
| viberwhisper | history | step_078 | current | rg | peak_rss | 9551872.000 | 150.45 |
| viberwhisper | history | step_078 | current | rg | wall | 5.567 | 63.60 |
| viberwhisper | history | step_079 | current | rg | peak_rss | 8650752.000 | 135.21 |
| viberwhisper | history | step_079 | current | rg | wall | 6.864 | 101.49 |
| viberwhisper | history | step_080 | current | rg | peak_rss | 5513216.000 | 87.52 |
| viberwhisper | history | step_080 | current | rg | wall | 1.419 | 20.48 |
| viberwhisper | history | step_081 | current | rg | peak_rss | 6766592.000 | 105.76 |
| viberwhisper | history | step_081 | current | rg | wall | 3.884 | 55.27 |
| viberwhisper | history | step_082 | current | rg | peak_rss | 7110656.000 | 111.71 |
| viberwhisper | history | step_082 | current | rg | wall | 2.189 | 32.29 |
| viberwhisper | history | step_083 | current | rg | peak_rss | 7495680.000 | 117.61 |
| viberwhisper | history | step_083 | current | rg | wall | 3.809 | 64.05 |
| viberwhisper | history | step_084 | current | rg | peak_rss | 8781824.000 | 138.32 |
| viberwhisper | history | step_084 | current | rg | wall | 5.866 | 71.84 |
| viberwhisper | history | step_085 | current | rg | peak_rss | 9617408.000 | 149.36 |
| viberwhisper | history | step_085 | current | rg | wall | 2.958 | 39.44 |
| viberwhisper | history | step_086 | current | rg | peak_rss | 8101888.000 | 127.28 |
| viberwhisper | history | step_086 | current | rg | wall | 3.175 | 48.52 |
| viberwhisper | history | step_087 | current | rg | peak_rss | 5898240.000 | 91.95 |
| viberwhisper | history | step_087 | current | rg | wall | 0.846 | 11.08 |
| viberwhisper | history | step_088 | current | rg | peak_rss | 7716864.000 | 121.55 |
| viberwhisper | history | step_088 | current | rg | wall | 3.320 | 49.00 |
| viberwhisper | history | step_089 | current | rg | peak_rss | 8364032.000 | 130.90 |
| viberwhisper | history | step_089 | current | rg | wall | 3.834 | 63.66 |
| viberwhisper | history | step_090 | current | rg | peak_rss | 8372224.000 | 131.19 |
| viberwhisper | history | step_090 | current | rg | wall | 1.141 | 11.65 |
| viberwhisper | history | step_091 | current | rg | peak_rss | 6766592.000 | 105.49 |
| viberwhisper | history | step_091 | current | rg | wall | 2.721 | 38.51 |
| viberwhisper | history | step_092 | current | rg | peak_rss | 8609792.000 | 135.26 |
| viberwhisper | history | step_092 | current | rg | wall | 3.733 | 42.16 |
| viberwhisper | history | step_093 | current | rg | peak_rss | 6660096.000 | 104.77 |
| viberwhisper | history | step_093 | current | rg | wall | 1.830 | 27.94 |
| viberwhisper | history | step_094 | current | rg | peak_rss | 7602176.000 | 119.28 |
| viberwhisper | history | step_094 | current | rg | wall | 3.571 | 44.45 |
| viberwhisper | history | step_095 | current | rg | peak_rss | 9986048.000 | 155.88 |
| viberwhisper | history | step_095 | current | rg | wall | 9.044 | 146.28 |
| viberwhisper | history | step_096 | current | rg | peak_rss | 6234112.000 | 97.31 |
| viberwhisper | history | step_096 | current | rg | wall | 1.572 | 21.08 |
| viberwhisper | history | step_097 | current | rg | peak_rss | 11378688.000 | 174.94 |
| viberwhisper | history | step_097 | current | rg | wall | 13.854 | 169.65 |
| viberwhisper | history | step_098 | current | rg | peak_rss | 8265728.000 | 126.44 |
| viberwhisper | history | step_098 | current | rg | wall | 7.331 | 90.18 |
| viberwhisper | history | step_099 | current | rg | peak_rss | 7692288.000 | 116.94 |
| viberwhisper | history | step_099 | current | rg | wall | 1.231 | 14.43 |
| viberwhisper | history | step_100 | current | rg | peak_rss | 5480448.000 | 83.94 |
| viberwhisper | history | step_100 | current | rg | wall | 1.748 | 27.14 |
| viberwhisper | search | history_0-absent | current | rg | peak_rss | 3096576.000 | 48.71 |
| viberwhisper | search | history_0-absent | current | rg | wall | -0.749 | -16.06 |
| viberwhisper | search | history_0-anchor | current | rg | peak_rss | 3653632.000 | 52.10 |
| viberwhisper | search | history_0-anchor | current | rg | wall | -1.046 | -18.18 |
| viberwhisper | search | history_0-blank | current | rg | peak_rss | 3088384.000 | 46.54 |
| viberwhisper | search | history_0-blank | current | rg | wall | -0.316 | -6.59 |
| viberwhisper | search | history_0-broad | current | rg | peak_rss | 3145728.000 | 49.23 |
| viberwhisper | search | history_0-broad | current | rg | wall | -0.494 | -10.64 |
| viberwhisper | search | history_0-count | current | rg | peak_rss | 3112960.000 | 46.68 |
| viberwhisper | search | history_0-count | current | rg | wall | -0.219 | -4.57 |
| viberwhisper | search | history_0-icase | current | rg | peak_rss | 3063808.000 | 46.00 |
| viberwhisper | search | history_0-icase | current | rg | wall | -0.708 | -14.62 |
| viberwhisper | search | history_0-icase_literal | current | rg | peak_rss | 3203072.000 | 49.87 |
| viberwhisper | search | history_0-icase_literal | current | rg | wall | -0.809 | -17.10 |
| viberwhisper | search | history_0-literal | current | rg | peak_rss | 3137536.000 | 49.10 |
| viberwhisper | search | history_0-literal | current | rg | wall | -0.807 | -17.27 |
| viberwhisper | search | history_0-literal_lines | current | rg | peak_rss | 3121152.000 | 48.66 |
| viberwhisper | search | history_0-literal_lines | current | rg | wall | -0.497 | -10.64 |
| viberwhisper | search | history_0-or | current | rg | peak_rss | 3170304.000 | 47.43 |
| viberwhisper | search | history_0-or | current | rg | wall | -0.625 | -12.92 |
| viberwhisper | search | history_0-short | current | rg | peak_rss | 3112960.000 | 48.78 |
| viberwhisper | search | history_0-short | current | rg | wall | -0.299 | -6.50 |
| viberwhisper | search | history_0-word | current | rg | peak_rss | 3022848.000 | 45.78 |
| viberwhisper | search | history_0-word | current | rg | wall | -0.665 | -13.95 |
| viberwhisper | search | history_100-absent | current | rg | peak_rss | 4505600.000 | 68.92 |
| viberwhisper | search | history_100-absent | current | rg | wall | -0.641 | -11.06 |
| viberwhisper | search | history_100-anchor | current | rg | peak_rss | 11190272.000 | 140.10 |
| viberwhisper | search | history_100-anchor | current | rg | wall | 0.707 | 9.28 |
| viberwhisper | search | history_100-blank | current | rg | peak_rss | 5193728.000 | 72.79 |
| viberwhisper | search | history_100-blank | current | rg | wall | 2.529 | 34.85 |
| viberwhisper | search | history_100-broad | current | rg | peak_rss | 5513216.000 | 81.67 |
| viberwhisper | search | history_100-broad | current | rg | wall | -0.725 | -10.23 |
| viberwhisper | search | history_100-count | current | rg | peak_rss | 4399104.000 | 60.47 |
| viberwhisper | search | history_100-count | current | rg | wall | 0.004 | 0.05 |
| viberwhisper | search | history_100-icase | current | rg | peak_rss | 8257536.000 | 120.29 |
| viberwhisper | search | history_100-icase | current | rg | wall | 0.173 | 2.94 |
| viberwhisper | search | history_100-icase_literal | current | rg | peak_rss | 7569408.000 | 114.36 |
| viberwhisper | search | history_100-icase_literal | current | rg | wall | -0.545 | -8.89 |
| viberwhisper | search | history_100-literal | current | rg | peak_rss | 5439488.000 | 83.00 |
| viberwhisper | search | history_100-literal | current | rg | wall | -0.310 | -5.34 |
| viberwhisper | search | history_100-literal_lines | current | rg | peak_rss | 5734400.000 | 84.64 |
| viberwhisper | search | history_100-literal_lines | current | rg | wall | -0.292 | -4.49 |
| viberwhisper | search | history_100-or | current | rg | peak_rss | 7913472.000 | 114.18 |
| viberwhisper | search | history_100-or | current | rg | wall | -0.002 | -0.03 |
| viberwhisper | search | history_100-short | current | rg | peak_rss | 4587520.000 | 67.80 |
| viberwhisper | search | history_100-short | current | rg | wall | -0.457 | -6.62 |
| viberwhisper | search | history_100-word | current | rg | peak_rss | 5726208.000 | 82.92 |
| viberwhisper | search | history_100-word | current | rg | wall | -0.479 | -7.11 |
| viberwhisper | search | history_25-absent | current | rg | peak_rss | 3899392.000 | 61.50 |
| viberwhisper | search | history_25-absent | current | rg | wall | -0.455 | -9.09 |
| viberwhisper | search | history_25-anchor | current | rg | peak_rss | 5283840.000 | 68.69 |
| viberwhisper | search | history_25-anchor | current | rg | wall | -0.579 | -8.66 |
| viberwhisper | search | history_25-blank | current | rg | peak_rss | 3940352.000 | 57.88 |
| viberwhisper | search | history_25-blank | current | rg | wall | 0.556 | 9.61 |
| viberwhisper | search | history_25-broad | current | rg | peak_rss | 4276224.000 | 66.75 |
| viberwhisper | search | history_25-broad | current | rg | wall | -0.530 | -9.33 |
| viberwhisper | search | history_25-count | current | rg | peak_rss | 3801088.000 | 55.57 |
| viberwhisper | search | history_25-count | current | rg | wall | -0.093 | -1.71 |
| viberwhisper | search | history_25-icase | current | rg | peak_rss | 4972544.000 | 73.58 |
| viberwhisper | search | history_25-icase | current | rg | wall | 0.020 | 0.39 |
| viberwhisper | search | history_25-icase_literal | current | rg | peak_rss | 5029888.000 | 77.72 |
| viberwhisper | search | history_25-icase_literal | current | rg | wall | -0.132 | -2.63 |
| viberwhisper | search | history_25-literal | current | rg | peak_rss | 4235264.000 | 66.45 |
| viberwhisper | search | history_25-literal | current | rg | wall | -0.127 | -2.66 |
| viberwhisper | search | history_25-literal_lines | current | rg | peak_rss | 4374528.000 | 67.34 |
| viberwhisper | search | history_25-literal_lines | current | rg | wall | -0.385 | -7.16 |
| viberwhisper | search | history_25-or | current | rg | peak_rss | 4915200.000 | 72.73 |
| viberwhisper | search | history_25-or | current | rg | wall | -0.180 | -3.32 |
| viberwhisper | search | history_25-short | current | rg | peak_rss | 3948544.000 | 60.86 |
| viberwhisper | search | history_25-short | current | rg | wall | -0.397 | -7.19 |
| viberwhisper | search | history_25-word | current | rg | peak_rss | 4374528.000 | 65.60 |
| viberwhisper | search | history_25-word | current | rg | wall | -0.225 | -4.08 |
| viberwhisper | search | history_50-absent | current | rg | peak_rss | 4153344.000 | 64.92 |
| viberwhisper | search | history_50-absent | current | rg | wall | -0.667 | -12.10 |
| viberwhisper | search | history_50-anchor | current | rg | peak_rss | 7356416.000 | 93.93 |
| viberwhisper | search | history_50-anchor | current | rg | wall | -0.685 | -8.92 |
| viberwhisper | search | history_50-blank | current | rg | peak_rss | 4243456.000 | 62.33 |
| viberwhisper | search | history_50-blank | current | rg | wall | 0.450 | 6.80 |
| viberwhisper | search | history_50-broad | current | rg | peak_rss | 4734976.000 | 73.44 |
| viberwhisper | search | history_50-broad | current | rg | wall | -0.413 | -7.19 |
| viberwhisper | search | history_50-count | current | rg | peak_rss | 3948544.000 | 56.57 |
| viberwhisper | search | history_50-count | current | rg | wall | 0.141 | 2.38 |
| viberwhisper | search | history_50-icase | current | rg | peak_rss | 6266880.000 | 92.50 |
| viberwhisper | search | history_50-icase | current | rg | wall | -0.028 | -0.52 |
| viberwhisper | search | history_50-icase_literal | current | rg | peak_rss | 6201344.000 | 96.68 |
| viberwhisper | search | history_50-icase_literal | current | rg | wall | -0.948 | -15.79 |
| viberwhisper | search | history_50-literal | current | rg | peak_rss | 4866048.000 | 76.45 |
| viberwhisper | search | history_50-literal | current | rg | wall | -0.535 | -9.68 |
| viberwhisper | search | history_50-literal_lines | current | rg | peak_rss | 4964352.000 | 75.37 |
| viberwhisper | search | history_50-literal_lines | current | rg | wall | -0.290 | -5.17 |
| viberwhisper | search | history_50-or | current | rg | peak_rss | 6111232.000 | 90.31 |
| viberwhisper | search | history_50-or | current | rg | wall | -0.277 | -4.50 |
| viberwhisper | search | history_50-short | current | rg | peak_rss | 4112384.000 | 63.22 |
| viberwhisper | search | history_50-short | current | rg | wall | -0.426 | -7.04 |
| viberwhisper | search | history_50-word | current | rg | peak_rss | 4988928.000 | 74.72 |
| viberwhisper | search | history_50-word | current | rg | wall | -0.133 | -2.33 |
| viberwhisper | search | history_75-absent | current | rg | peak_rss | 3563520.000 | 55.84 |
| viberwhisper | search | history_75-absent | current | rg | wall | -1.322 | -21.76 |
| viberwhisper | search | history_75-anchor | current | rg | peak_rss | 5103616.000 | 64.16 |
| viberwhisper | search | history_75-anchor | current | rg | wall | -0.852 | -11.23 |
| viberwhisper | search | history_75-blank | current | rg | peak_rss | 4186112.000 | 60.19 |
| viberwhisper | search | history_75-blank | current | rg | wall | 1.271 | 17.23 |
| viberwhisper | search | history_75-broad | current | rg | peak_rss | 3964928.000 | 60.65 |
| viberwhisper | search | history_75-broad | current | rg | wall | -1.488 | -20.51 |
| viberwhisper | search | history_75-count | current | rg | peak_rss | 3424256.000 | 47.45 |
| viberwhisper | search | history_75-count | current | rg | wall | -1.147 | -15.00 |
| viberwhisper | search | history_75-icase | current | rg | peak_rss | 4325376.000 | 63.46 |
| viberwhisper | search | history_75-icase | current | rg | wall | -1.189 | -19.21 |
| viberwhisper | search | history_75-icase_literal | current | rg | peak_rss | 4317184.000 | 66.88 |
| viberwhisper | search | history_75-icase_literal | current | rg | wall | -0.899 | -15.47 |
| viberwhisper | search | history_75-literal | current | rg | peak_rss | 3883008.000 | 60.85 |
| viberwhisper | search | history_75-literal | current | rg | wall | -1.756 | -27.22 |
| viberwhisper | search | history_75-literal_lines | current | rg | peak_rss | 4087808.000 | 61.53 |
| viberwhisper | search | history_75-literal_lines | current | rg | wall | -0.624 | -9.93 |
| viberwhisper | search | history_75-or | current | rg | peak_rss | 4481024.000 | 65.35 |
| viberwhisper | search | history_75-or | current | rg | wall | -0.997 | -14.85 |
| viberwhisper | search | history_75-short | current | rg | peak_rss | 3776512.000 | 57.48 |
| viberwhisper | search | history_75-short | current | rg | wall | -1.255 | -17.43 |
| viberwhisper | search | history_75-word | current | rg | peak_rss | 4030464.000 | 59.13 |
| viberwhisper | search | history_75-word | current | rg | wall | -0.835 | -12.94 |
| viberwhisper | search | initial-absent | current | rg | peak_rss | 3178496.000 | 48.81 |
| viberwhisper | search | initial-absent | current | rg | wall | -1.129 | -18.61 |
| viberwhisper | search | initial-anchor | current | rg | peak_rss | 4169728.000 | 52.31 |
| viberwhisper | search | initial-anchor | current | rg | wall | -1.503 | -18.21 |
| viberwhisper | search | initial-blank | current | rg | peak_rss | 4325376.000 | 61.04 |
| viberwhisper | search | initial-blank | current | rg | wall | 2.168 | 27.25 |
| viberwhisper | search | initial-broad | current | rg | peak_rss | 3833856.000 | 57.42 |
| viberwhisper | search | initial-broad | current | rg | wall | -1.700 | -21.93 |
| viberwhisper | search | initial-count | current | rg | peak_rss | 3301376.000 | 44.78 |
| viberwhisper | search | initial-count | current | rg | wall | -0.756 | -10.14 |
| viberwhisper | search | initial-icase | current | rg | peak_rss | 3244032.000 | 47.20 |
| viberwhisper | search | initial-icase | current | rg | wall | -1.625 | -24.58 |
| viberwhisper | search | initial-icase_literal | current | rg | peak_rss | 3358720.000 | 50.74 |
| viberwhisper | search | initial-icase_literal | current | rg | wall | -1.456 | -21.98 |
| viberwhisper | search | initial-literal | current | rg | peak_rss | 3293184.000 | 49.94 |
| viberwhisper | search | initial-literal | current | rg | wall | -1.667 | -24.70 |
| viberwhisper | search | initial-literal_lines | current | rg | peak_rss | 3538944.000 | 52.17 |
| viberwhisper | search | initial-literal_lines | current | rg | wall | -1.486 | -19.83 |
| viberwhisper | search | initial-or | current | rg | peak_rss | 3997696.000 | 57.62 |
| viberwhisper | search | initial-or | current | rg | wall | -1.530 | -19.20 |
| viberwhisper | search | initial-short | current | rg | peak_rss | 3694592.000 | 54.87 |
| viberwhisper | search | initial-short | current | rg | wall | -0.996 | -13.56 |
| viberwhisper | search | initial-word | current | rg | peak_rss | 3620864.000 | 52.93 |
| viberwhisper | search | initial-word | current | rg | wall | -1.100 | -15.51 |
| viberwhisper | workflow | edit_1pct-add_commit | current | rg | peak_rss | 3858432.000 | 58.95 |
| viberwhisper | workflow | edit_1pct-add_commit | current | rg | wall | 0.371 | 6.05 |
| viberwhisper | workflow | edit_1pct-before_ignore | current | rg | peak_rss | 3874816.000 | 59.50 |
| viberwhisper | workflow | edit_1pct-before_ignore | current | rg | wall | -1.447 | -18.65 |
| viberwhisper | workflow | edit_1pct-delete | current | rg | peak_rss | 3653632.000 | 56.38 |
| viberwhisper | workflow | edit_1pct-delete | current | rg | wall | -1.421 | -19.79 |
| viberwhisper | workflow | edit_1pct-dirty | current | rg | peak_rss | 4685824.000 | 71.32 |
| viberwhisper | workflow | edit_1pct-dirty | current | rg | wall | -0.796 | -10.32 |
| viberwhisper | workflow | edit_1pct-discard | current | rg | peak_rss | 3678208.000 | 56.05 |
| viberwhisper | workflow | edit_1pct-discard | current | rg | wall | -0.177 | -2.97 |
| viberwhisper | workflow | edit_1pct-edit | current | rg | peak_rss | 4677632.000 | 71.46 |
| viberwhisper | workflow | edit_1pct-edit | current | rg | wall | -0.763 | -10.44 |
| viberwhisper | workflow | edit_1pct-ignored | current | rg | peak_rss | 3899392.000 | 59.28 |
| viberwhisper | workflow | edit_1pct-ignored | current | rg | wall | 0.437 | 7.45 |
| viberwhisper | workflow | edit_1pct-promotion | current | rg | peak_rss | 3825664.000 | 58.45 |
| viberwhisper | workflow | edit_1pct-promotion | current | rg | wall | 2.305 | 34.99 |
| viberwhisper | workflow | edit_1pct-rename | current | rg | peak_rss | 3883008.000 | 59.62 |
| viberwhisper | workflow | edit_1pct-rename | current | rg | wall | 0.496 | 8.52 |
| viberwhisper | workflow | edit_1pct-revisit | current | rg | peak_rss | 3661824.000 | 55.74 |
| viberwhisper | workflow | edit_1pct-revisit | current | rg | wall | -0.794 | -12.26 |
| viberwhisper | workflow | edit_1pct-rollback | current | rg | peak_rss | 3620864.000 | 55.18 |
| viberwhisper | workflow | edit_1pct-rollback | current | rg | wall | -0.049 | -0.83 |
| viberwhisper | workflow | edit_1pct-untracked | current | rg | peak_rss | 3874816.000 | 59.50 |
| viberwhisper | workflow | edit_1pct-untracked | current | rg | wall | -2.384 | -27.94 |
| viberwhisper | workflow | edit_50pct-add_commit | current | rg | peak_rss | 3858432.000 | 58.80 |
| viberwhisper | workflow | edit_50pct-add_commit | current | rg | wall | 0.334 | 5.80 |
| viberwhisper | workflow | edit_50pct-before_ignore | current | rg | peak_rss | 3833856.000 | 58.79 |
| viberwhisper | workflow | edit_50pct-before_ignore | current | rg | wall | -0.672 | -10.19 |
| viberwhisper | workflow | edit_50pct-delete | current | rg | peak_rss | 3629056.000 | 55.10 |
| viberwhisper | workflow | edit_50pct-delete | current | rg | wall | -1.085 | -15.87 |
| viberwhisper | workflow | edit_50pct-dirty | current | rg | peak_rss | 9347072.000 | 143.34 |
| viberwhisper | workflow | edit_50pct-dirty | current | rg | wall | 1.642 | 17.20 |
| viberwhisper | workflow | edit_50pct-discard | current | rg | peak_rss | 3964928.000 | 60.65 |
| viberwhisper | workflow | edit_50pct-discard | current | rg | wall | 0.481 | 8.12 |
| viberwhisper | workflow | edit_50pct-edit | current | rg | peak_rss | 8749056.000 | 133.33 |
| viberwhisper | workflow | edit_50pct-edit | current | rg | wall | 5.351 | 73.92 |
| viberwhisper | workflow | edit_50pct-ignored | current | rg | peak_rss | 3899392.000 | 59.43 |
| viberwhisper | workflow | edit_50pct-ignored | current | rg | wall | -0.375 | -5.34 |
| viberwhisper | workflow | edit_50pct-promotion | current | rg | peak_rss | 3817472.000 | 58.03 |
| viberwhisper | workflow | edit_50pct-promotion | current | rg | wall | -0.400 | -5.69 |
| viberwhisper | workflow | edit_50pct-rename | current | rg | peak_rss | 3874816.000 | 59.42 |
| viberwhisper | workflow | edit_50pct-rename | current | rg | wall | -1.554 | -19.55 |
| viberwhisper | workflow | edit_50pct-revisit | current | rg | peak_rss | 3883008.000 | 59.32 |
| viberwhisper | workflow | edit_50pct-revisit | current | rg | wall | -1.235 | -17.22 |
| viberwhisper | workflow | edit_50pct-rollback | current | rg | peak_rss | 3891200.000 | 59.60 |
| viberwhisper | workflow | edit_50pct-rollback | current | rg | wall | 0.017 | 0.28 |
| viberwhisper | workflow | edit_50pct-untracked | current | rg | peak_rss | 3874816.000 | 58.98 |
| viberwhisper | workflow | edit_50pct-untracked | current | rg | wall | -0.963 | -12.33 |
