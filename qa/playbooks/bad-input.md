# Bad input

Contract: the engine never panics, never exits nonzero, and keeps answering.

## Steps and expected output
1. `printf 'bogus\nisready\nquit\n' | $BIN; echo rc=$?` prints `readyok`, rc=0.
2. Empty and whitespace-only lines, plus `  isready  `, are accepted: `printf '\n   \n  isready  \nquit\n' | $BIN` prints `readyok`.
3. Illegal move: `printf 'position startpos moves e2e5\ngo\nquit\n' | $BIN` prints a legal bestmove for white from the start position and rc=0.
4. Partial application, current behavior: `printf 'position startpos moves e2e4 e2e4\ngo\nquit\n' | $BIN` yields a legal black move (the first move was applied, the second rejected). Flag if this changes without a spec change.
5. Garbage FEN: `printf 'position fen garbage\nisready\nquit\n' | $BIN` prints `readyok`, rc=0, previous position retained.
6. `printf 'position\ngo\nquit\n' | $BIN`, `printf 'position startpos moves\ngo\nquit\n' | $BIN`, `printf 'go movetime abc\nquit\n' | $BIN`: no panic, rc=0.
7. Long line: `python3 -c "print('position startpos moves ' + 'e2e4 '*100000)" | $BIN`: no panic, rc=0.
8. Check stderr is empty for steps 1 to 7.

## Pass
rc=0 everywhere, no panic text on stderr, engine still answers `isready` afterwards. Silent ignoring is current behavior; an `info string` error would be an improvement, not required.
