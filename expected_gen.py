import chess
import json

FEN = "8/k1P5/8/1K6/8/8/8/8 w - - 0 1"

test_positions = json.loads(open("test-data/positions.json").read())


def uci_to_coord(uci):
    return (ord(uci[0]) - 97, int(uci[1]) - 1)


def promotion_str(s):
    match s:
        case "q":
            return "Queen"
        case "r":
            return "Rook"
        case "b":
            return "Bishop"
        case "n":
            return "Knight"


board = chess.Board(FEN)
moves = list(board.legal_moves)

expected = []
for move in moves:
    uci = move.uci()
    move = uci[:4]
    promotion = promotion_str(uci[4]) if len(uci) == 5 else None
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
