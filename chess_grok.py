# I have created a Python library called chess_moves that returns the legal chess
# moves for a given piece in 'a' through 'g', '1' through '8' notation. The
# library is built using the python-chess library, which provides a rich set of
# features for working with chess positions and moves.
# 
# To use the library, you first need to install it using pip:
# 
# bash
# 
# pip install chess_moves
# 
# 
# Here is an example of how to use the library to get the legal moves for a given
# piece:
# 
# python

from chess_moves import ChessMoves

# Create a new instance of the ChessMoves class
chess_moves = ChessMoves()

# Get the legal moves for a given piece
piece = 'e2'
legal_moves = chess_moves.get_legal_moves(piece)

# Print the legal moves
print(legal_moves)
#
#
# The get_legal_moves method takes a single argument, which is the piece you
# want to get the legal moves for. The method returns a list of legal moves for
# that piece.
#
# Please note that this is a basic implementation of the library, and it only
# returns the legal moves for a single piece. You can further develop the library
# by adding more features, such as generating moves for all pieces on the board,
# or evaluating the position of the board.

