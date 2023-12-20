class ChessBoard:
    def __init__(self):
        self.board = [[' ' for _ in range(8)] for _ in range(8)]

    def print_board(self):
        for row in self.board:
            print(' '.join(row))
        print()

    def set_piece(self, position, piece):
        col, row = self._position_to_indices(position)
        self.board[row][col] = piece

    def get_legal_moves(self, position):
        piece = self.get_piece(position)
        if piece == ' ':
            return []

        col, row = self._position_to_indices(position)

        if piece.lower() == 'p':
            return self._get_pawn_moves(col, row, piece.isupper())
        elif piece.lower() == 'r':
            return self._get_rook_moves(col, row)
        elif piece.lower() == 'n':
            return self._get_knight_moves(col, row)
        elif piece.lower() == 'b':
            return self._get_bishop_moves(col, row)
        elif piece.lower() == 'q':
            return self._get_queen_moves(col, row)
        elif piece.lower() == 'k':
            return self._get_king_moves(col, row)

    def get_piece(self, position):
        col, row = self._position_to_indices(position)
        return self.board[row][col]

    def _position_to_indices(self, position):
        col = ord(position[0]) - ord('a')
        row = 8 - int(position[1])
        return col, row

    def _is_valid_position(self, col, row):
        return 0 <= col < 8 and 0 <= row < 8

    def _get_pawn_moves(self, col, row, is_white):
        moves = []
        direction = 1 if is_white else -1

        # Move one square forward
        if self._is_valid_position(col, row + direction) and self.board[row + direction][col] == ' ':
            moves.append((col, row + direction))

        # Move two squares forward (only on the starting rank)
        if (
            row == 1 and is_white or
            row == 6 and not is_white
        ) and self._is_valid_position(col, row + 2 * direction) and self.board[row + 2 * direction][col] == ' ':
            moves.append((col, row + 2 * direction))

        # Capture diagonally
        for d_col in [-1, 1]:
            new_col = col + d_col
            if self._is_valid_position(new_col, row + direction) and self.board[row + direction][new_col].islower() != is_white:
                moves.append((new_col, row + direction))

        return moves

    def _get_rook_moves(self, col, row):
        moves = []

        # Horizontal moves
        for new_col in range(8):
            if new_col != col:
                moves.append((new_col, row))

        # Vertical moves
        for new_row in range(8):
            if new_row != row:
                moves.append((col, new_row))

        return moves

    def _get_knight_moves(self, col, row):
        moves = []

        # Knight moves
        knight_moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2)
        ]

        for d_col, d_row in knight_moves:
            new_col, new_row = col + d_col, row + d_row
            if self._is_valid_position(new_col, new_row):
                moves.append((new_col, new_row))

        return moves

    def _get_bishop_moves(self, col, row):
        moves = []

        # Diagonal moves
        for d_col, d_row in [(-1, -1), (-1, 1), (1, -1), (1, 1)]:
            new_col, new_row = col + d_col, row + d_row
            while self._is_valid_position(new_col, new_row):
                moves.append((new_col, new_row))
                if self.board[new_row][new_col] != ' ':
                    break
                new_col += d_col
                new_row += d_row

        return moves

    def _get_queen_moves(self, col, row):
        return self._get_rook_moves(col, row) + self._get_bishop_moves(col, row)

    def _get_king_moves(self, col, row):
        moves = []

        # King moves
        king_moves = [
            (1, 0), (-1, 0), (0, 1), (0, -1),
            (1, 1), (1, -1), (-1, 1), (-1, -1)
        ]

        for d_col, d_row in king_moves:
            new_col, new_row = col + d_col, row + d_row
            if self._is_valid_position(new_col, new_row):
                moves.append((new_col, new_row))

        return moves

# Example usage
if __name__ == "__main__":
    chess_board = ChessBoard()

    # Set up pieces
    chess_board.set_piece('e1', 'K')
    chess_board.set_piece('e8', 'k')
    chess_board.set_piece('d2', 'P')
    chess_board.set_piece('d7', 'p')
    chess_board.set_piece('a1', 'R')
    chess_board.set_piece('h8', 'r')

    # Print the initial board
    print("Initial Chess Board:")
    chess_board.print_board()

    # Get legal moves for a piece
    piece_position = 'd2'
    legal_moves = chess_board.get_legal_moves(piece_position)

    print(f"Legal moves for the piece at {piece_position}: {legal_moves}")

