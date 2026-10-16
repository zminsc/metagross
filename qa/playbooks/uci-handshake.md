# UCI handshake

Setup: `cargo build` then `BIN=target/debug/metagross`.

## Steps and expected output
1. `printf 'uci\nquit\n' | $BIN` prints exactly `id name metagross`, `id author metagross team`, `uciok` in that order, exit code 0.
2. `printf 'uci\nisready\nquit\n' | $BIN` ends with `readyok` after `uciok`.
3. `printf 'isready\nquit\n' | $BIN` prints only `readyok`.
4. `printf 'ucinewgame\nisready\nquit\n' | $BIN` prints only `readyok` (ucinewgame is silent).
5. `printf 'uci\r\nisready\r\n' | $BIN | od -c` has no `\r` in the output (CRLF input accepted).
6. `printf 'uci\n' | $BIN; echo rc=$?` exits 0 on EOF without `quit`.
7. `printf 'quit\nuci\n' | $BIN` prints nothing: nothing is processed after `quit`.

## Pass
All seven match. Any extra output line, missing newline, or nonzero exit is a defect.
