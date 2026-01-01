import unittest
from chess import Piece, get_pawn_moves, get_rook_moves, get_knight_moves, get_bishop_moves, get_queen_moves, get_king_moves


class TestMoveGenerators(unittest.TestCase):
    def test_pawn_initial_two_squares(self):
        p = Piece('White', 'Pawn', 'e', 2)
        pieces = [p]
        moves = get_pawn_moves(pieces, p)
        self.assertIn('e3', moves)
        self.assertIn('e4', moves)

    def test_pawn_blocked(self):
        p = Piece('White', 'Pawn', 'e', 2)
        blocker = Piece('White', 'Pawn', 'e', 3)
        pieces = [p, blocker]
        moves = get_pawn_moves(pieces, p)
        self.assertEqual(moves, [])

    def test_pawn_capture(self):
        p = Piece('White', 'Pawn', 'e', 4)
        enemy = Piece('Black', 'Pawn', 'd', 5)
        pieces = [p, enemy]
        moves = get_pawn_moves(pieces, p)
        self.assertIn('d5', moves)

    def test_rook_moves_open(self):
        r = Piece('White', 'Rook', 'd', 4)
        pieces = [r]
        moves = get_rook_moves(pieces, r)
        # rook should be able to move along file and rank
        self.assertIn('a4', moves)
        self.assertIn('h4', moves)
        self.assertIn('d1', moves)
        self.assertIn('d8', moves)

    def test_bishop_moves_open(self):
        b = Piece('White', 'Bishop', 'd', 4)
        pieces = [b]
        moves = get_bishop_moves(pieces, b)
        self.assertIn('a1', moves)
        self.assertIn('g7', moves)

    def test_knight_moves(self):
        n = Piece('White', 'Knight', 'd', 4)
        pieces = [n]
        moves = get_knight_moves(pieces, n)
        expected = {'c2', 'e2', 'b3', 'f3', 'b5', 'f5', 'c6', 'e6'}
        self.assertTrue(expected.issubset(set(moves)))

    def test_queen_moves(self):
        q = Piece('White', 'Queen', 'd', 4)
        pieces = [q]
        moves = get_queen_moves(pieces, q)
        self.assertIn('d1', moves)
        self.assertIn('a4', moves)
        self.assertIn('g7', moves)

    def test_king_moves(self):
        k = Piece('White', 'King', 'e', 4)
        pieces = [k]
        moves = get_king_moves(pieces, k)
        expected = {'d3', 'e3', 'f3', 'd4', 'f4', 'd5', 'e5', 'f5'}
        self.assertTrue(expected.issubset(set(moves)))


if __name__ == '__main__':
    unittest.main()
