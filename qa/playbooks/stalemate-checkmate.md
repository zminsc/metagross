# Terminal positions

## Steps and expected output
1. Stalemate, black to move: `printf 'position fen 7k/5Q2/6K1/8/8/8/8/8 b - - 0 1\ngo\nquit\n' | $BIN` prints `bestmove 0000`.
2. Fool's mate, white is checkmated: `printf 'position startpos moves f2f3 e7e5 g2g4 d8h4\ngo\nquit\n' | $BIN` prints `bestmove 0000`.
3. Scholar's mate, black is checkmated: `printf 'position startpos moves e2e4 e7e5 f1c4 b8c6 d1h5 g8f6 h5f7\ngo\nquit\n' | $BIN` prints `bestmove 0000`.
4. After a terminal position, `position startpos` then `go` returns a normal legal move (session recovers).

## Pass
All terminal positions print `bestmove 0000` and exit cleanly; step 4 returns a legal move.
