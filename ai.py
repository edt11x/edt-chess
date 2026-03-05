import chess

# Centipawn values indexed by piece type constant
PIECE_VALUES = {
    chess.PAWN: 100,
    chess.KNIGHT: 320,
    chess.BISHOP: 330,
    chess.ROOK: 500,
    chess.QUEEN: 900,
    chess.KING: 20000,
}


class ChessAI:
    def __init__(self, depth=3):
        self.depth = depth

    def evaluate_board(self, board):
        """Evaluate the board from the perspective of the side to move."""
        if board.is_checkmate():
            return -9999
        if board.is_stalemate() or board.is_insufficient_material():
            return 0

        score = 0
        for square in chess.SQUARES:
            piece = board.piece_at(square)
            if piece:
                value = PIECE_VALUES[piece.piece_type]
                score += value if piece.color == board.turn else -value
        return score

    def order_moves(self, board, captures_only=False):
        """Return moves sorted for better alpha-beta pruning.

        Captures are scored using MVV-LVA (Most Valuable Victim,
        Least Valuable Attacker). Promotions get a bonus equal to
        the promoted piece value. Non-captures score 0 and appear last.
        """
        scored = []
        for move in board.legal_moves:
            is_capture = board.is_capture(move)
            if captures_only and not is_capture:
                continue

            score = 0
            if is_capture:
                if board.is_en_passant(move):
                    # Always pawn x pawn
                    score = 10 * PIECE_VALUES[chess.PAWN] - PIECE_VALUES[chess.PAWN]
                else:
                    victim = board.piece_at(move.to_square)
                    attacker = board.piece_at(move.from_square)
                    if victim and attacker:
                        score = (10 * PIECE_VALUES[victim.piece_type]
                                 - PIECE_VALUES[attacker.piece_type])

            if move.promotion:
                score += PIECE_VALUES.get(move.promotion, 0)

            scored.append((score, move))

        scored.sort(key=lambda x: x[0], reverse=True)
        return [m for _, m in scored]

    def quiescence(self, board, alpha, beta):
        """Search captures until the position is quiet.

        This prevents the horizon effect where the engine stops searching
        just before a losing capture is made.
        """
        stand_pat = self.evaluate_board(board)

        if stand_pat >= beta:
            return beta
        if stand_pat > alpha:
            alpha = stand_pat

        for move in self.order_moves(board, captures_only=True):
            board.push(move)
            score = -self.quiescence(board, -beta, -alpha)
            board.pop()

            if score >= beta:
                return beta
            if score > alpha:
                alpha = score

        return alpha

    def negamax(self, board, depth, alpha, beta):
        """Negamax with alpha-beta pruning and quiescence search at the leaves."""
        if board.is_game_over():
            return -9999 if board.is_checkmate() else 0

        if depth == 0:
            return self.quiescence(board, alpha, beta)

        for move in self.order_moves(board):
            board.push(move)
            score = -self.negamax(board, depth - 1, -beta, -alpha)
            board.pop()

            if score >= beta:
                return beta
            if score > alpha:
                alpha = score

        return alpha

    def get_best_move(self, board):
        """Get the best move for the current position."""
        best_move = None
        best_value = -float('inf')
        alpha = -float('inf')

        for move in self.order_moves(board):
            board.push(move)
            score = -self.negamax(board, self.depth - 1, -float('inf'), -alpha)
            board.pop()

            if score > best_value:
                best_value = score
                best_move = move
            if score > alpha:
                alpha = score

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
