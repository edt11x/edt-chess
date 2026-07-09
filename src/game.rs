//! Board management, move validation, and game state.
//!
//! Implemented with the [shakmaty](https://docs.rs/shakmaty) chess library.

use shakmaty::{
    san::SanPlus,
    uci::UciMove,
    CastlingSide, Chess, Color, File, Move, Position, Rank, Role, Square,
};

use crate::color_name;

/// Result of a finished game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Checkmate { winner: Color },
    Stalemate,
    Draw,
}

/// High-level chess game wrapper used by the UI and AI.
#[derive(Debug, Clone)]
pub struct ChessGame {
    board: Chess,
    move_history: Vec<Move>,
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            board: Chess::default(),
            move_history: Vec::new(),
        }
    }

    pub fn board(&self) -> &Chess {
        &self.board
    }

    pub fn move_history(&self) -> &[Move] {
        &self.move_history
    }

    /// Apply a UCI move string such as `"e2e4"`, `"e1g1"` (castle), or `"e7e8q"`.
    ///
    /// Uses shakmaty's UCI parser so castling and promotions resolve correctly
    /// (castling moves use the rook square as [`Move::to`], not g1/c1).
    pub fn make_move_uci(&mut self, uci: &str) -> bool {
        let Ok(uci_move) = UciMove::from_ascii(uci.as_bytes()) else {
            return false;
        };
        let Ok(mv) = uci_move.to_move(&self.board) else {
            return false;
        };
        self.make_move(mv)
    }

    /// Apply a legal [`Move`]. Returns `false` if the move is illegal.
    pub fn make_move(&mut self, mv: Move) -> bool {
        if !self.board.is_legal(mv) {
            return false;
        }
        self.push_move(mv);
        true
    }

    fn push_move(&mut self, mv: Move) {
        self.board = self.board.clone().play(mv).expect("move checked legal");
        self.move_history.push(mv);
    }

    /// Legal destination square names for a piece on `square` (e.g. `"e2"`).
    /// Destinations use UI squares (king landing square for castling).
    pub fn legal_moves_for_square(&self, square: &str) -> Vec<String> {
        let Ok(from) = parse_square(square) else {
            return Vec::new();
        };
        self.legal_destinations(from)
            .into_iter()
            .map(square_name)
            .collect()
    }

    /// All legal moves originating from `from`.
    pub fn legal_moves_from(&self, from: Square) -> Vec<Move> {
        self.board
            .legal_moves()
            .into_iter()
            .filter(|m| m.from() == Some(from))
            .collect()
    }

    /// Destination squares for UI highlighting (king landing square for castling).
    pub fn legal_destinations(&self, from: Square) -> Vec<Square> {
        self.legal_moves_from(from)
            .into_iter()
            .map(|m| {
                if m.is_castle() {
                    castling_king_destination(&m).unwrap_or_else(|| m.to())
                } else {
                    m.to()
                }
            })
            .collect()
    }

    pub fn find_legal_move(
        &self,
        from: Square,
        to: Square,
        promotion: Option<Role>,
    ) -> Option<Move> {
        self.board.legal_moves().into_iter().find(|m| {
            if m.from() != Some(from) {
                return false;
            }
            // Castling: shakmaty stores the rook square as `to()`, but UI/UCI
            // clicks use the king's landing square (g1/c1/g8/c8).
            if m.is_castle() {
                return castling_king_destination(m) == Some(to);
            }
            m.to() == to
                && match (m.promotion(), promotion) {
                    (None, None) => true,
                    (Some(a), Some(b)) => a == b,
                    // Default promotion to queen when not specified.
                    (Some(Role::Queen), None) => true,
                    _ => false,
                }
        })
    }

    pub fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn result(&self) -> Option<GameResult> {
        if !self.board.is_game_over() {
            return None;
        }
        if self.board.is_checkmate() {
            // Side to move is checkmated; the other side wins.
            let winner = !self.board.turn();
            Some(GameResult::Checkmate { winner })
        } else if self.board.is_stalemate() {
            Some(GameResult::Stalemate)
        } else {
            Some(GameResult::Draw)
        }
    }

    pub fn winner_name(&self) -> Option<&'static str> {
        match self.result()? {
            GameResult::Checkmate { winner } => Some(color_name(winner)),
            _ => None,
        }
    }

    pub fn fen(&self) -> String {
        shakmaty::fen::Fen::from_position(&self.board, shakmaty::EnPassantMode::Legal).to_string()
    }

    pub fn undo_move(&mut self) -> bool {
        if self.move_history.is_empty() {
            return false;
        }
        // Rebuild from start — simple and correct for practice games.
        let history = self.move_history.clone();
        *self = ChessGame::new();
        for mv in history.iter().take(history.len() - 1) {
            self.push_move(*mv);
        }
        true
    }

    /// Undo up to `count` half-moves. Returns how many were undone.
    pub fn undo_moves(&mut self, count: usize) -> usize {
        let mut undone = 0;
        while undone < count && self.undo_move() {
            undone += 1;
        }
        undone
    }

    pub fn current_turn(&self) -> Color {
        self.board.turn()
    }

    pub fn current_turn_name(&self) -> &'static str {
        color_name(self.board.turn())
    }

    /// Piece glyph for display (empty string if vacant).
    pub fn piece_glyph_at(&self, square: Square) -> &'static str {
        match self.board.board().piece_at(square) {
            Some(piece) => piece_glyph(piece.color, piece.role),
            None => "",
        }
    }

    /// SAN notation for a move in the current position (before it is played).
    pub fn move_san(&self, mv: Move) -> String {
        SanPlus::from_move(self.board.clone(), mv).to_string()
    }

    pub fn last_move(&self) -> Option<Move> {
        self.move_history.last().copied()
    }

    pub fn status_text(&self) -> String {
        if let Some(result) = self.result() {
            return match result {
                GameResult::Checkmate { winner } => {
                    format!("Checkmate — {} wins", color_name(winner))
                }
                GameResult::Stalemate => "Stalemate — draw".into(),
                GameResult::Draw => "Draw".into(),
            };
        }
        let check = if self.board.is_check() {
            " — check!"
        } else {
            ""
        };
        format!("{} to move{}", self.current_turn_name(), check)
    }
}

