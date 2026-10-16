import subprocess, sys, random, chess
BIN = sys.argv[1]
GAMES = int(sys.argv[2]) if len(sys.argv) > 2 else 20
bad = 0
for g in range(GAMES):
    board = chess.Board()
    moves = []
    for ply in range(200):
        if board.is_game_over():
            break
        cmd = "position startpos" + (" moves " + " ".join(moves) if moves else "") + "\ngo\nquit\n"
        out = subprocess.run([BIN], input=cmd, capture_output=True, text=True).stdout.split()
        bm = out[out.index("bestmove") + 1]
        if chess.Move.from_uci(bm) not in board.legal_moves:
            print("ILLEGAL", bm, "after", " ".join(moves)); bad += 1; break
        board.push_uci(bm); moves.append(bm)
print("games", GAMES, "illegal", bad)
sys.exit(1 if bad else 0)
