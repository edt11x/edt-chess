import chess

class ChessBoard:
    """Wrapper around python-chess that exposes a simple board API.

    get_legal_moves() now uses python-chess's legal move generator, which
    guarantees that moves never leave your own king in check.
    """

    def __init__(self):
        # Start from an empty board; we place pieces explicitly.
        self.board = chess.Board.empty()

    def print_board(self):
        print(self.board)
        print()

    def _position_to_square(self, position: str) -> chess.Square:
        file_idx = ord(position[0]) - ord("a")  # a -> 0, b -> 1, ...
        rank_idx = int(position[1]) - 1        # "1" -> 0, "2" -> 1, ...
        return chess.square(file_idx, rank_idx)

    def _square_to_indices(self, square: chess.Square):
        file_idx = chess.square_file(square)
        rank_idx = chess.square_rank(square)
        # Match the old convention: row 0 is rank 8, row 7 is rank 1
        col = file_idx
        row = 7 - rank_idx
        return col, row

    def set_piece(self, position, piece):
        """Place or remove a piece using single-letter notation, e.g. 'K', 'q', 'p'."""
        square = self._position_to_square(position)
        if piece == " ":
            self.board.remove_piece_at(square)
            return

        piece_map = {
            "p": chess.PAWN,
            "n": chess.KNIGHT,
            "b": chess.BISHOP,
            "r": chess.ROOK,
            "q": chess.QUEEN,
            "k": chess.KING,
        }
        piece_type = piece_map[piece.lower()]
        color = chess.WHITE if piece.isupper() else chess.BLACK
        self.board.set_piece_at(square, chess.Piece(piece_type, color))

    def get_legal_moves(self, position):
        """Return legal destination squares (col, row) for the piece at position.

        Uses python-chess's legal move generator, so any returned move is
        guaranteed not to leave the moving side's king in check.
        """
        square = self._position_to_square(position)
        if self.board.piece_at(square) is None:
            return []

        moves = []
        for move in self.board.legal_moves:
            if move.from_square == square:
                col, row = self._square_to_indices(move.to_square)
                moves.append((col, row))
        return moves


if __name__ == "__main__":
    chess_board = ChessBoard()

    # White major pieces (rank 1)
    for file, piece in zip("abcdefgh", "RNBQKBNR"):
        chess_board.set_piece(f"{file}1", piece)

    # White pawns (rank 2)
    for file in "abcdefgh":
        chess_board.set_piece(f"{file}2", "P")

    # Black major pieces (rank 8)
    for file, piece in zip("abcdefgh", "rnbqkbnr"):
        chess_board.set_piece(f"{file}8", piece)

    # Black pawns (rank 7)
    for file in "abcdefgh":
        chess_board.set_piece(f"{file}7", "p")

    print("Initial Chess Board (python-chess backed):")
    chess_board.print_board()

    for piece_position in ("e2", "b1", "g8"):
        legal_moves = chess_board.get_legal_moves(piece_position)
        print(f"Legal moves for the piece at {piece_position}: {legal_moves}")