pub fn parse_square(s: &str) -> Result<Square, ()> {
    if s.len() != 2 {
        return Err(());
    }
    let mut chars = s.chars();
    let file = File::from_char(chars.next().unwrap()).ok_or(())?;
    let rank = Rank::from_char(chars.next().unwrap()).ok_or(())?;
    Ok(Square::from_coords(file, rank))
}

pub fn square_name(sq: Square) -> String {
    format!("{}{}", sq.file(), sq.rank())
}

/// Convert board index 0..63 (a1=0, h1=7, a8=56, h8=63) to Square.
pub fn square_from_index(index: i32) -> Option<Square> {
    if (0..64).contains(&index) {
        Some(Square::new(index as u32))
    } else {
        None
    }
}

pub fn square_to_index(sq: Square) -> i32 {
    u32::from(sq) as i32
}

/// King destination square for a castling move (g-file or c-file).
fn castling_king_destination(m: &Move) -> Option<Square> {
    let side = m.castling_side()?;
    let from = m.from()?;
    let color = if from.rank() == Rank::First {
        Color::White
    } else {
        Color::Black
    };
    Some(match (color, side) {
        (Color::White, CastlingSide::KingSide) => Square::G1,
        (Color::White, CastlingSide::QueenSide) => Square::C1,
        (Color::Black, CastlingSide::KingSide) => Square::G8,
        (Color::Black, CastlingSide::QueenSide) => Square::C8,
    })
}

