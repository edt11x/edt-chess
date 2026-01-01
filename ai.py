import chess


class ChessAI:
    def __init__(self, depth=3):
        self.depth = depth

    def evaluate_board(self, board):
        """Simple evaluation function."""
        if board.is_checkmate():
            return -9999 if board.turn else 9999
        if board.is_stalemate():
            return 0

        # Piece values
        piece_values = {'p': 1, 'n': 3, 'b': 3, 'r': 5, 'q': 9, 'k': 0}
        score = 0
        for square in chess.SQUARES:
            piece = board.piece_at(square)
            if piece:
                value = piece_values[piece.symbol().lower()]
                score += value if piece.color else -value
        return score

    def minimax(self, board, depth, alpha, beta, maximizing):
        if depth == 0 or board.is_game_over():
            return self.evaluate_board(board)

        if maximizing:
            max_eval = -float('inf')
            for move in board.legal_moves:
                board.push(move)
                eval_score = self.minimax(board, depth - 1, alpha, beta, False)
                board.pop()
                max_eval = max(max_eval, eval_score)
                alpha = max(alpha, eval_score)
                if beta <= alpha:
                    break
            return max_eval
        else:
            min_eval = float('inf')
            for move in board.legal_moves:
                board.push(move)
                eval_score = self.minimax(board, depth - 1, alpha, beta, True)
                board.pop()
                min_eval = min(min_eval, eval_score)
                beta = min(beta, eval_score)
                if beta <= alpha:
                    break
            return min_eval

    def get_best_move(self, board):
        """Get the best move for the current position."""
        best_move = None
        best_value = -float('inf')
        for move in board.legal_moves:
            board.push(move)
            move_value = self.minimax(board, self.depth - 1, -float('inf'), float('inf'), False)
            board.pop()
            if move_value > best_value:
                best_value = move_value
                best_move = move
        return best_move

    def get_hint(self, board):
        """Get a hint for the current position."""
        move = self.get_best_move(board)
        if move:
            return {
                'move': move,
                'evaluation': self.evaluate_position_after_move(board, move)
            }
        return None

    def evaluate_position_after_move(self, board, move):
        """Evaluate the position after making a move."""
        board.push(move)
        eval_score = self.evaluate_board(board)
        board.pop()
        return eval_score

    def set_difficulty(self, level):
        """Set AI difficulty level."""
        if level == 'easy':
            self.depth = 2
        elif level == 'medium':
            self.depth = 3
        elif level == 'hard':
            self.depth = 4
        else:
            self.depth = level if isinstance(level, int) else 3