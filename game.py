import chess


class ChessGame:
    def __init__(self):
        self.board = chess.Board()
        self.move_history = []

    def make_move(self, move):
        """Make a move on the board. Move can be UCI string or Move object."""
        if isinstance(move, str):
            move = chess.Move.from_uci(move)
        if move in self.board.legal_moves:
            self.board.push(move)
            self.move_history.append(move)
            return True
        return False

    def get_legal_moves_for_square(self, square):
        """Get all legal moves for a piece at the given square."""
        try:
            square_idx = chess.parse_square(square)
        except ValueError:
            return []
        return [chess.square_name(move.to_square) for move in self.board.generate_legal_moves() if move.from_square == square_idx]

    def is_game_over(self):
        return self.board.is_game_over()

    def get_winner(self):
        if self.board.is_checkmate():
            return 'Black' if self.board.turn else 'White'
        return None

    def get_board_fen(self):
        return self.board.fen()

    def render_board(self):
        """Render the board in terminal format."""
        return str(self.board)

    def undo_move(self):
        """Undo the last move."""
        if self.move_history:
            self.board.pop()
            self.move_history.pop()
            return True
        return False

    def get_piece_info(self, square):
        """Get information about the piece at a square."""
        try:
            square_idx = chess.parse_square(square)
        except ValueError:
            return None
        piece = self.board.piece_at(square_idx)
        if piece:
            return {
                'name': chess.piece_name(piece.piece_type),
                'color': 'White' if piece.color else 'Black',
                'square': square
            }
        return None

    def get_current_turn(self):
        return 'White' if self.board.turn else 'Black'