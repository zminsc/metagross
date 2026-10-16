# position startpos and moves

## Steps and expected output
1. `printf 'position startpos moves e2e4 e7e5\ngo\nquit\n' | $BIN` prints one `bestmove` that is legal for white after 1.e4 e5 (white to move).
2. `printf 'position startpos moves e2e4\ngo\nquit\n' | $BIN` prints a legal black move.
3. FEN: `printf 'position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\ngo\nquit\n' | $BIN` prints a legal white opening move.
4. `go` with no prior `position` behaves as startpos: prints a legal white opening move.
5. Promotion: `printf 'position fen 8/P6k/8/8/8/8/8/K7 w - - 0 1\ngo\nquit\n' | $BIN` run 10 times with `sleep 1` between runs (until the seed fix lands, same-second runs repeat one move) must only produce moves from the set `a7a8q a7a8r a7a8b a7a8n a1a2 a1b1 a1b2`. Any other move is a defect.
6. Castling and en passant are covered by `cargo test` contract tests; cross-check with `python3 qa/legality.py $BIN 20`, which plays 20 random-vs-engine games against python-chess and fails on any illegal bestmove.

## Pass
All bestmoves are legal for the side to move; the legality script exits 0.
