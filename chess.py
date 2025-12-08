#
import tkinter as tk


class Piece:
    def __init__(self, color, name, col, row):
        self.color = color
        self.name = name
        self.col = col
        self.row = row
        self.firstMove = True

    def printNewPosition(self, col, row):
        print(self.color, self.name, 'at :', col, row)

    def printPosition(self):
        print(self.color, self.name, 'at :', self.col, self.row)


def initPositions(array, color, name, columns, row):
    for c in columns:
        array.append(Piece(color, name, c, row))


def isSquareOccupied(array, col, row):
    ret = False
    for x in range(len(array)):
        if ((array[x].col == col) and (array[x].row == row)):
            ret = True
    return ret


def randomMovePiece(array, color):
    for x in range(len(Pieces)):
        if (Pieces[x].color == color):
            print('Try to move', Pieces[x].color, Pieces[x].name, 'at :', Pieces[x].col, Pieces[x].row)


def possiblePawnMoves(array, color, piece):
    if (piece.name == 'Pawn') and (piece.row != 8) and (piece.row != 1) and (piece.color == color):
        # walk through the array and see if we can move one space forward?
        if (piece.color == 'White') and (isSquareOccupied(array, piece.col, piece.row + 1) == False):
            print(' possible move to', end=' ')
            piece.printNewPosition(piece.col, piece.row + 1)
        if (piece.color == 'Black') and (isSquareOccupied(array, piece.col, piece.row - 1) == False):
            print(' possible move to', end=' ')
            piece.printNewPosition(piece.col, piece.row - 1)
        # walk through the array and see if we can move two spaces forward?
        if (piece.color == 'White') and (isSquareOccupied(array, piece.col, piece.row + 1) == False):
            if (piece.row == 2) and (piece.firstMove) and (isSquareOccupied(array, piece.col, piece.row + 2) == False):
                print(' possible move to', end=' ')
                piece.printNewPosition(piece.col, piece.row + 2)
        if (piece.color == 'Black') and (isSquareOccupied(array, piece.col, piece.row - 1) == False):
            if (piece.row == 7) and (piece.firstMove) and (isSquareOccupied(array, piece.col, piece.row - 2) == False):
                print(' possible move to', end=' ')
                piece.printNewPosition(piece.col, piece.row - 2)
    # walk through the array and see if we can take left?
    # walk through the array and see if we can take right?
    # walk through the array and check to see if we can take en passant?
    # check to see if we are promoted.


def possibleMoves(array, color, piece):
    if (piece.color == color):
        if (piece.name == 'Pawn'):
            if (piece.firstMove == True):
                print('Pawn can move one or two spaces')


def _col_to_index(col):
    # Accept either a character 'a'..'h' or a 0-based integer
    if isinstance(col, int):
        return col
    try:
        return ord(col.lower()) - ord('a')
    except Exception:
        return 0


def _row_to_index(row):
    # Convert chess row (1..8) to 0-based canvas row (0 at top)
    try:
        return 8 - int(row)
    except Exception:
        return int(row)


def piece_unicode(name, color):
    # Map piece name and color to a unicode chess glyph
    glyphs = {
        'Pawn':   {'White': '\u2659', 'Black': '\u265F'},
        'Rook':   {'White': '\u2656', 'Black': '\u265C'},
        'Knight': {'White': '\u2658', 'Black': '\u265E'},
        'Bishop': {'White': '\u2657', 'Black': '\u265D'},
        'Queen':  {'White': '\u2655', 'Black': '\u265B'},
        'King':   {'White': '\u2654', 'Black': '\u265A'},
    }
    return glyphs.get(name, {}).get(color, '?')


def draw_piece(canvas, piece, square_size):
    col_idx = _col_to_index(piece.col)
    row_idx = _row_to_index(piece.row)

    # center of the square
    cx = col_idx * square_size + square_size / 2
    cy = row_idx * square_size + square_size / 2

    glyph = piece_unicode(piece.name, piece.color)
    size = int(square_size * 0.6)
    # use a tag so pieces can be cleared/redrawn later
    canvas.create_text(cx, cy, text=glyph, font=("DejaVu Sans", size), tags=("piece",), fill=("white" if piece.color == 'White' else "black"))


def draw_all_pieces(canvas, pieces, square_size):
    # remove previously drawn pieces
    canvas.delete("piece")
    for p in pieces:
        draw_piece(canvas, p, square_size)


# initialize the board
Pieces = []
initPositions(Pieces, 'White', 'Pawn', 'abcdefgh', 2)
initPositions(Pieces, 'White', 'Rook', 'ah', 1)
initPositions(Pieces, 'White', 'Knight', 'bg', 1)
initPositions(Pieces, 'White', 'Bishop', 'cf', 1)
initPositions(Pieces, 'White', 'Queen', 'd', 1)
initPositions(Pieces, 'White', 'King', 'e', 1)

initPositions(Pieces, 'Black', 'Pawn', 'abcdefgh', 7)
initPositions(Pieces, 'Black', 'Rook', 'ah', 8)
initPositions(Pieces, 'Black', 'Knight', 'bg', 8)
initPositions(Pieces, 'Black', 'Bishop', 'cf', 8)
initPositions(Pieces, 'Black', 'Queen', 'd', 8)
initPositions(Pieces, 'Black', 'King', 'e', 8)

# Print positions of white pawns, to test the initialization
for x in range(len(Pieces)):
    if ((Pieces[x].name == 'Pawn') and (Pieces[x].color == 'White')):
        Pieces[x].printPosition()
        possiblePawnMoves(Pieces, 'White', Pieces[x])
print(' ')
# Print positions of black pawns, to test the initialization
for x in range(len(Pieces)):
    if ((Pieces[x].name == 'Pawn') and (Pieces[x].color == 'Black')):
        Pieces[x].printPosition()
        possiblePawnMoves(Pieces, 'Black', Pieces[x])
print(' ')

root = tk.Tk()

square_size = 50

width = height = square_size * 8

chess_canvas = tk.Canvas(root, width=width, height=height)
chess_canvas.pack()

for row in range(8):
    for col in range(8):
        x1 = col * square_size
        y1 = row * square_size
        x2 = x1 + square_size
        y2 = y1 + square_size

        if (row + col) % 2 == 0:
            chess_canvas.create_rectangle(x1, y1, x2, y2, fill='white')
        else:
            chess_canvas.create_rectangle(x1, y1, x2, y2, fill='black')

# draw all pieces from the Pieces list
draw_all_pieces(chess_canvas, Pieces, square_size)


root.mainloop()
