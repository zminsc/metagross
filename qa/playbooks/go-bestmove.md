# go and bestmove

## Steps and expected output
1. `printf 'position startpos\ngo\nquit\n' | $BIN` prints exactly one line `bestmove <uci move>`, a move from the 20 legal opening moves.
2. Each of `go movetime 100`, `go depth 3`, `go wtime 1000 btime 1000`, `go infinite` returns exactly one `bestmove` line and does not hang (wrap in `timeout 5`). Record `go infinite` result: it is allowed to return immediately for a random engine.
3. Two `go` commands in a row produce two bestmove lines.
4. `python3 qa/legality.py $BIN 50` exits 0 (`illegal 0`).

## Pass
Exactly one well-formed bestmove per go, always legal, never a hang.
