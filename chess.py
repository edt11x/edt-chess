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
        print(self.color,  self.name, 'at :', col, row)

    def printPosition(self):
        print(self.color,  self.name, 'at :', self.col, self.row)


def initPositions(array, color, name, columns, row):
    for c in columns:
        array.append(Piece(color, name, c, row))

# rather than worry about a separate concept of squares, we are just going to
# keep track of the positions that the pieces occupy.
def isSquareOccupied(array, col, row):
    ret = False
    for x in range(len(array)):
        if ((array[x].col == col) and (array[x].row == row)):
            ret = True
    return ret

def randomMovePiece(array, color):
    for x in range(len(Pieces)):
        if (Pieces[x].color == color):
            print('Try to move', Pieces[x].color,  Pieces[x].name, 'at :', Pieces[x].col, Pieces[x].row)

def possiblePawnMoves(array, color, piece):
    if (piece.name == 'Pawn') and (piece.row != 8) and (piece.row != 1) and (piece.color == color):
        # walk through the array and see if we can move one space forward?
        if (piece.color == 'White') and (isSquareOccupied(array, piece.col, piece.row+1) == False):
            print(' possible move to', end=' ')
            piece.printNewPosition(piece.col, piece.row+1)
        if (piece.color == 'Black') and (isSquareOccupied(array, piece.col, piece.row-1) == False):
            print(' possible move to', end=' ')
            piece.printNewPosition(piece.col, piece.row-1)
        # walk through the array and see if we can move two spaces forward?
        if (piece.color == 'White') and (isSquareOccupied(array, piece.col, piece.row+1) == False):
            if (piece.row == 2) and (piece.firstMove) and (isSquareOccupied(array, piece.col, piece.row+2) == False):
                print(' possible move to', end=' ')
                piece.printNewPosition(piece.col, piece.row+2)
        if (piece.color == 'Black') and (isSquareOccupied(array, piece.col, piece.row-1) == False):
            if (piece.row == 7) and (piece.firstMove) and (isSquareOccupied(array, piece.col, piece.row-2) == False):
                    print(' possible move to', end=' ')
                    piece.printNewPosition(piece.col, piece.row-2)
    # walk through the array and see if we can take left?
    # walk through the array and see if we can take right?
    # walk through the array and check to see if we can take en passant?
    # check to see if we are promoted.
    print

def possibleMoves(array, color, piece):
    if (piece.color == color):
        if (piece.name == 'Pawn'):
            if (piece.firstMove == True):
                print('Pawn can move one or two spaces')


def drawPawn(canvas, color, col, row):
    # Draw a pawn shape on the canvas 

    # Pawn dimensions
    pawn_width = square_size * 0.8
    pawn_height = square_size * 0.8
    bottom_width = pawn_width * 0.5

    # Draw the pawn shape
    x1 = col*square_size + (square_size - pawn_width)/2
    y1 = row*square_size + (square_size - pawn_height)/2
    x2 = x1 + pawn_width
    y2 = y1 + bottom_width
    x3 = x1 + pawn_width/2
    y3 = y1 + pawn_height

    pawn_shape = [x1, y1, x2, y2, x3, y3]
    chess_canvas.create_polygon(pawn_shape, fill="red", outline="black")


def drawPawn2(canvas, color, col, row):

    # Draw the base of the pawn
    canvas.create_rectangle(10, 0, 40, 50, fill='white')

    # Draw the top part of the pawn
    canvas.create_line(25, 15, 25, 35, width=5, fill='black')

    # Draw the bottom part of the pawn
    canvas.create_line(25, 35, 25, 45, width=2, fill='black')


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

drawPawn(chess_canvas, 'White', 0, 1)
drawPawn2(chess_canvas, 'White', 0, 2)


root.mainloop()
