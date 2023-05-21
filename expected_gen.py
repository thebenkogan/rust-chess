import chess
import json

FEN = "8/8/8/2k5/3Pp3/8/8/4K3 b - d3 0 1"

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
