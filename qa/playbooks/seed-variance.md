# Seed variance

Expected to FAIL until the seed fix lands (seed is currently whole seconds).

## Steps and expected output
1. Same-second runs: `for i in 1 2 3 4 5 6; do printf 'position startpos\ngo\nquit\n' | $BIN; done`. The 6 lines must not all be identical. With 20 opening moves the chance of 6 identical by luck is 20^-5, so identical means a seed defect.
2. Larger sample: `for i in $(seq 200); do printf 'position startpos\ngo\nquit\n' | $BIN; done | sort | uniq -c`. Expect at least 15 distinct moves, and no move above 15 percent (30 of 200).
3. Different positions still differ: run step 2 on `position startpos moves e2e4`.
4. Full e2e: `for i in 1 2 3 4 5 6; do printf 'uci\nisready\nposition startpos moves e2e4\ngo\nquit\n' | $BIN | tail -1; done` must not be 6 identical lines.

## Pass
Steps 1, 2 and 4 meet the thresholds. Record the distribution in the report.
