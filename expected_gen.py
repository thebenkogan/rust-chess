import chess
import json

FEN = "8/6bb/8/8/R1pP2k1/4P3/P7/K7 b - d3"

test_positions = json.loads(open("test-data/positions.json").read())


def uci_to_coord(uci):
    return (ord(uci[0]) - 97, int(uci[1]) - 1)


board = chess.Board(FEN)
moves = list(board.legal_moves)

expected = []
for move in moves:
    uci = move.uci()[:4]
    move = {"from": uci_to_coord(uci[:2]), "to": uci_to_coord(uci[2:])}
    expected.append(move)

position = {
    "fen": FEN,
    "moves": expected,
}

test_positions.append(position)

json.dump(test_positions, open("test-data/positions.json", "w"))
