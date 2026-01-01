import unittest
import chess
import sunfish


class TestChessIntegration(unittest.TestCase):
    def test_board_initialization(self):
        board = chess.Board()
        self.assertEqual(board.fen(), chess.STARTING_FEN)

    def test_legal_moves_generation(self):
        board = chess.Board()
        moves = list(board.legal_moves)
        self.assertGreater(len(moves), 0)
        # e2e4 should be legal
        e2e4 = chess.Move.from_uci('e2e4')
        self.assertIn(e2e4, moves)

    def test_user_move_application(self):
        board = chess.Board()
        move = chess.Move.from_uci('e2e4')
        board.push(move)
        self.assertEqual(board.piece_at(chess.E4), chess.Piece(chess.PAWN, chess.WHITE))
        self.assertIsNone(board.piece_at(chess.E2))

    def test_computer_move_ai(self):
        board = chess.Board()
        # Make a user move
        board.push(chess.Move.from_uci('e2e4'))
        # Computer should find a move
        move = sunfish.get_computer_move(board)
        self.assertIsNotNone(move)
        self.assertIn(move, board.legal_moves)

    def test_game_over_detection(self):
        board = chess.Board()
        # Fool's mate
        board.push(chess.Move.from_uci('f2f3'))
        board.push(chess.Move.from_uci('e7e5'))
        board.push(chess.Move.from_uci('g2g4'))
        board.push(chess.Move.from_uci('d8h4'))
        self.assertTrue(board.is_checkmate())

    def test_turn_switching(self):
        board = chess.Board()
        self.assertEqual(board.turn, chess.WHITE)
        board.push(chess.Move.from_uci('e2e4'))
        self.assertEqual(board.turn, chess.BLACK)


if __name__ == '__main__':
    unittest.main()