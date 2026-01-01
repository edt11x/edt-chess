import unittest
import chess
from game import ChessGame
from ai import ChessAI


class TestChessIntegration(unittest.TestCase):
    def test_game_initialization(self):
        game = ChessGame()
        self.assertEqual(game.board.fen(), chess.STARTING_FEN)

    def test_legal_moves_generation(self):
        game = ChessGame()
        moves = game.get_legal_moves_for_square('e2')
        self.assertIn('e3', moves)
        self.assertIn('e4', moves)

    def test_user_move_application(self):
        game = ChessGame()
        success = game.make_move('e2e4')
        self.assertTrue(success)
        self.assertEqual(game.board.piece_at(chess.E4), chess.Piece(chess.PAWN, chess.WHITE))
        self.assertIsNone(game.board.piece_at(chess.E2))

    def test_computer_move_ai(self):
        game = ChessGame()
        ai = ChessAI()
        # Make a user move
        game.make_move('e2e4')
        # Computer should find a move
        move = ai.get_best_move(game.board)
        self.assertIsNotNone(move)
        self.assertIn(move, game.board.legal_moves)

    def test_game_over_detection(self):
        game = ChessGame()
        # Fool's mate
        game.make_move('f2f3')
        game.make_move('e7e5')
        game.make_move('g2g4')
        game.make_move('d8h4')
        self.assertTrue(game.is_game_over())
        self.assertEqual(game.get_winner(), 'Black')

    def test_turn_switching(self):
        game = ChessGame()
        self.assertEqual(game.get_current_turn(), 'White')
        game.make_move('e2e4')
        self.assertEqual(game.get_current_turn(), 'Black')

    def test_hint_functionality(self):
        game = ChessGame()
        ai = ChessAI()
        hint = ai.get_hint(game.board)
        self.assertIsNotNone(hint)
        self.assertIn(hint['move'], game.board.legal_moves)

    def test_undo_functionality(self):
        game = ChessGame()
        original_fen = game.board.fen()
        game.make_move('e2e4')
        self.assertNotEqual(game.board.fen(), original_fen)
        game.undo_move()
        self.assertEqual(game.board.fen(), original_fen)


if __name__ == '__main__':
    unittest.main()