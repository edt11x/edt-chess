#
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
def checkIfSquareIsOccupied(array, col, row):
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
    # walk through the array and see if we can move one space forward?
    if (piece.color == color) and (piece.name == 'Pawn') and (piece.row != 8) and (piece.row != 1):
        if (piece.color == 'White') and (checkIfSquareIsOccupied(array, piece.col, piece.row+1) == False):
            print(' possible move to', end=' ')
            piece.printNewPosition(piece.col, piece.row+1)
        if (piece.color == 'Black') and (checkIfSquareIsOccupied(array, piece.col, piece.row-1) == False):
            print(' possible move to', end=' ')
            piece.printNewPosition(piece.col, piece.row-1)
    # walk through the array and see if we can move two spaces forward?
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
