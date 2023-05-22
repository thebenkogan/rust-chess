import chess
import json

FEN = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"

test_positions = json.loads(open("test-data/positions.json").read())


def uci_to_coord(uci):
    return (ord(uci[0]) - 97, int(uci[1]) - 1)


board = chess.Board(FEN)
moves = list(board.legal_moves)

expected = []
for move in moves:
    uci = move.uci()
    move = uci[:4]
    promotion = uci[4] if len(uci) == 5 else None
    move = {
        "from": uci_to_coord(move[:2]),
        "to": uci_to_coord(move[2:]),
        "promotion": promotion,
    }
    expected.append(move)

position = {
    "fen": FEN,
    "moves": expected,
}

test_positions.append(position)

json.dump(test_positions, open("test-data/positions.json", "w"))