pub fn piece_glyph(color: Color, role: Role) -> &'static str {
    match (color, role) {
        (Color::White, Role::King) => "♔",
        (Color::White, Role::Queen) => "♕",
        (Color::White, Role::Rook) => "♖",
        (Color::White, Role::Bishop) => "♗",
        (Color::White, Role::Knight) => "♘",
        (Color::White, Role::Pawn) => "♙",
        (Color::Black, Role::King) => "♚",
        (Color::Black, Role::Queen) => "♛",
        (Color::Black, Role::Rook) => "♜",
        (Color::Black, Role::Bishop) => "♝",
        (Color::Black, Role::Knight) => "♞",
        (Color::Black, Role::Pawn) => "♟",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_initial_position() {
        let game = ChessGame::new();
        assert!(game
            .fen()
            .starts_with("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
        assert_eq!(game.current_turn(), Color::White);
    }

    #[test]
    fn legal_moves_for_e2() {
        let game = ChessGame::new();
        let moves = game.legal_moves_for_square("e2");
        assert!(moves.contains(&"e3".into()));
        assert!(moves.contains(&"e4".into()));
    }

    #[test]
    fn empty_square_has_no_legal_moves() {
        let game = ChessGame::new();
        assert!(game.legal_moves_for_square("e4").is_empty());
    }

    #[test]
    fn illegal_square_name_returns_empty() {
        let game = ChessGame::new();
        assert!(game.legal_moves_for_square("z9").is_empty());
        assert!(game.legal_moves_for_square("").is_empty());
    }

    #[test]
    fn make_move_e2e4() {
        let mut game = ChessGame::new();
        assert!(game.make_move_uci("e2e4"));
        assert_eq!(game.current_turn(), Color::Black);
        assert_eq!(game.piece_glyph_at(Square::E4), "♙");
        assert_eq!(game.piece_glyph_at(Square::E2), "");
    }

    #[test]
    fn illegal_move_rejected() {
        let mut game = ChessGame::new();
        assert!(!game.make_move_uci("e2e5")); // pawn cannot jump two then one illegally
        assert!(!game.make_move_uci("e1e2")); // king blocked
        assert!(!game.make_move_uci("xx"));
        assert!(!game.make_move_uci(""));
        assert_eq!(game.move_history().len(), 0);
    }

    #[test]
    fn fools_mate() {
        let mut game = ChessGame::new();
        assert!(game.make_move_uci("f2f3"));
        assert!(game.make_move_uci("e7e5"));
        assert!(game.make_move_uci("g2g4"));
        assert!(game.make_move_uci("d8h4"));
        assert!(game.is_game_over());
        assert_eq!(game.winner_name(), Some("Black"));
        assert!(game.status_text().contains("Checkmate"));
        assert!(matches!(
            game.result(),
            Some(GameResult::Checkmate {
                winner: Color::Black
            })
        ));
    }

    #[test]
    fn scholars_mate_white_wins() {
        let mut game = ChessGame::new();
        for mv in ["e2e4", "e7e5", "f1c4", "b8c6", "d1h5", "g8f6", "h5f7"] {
            assert!(game.make_move_uci(mv), "failed on {mv}");
        }
        assert!(game.is_game_over());
        assert_eq!(game.winner_name(), Some("White"));
    }

    #[test]
    fn undo_restores_position() {
        let mut game = ChessGame::new();
        let fen = game.fen();
        game.make_move_uci("e2e4");
        assert_ne!(game.fen(), fen);
        assert!(game.undo_move());
        assert_eq!(game.fen(), fen);
    }

    #[test]
    fn undo_empty_history_fails() {
        let mut game = ChessGame::new();
        assert!(!game.undo_move());
        assert_eq!(game.undo_moves(3), 0);
    }

    #[test]
    fn undo_multiple_half_moves() {
        let mut game = ChessGame::new();
        let fen = game.fen();
        game.make_move_uci("e2e4");
        game.make_move_uci("e7e5");
        game.make_move_uci("g1f3");
        assert_eq!(game.undo_moves(2), 2);
        assert_eq!(game.move_history().len(), 1);
        assert!(game.undo_move());
        assert_eq!(game.fen(), fen);
    }

    #[test]
    fn turn_switches_after_move() {
        let mut game = ChessGame::new();
        assert_eq!(game.current_turn_name(), "White");
        game.make_move_uci("e2e4");
        assert_eq!(game.current_turn_name(), "Black");
    }

    #[test]
    fn parse_square_roundtrip() {
        for name in ["a1", "h1", "a8", "h8", "e4", "d5"] {
            let sq = parse_square(name).expect(name);
            assert_eq!(square_name(sq), name);
            let idx = square_to_index(sq);
            assert_eq!(square_from_index(idx), Some(sq));
        }
    }

    #[test]
    fn square_from_index_bounds() {
        assert!(square_from_index(0).is_some());
        assert!(square_from_index(63).is_some());
        assert!(square_from_index(-1).is_none());
        assert!(square_from_index(64).is_none());
    }

    #[test]
    fn piece_glyphs_present_at_start() {
        let game = ChessGame::new();
        assert_eq!(game.piece_glyph_at(Square::E1), "♔");
        assert_eq!(game.piece_glyph_at(Square::E8), "♚");
        assert_eq!(game.piece_glyph_at(Square::D1), "♕");
        assert_eq!(game.piece_glyph_at(Square::D8), "♛");
        assert_eq!(game.piece_glyph_at(Square::A2), "♙");
        assert_eq!(game.piece_glyph_at(Square::A7), "♟");
    }

    #[test]
    fn status_shows_check() {
        let mut game = ChessGame::new();
        // Fool's mate setup until black delivers checkmate; after g2g4 white is not in check yet.
        game.make_move_uci("f2f3");
        game.make_move_uci("e7e5");
        game.make_move_uci("g2g4");
        assert!(game.status_text().contains("Black to move"));
        game.make_move_uci("d8h4");
        assert!(game.status_text().contains("Checkmate"));
    }

    #[test]
    fn castling_kingside_white() {
        let mut game = ChessGame::new();
        // Clear path: Nf3, e3, Be2, then O-O
        for mv in ["g1f3", "g8f6", "e2e3", "e7e6", "f1e2", "f8e7", "e1g1"] {
            assert!(game.make_move_uci(mv), "failed on {mv}");
        }
        assert_eq!(game.piece_glyph_at(Square::G1), "♔");
        assert_eq!(game.piece_glyph_at(Square::F1), "♖");
        assert_eq!(game.piece_glyph_at(Square::H1), "");
        assert_eq!(game.piece_glyph_at(Square::E1), "");
    }

    #[test]
    fn castling_queenside_white() {
        let mut game = ChessGame::new();
        for mv in [
            "d2d3", "d7d6", "c1e3", "c8e6", "d1d2", "d8d7", "b1c3", "b8c6", "e1c1",
        ] {
            assert!(game.make_move_uci(mv), "failed on {mv}");
        }
        assert_eq!(game.piece_glyph_at(Square::C1), "♔");
        assert_eq!(game.piece_glyph_at(Square::D1), "♖");
    }

    #[test]
    fn promotion_uci_not_available_from_start() {
        let mut game = ChessGame::new();
        assert!(!game.make_move_uci("a7a8q"));
        assert!(!game.make_move_uci("e7e8q"));
    }

    #[test]
    fn click_style_find_matches_castling_king_destination() {
        // shakmaty stores rook square as Move::to for castling; UI uses king landing square.
        let mut game = ChessGame::new();
        for mv in ["g1f3", "g8f6", "e2e3", "e7e6", "f1e2", "f8e7"] {
            assert!(game.make_move_uci(mv), "failed on {mv}");
        }
        assert!(game.find_legal_move(Square::E1, Square::G1, None).is_some());
        let dests = game.legal_destinations(Square::E1);
        assert!(dests.contains(&Square::G1));
        assert!(game.make_move_uci("e1g1"));
    }

    #[test]
    fn move_san_opening() {
        let game = ChessGame::new();
        let mv = game.find_legal_move(Square::E2, Square::E4, None).unwrap();
        let san = game.move_san(mv);
        assert!(san.starts_with('e') || san.contains('e'), "unexpected SAN {san}");
    }
}
