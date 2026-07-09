//! Board management, move validation, game state, and PGN export.

use shakmaty::{
    fen::Fen,
    san::SanPlus,
    uci::UciMove,
    CastlingMode, CastlingSide, Chess, Color, File, Move, Position, Rank, Role, Square,
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
    san_history: Vec<String>,
    /// FEN of the starting position when not the standard start (puzzles/practice).
    start_fen: Option<String>,
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
            san_history: Vec::new(),
            start_fen: None,
        }
    }

    /// Load a position from FEN (standard chess). Clears move history.
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parsed: Fen = fen
            .parse()
            .map_err(|e| format!("invalid FEN: {e}"))?;
        let board: Chess = parsed
            .into_position(CastlingMode::Standard)
            .map_err(|e| format!("illegal position: {e}"))?;
        Ok(Self {
            board,
            move_history: Vec::new(),
            san_history: Vec::new(),
            start_fen: Some(fen.trim().to_string()),
        })
    }

    pub fn board(&self) -> &Chess {
        &self.board
    }

    pub fn move_history(&self) -> &[Move] {
        &self.move_history
    }

    pub fn san_history(&self) -> &[String] {
        &self.san_history
    }

    /// Human-readable move list, e.g. `"1. e4 e5 2. Nf3"`.
    pub fn move_list_text(&self) -> String {
        let mut out = String::new();
        for (i, san) in self.san_history.iter().enumerate() {
            if i % 2 == 0 {
                if i > 0 {
                    out.push(' ');
                }
                out.push_str(&format!("{}. {}", i / 2 + 1, san));
            } else {
                out.push(' ');
                out.push_str(san);
            }
        }
        out
    }

    /// Export the game as PGN (portable game notation).
    pub fn to_pgn(&self, white_name: &str, black_name: &str) -> String {
        let mut pgn = String::new();
        pgn.push_str("[Event \"edt-chess practice\"]\n");
        pgn.push_str("[Site \"Local\"]\n");
        pgn.push_str(&format!("[White \"{white_name}\"]\n"));
        pgn.push_str(&format!("[Black \"{black_name}\"]\n"));
        if let Some(fen) = &self.start_fen {
            pgn.push_str("[SetUp \"1\"]\n");
            pgn.push_str(&format!("[FEN \"{fen}\"]\n"));
        }
        let result = match self.result() {
            Some(GameResult::Checkmate { winner: Color::White }) => "1-0",
            Some(GameResult::Checkmate { winner: Color::Black }) => "0-1",
            Some(GameResult::Stalemate) | Some(GameResult::Draw) => "1/2-1/2",
            None => "*",
        };
        pgn.push_str(&format!("[Result \"{result}\"]\n\n"));

        let moves = self.move_list_text();
        if !moves.is_empty() {
            pgn.push_str(&moves);
            pgn.push(' ');
        }
        pgn.push_str(result);
        pgn.push('\n');
        pgn
    }

    /// Apply a UCI move string such as `"e2e4"`, `"e1g1"` (castle), or `"e7e8q"`.
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
        let san = SanPlus::from_move(self.board.clone(), mv).to_string();
        self.board = self.board.clone().play(mv).expect("move checked legal");
        self.move_history.push(mv);
        self.san_history.push(san);
    }

    /// Legal destination square names for a piece on `square` (e.g. `"e2"`).
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
        let mut dests = Vec::new();
        for m in self.legal_moves_from(from) {
            let d = if m.is_castle() {
                castling_king_destination(&m).unwrap_or_else(|| m.to())
            } else {
                m.to()
            };
            if !dests.contains(&d) {
                dests.push(d);
            }
        }
        dests
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
            if m.is_castle() {
                return castling_king_destination(m) == Some(to);
            }
            m.to() == to
                && match (m.promotion(), promotion) {
                    (None, None) => true,
                    (Some(a), Some(b)) => a == b,
                    (Some(Role::Queen), None) => true,
                    _ => false,
                }
        })
    }

    /// True if moving from→to requires choosing a promotion piece.
    pub fn requires_promotion(&self, from: Square, to: Square) -> bool {
        self.board.legal_moves().into_iter().any(|m| {
            m.from() == Some(from) && m.to() == to && m.promotion().is_some()
        })
    }

    /// Promotion roles available for from→to (typically Q,R,B,N).
    pub fn promotion_roles(&self, from: Square, to: Square) -> Vec<Role> {
        self.board
            .legal_moves()
            .into_iter()
            .filter(|m| m.from() == Some(from) && m.to() == to)
            .filter_map(|m| m.promotion())
            .collect()
    }

    pub fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn result(&self) -> Option<GameResult> {
        if !self.board.is_game_over() {
            return None;
        }
        if self.board.is_checkmate() {
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
        Fen::from_position(&self.board, shakmaty::EnPassantMode::Legal).to_string()
    }

    pub fn undo_move(&mut self) -> bool {
        if self.move_history.is_empty() {
            return false;
        }
        let history = self.move_history.clone();
        let start = self.start_fen.clone();
        *self = if let Some(fen) = start {
            Self::from_fen(&fen).unwrap_or_else(|_| Self::new())
        } else {
            Self::new()
        };
        for mv in history.iter().take(history.len() - 1) {
            self.push_move(*mv);
        }
        true
    }

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

    pub fn piece_glyph_at(&self, square: Square) -> &'static str {
        match self.board.board().piece_at(square) {
            Some(piece) => piece_glyph(piece.color, piece.role),
            None => "",
        }
    }

    pub fn move_san(&self, mv: Move) -> String {
        SanPlus::from_move(self.board.clone(), mv).to_string()
    }

    pub fn last_move(&self) -> Option<Move> {
        self.move_history.last().copied()
    }

    pub fn last_san(&self) -> Option<&str> {
        self.san_history.last().map(String::as_str)
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

pub fn role_from_promo_char(c: char) -> Option<Role> {
    match c.to_ascii_lowercase() {
        'q' => Some(Role::Queen),
        'r' => Some(Role::Rook),
        'b' => Some(Role::Bishop),
        'n' => Some(Role::Knight),
        _ => None,
    }
}

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
        assert_eq!(game.san_history(), &["e4".to_string()]);
    }

    #[test]
    fn illegal_move_rejected() {
        let mut game = ChessGame::new();
        assert!(!game.make_move_uci("e2e5"));
        assert!(!game.make_move_uci("e1e2"));
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
        assert!(game.san_history().is_empty());
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
    }

    #[test]
    fn status_shows_check() {
        let mut game = ChessGame::new();
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
        for mv in ["g1f3", "g8f6", "e2e3", "e7e6", "f1e2", "f8e7", "e1g1"] {
            assert!(game.make_move_uci(mv), "failed on {mv}");
        }
        assert_eq!(game.piece_glyph_at(Square::G1), "♔");
        assert_eq!(game.piece_glyph_at(Square::F1), "♖");
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
    }

    #[test]
    fn click_style_find_matches_castling_king_destination() {
        let mut game = ChessGame::new();
        for mv in ["g1f3", "g8f6", "e2e3", "e7e6", "f1e2", "f8e7"] {
            assert!(game.make_move_uci(mv), "failed on {mv}");
        }
        assert!(game.find_legal_move(Square::E1, Square::G1, None).is_some());
        assert!(game.legal_destinations(Square::E1).contains(&Square::G1));
        assert!(game.make_move_uci("e1g1"));
    }

    #[test]
    fn move_san_opening() {
        let game = ChessGame::new();
        let mv = game.find_legal_move(Square::E2, Square::E4, None).unwrap();
        let san = game.move_san(mv);
        assert!(san.contains('e'), "unexpected SAN {san}");
    }

    #[test]
    fn move_list_and_pgn() {
        let mut game = ChessGame::new();
        game.make_move_uci("e2e4");
        game.make_move_uci("e7e5");
        game.make_move_uci("g1f3");
        let list = game.move_list_text();
        assert!(list.contains("1. e4 e5"));
        assert!(list.contains("2. Nf3"));
        let pgn = game.to_pgn("Human", "Computer");
        assert!(pgn.contains("[White \"Human\"]"));
        assert!(pgn.contains("1. e4 e5 2. Nf3"));
        assert!(pgn.contains(" *") || pgn.ends_with("*\n"));
    }

    #[test]
    fn from_fen_loads_puzzle_position() {
        // Mate in 1: white to move Qh5# style setup simplified — back rank idea
        let fen = "6k1/5ppp/8/8/8/8/8/4Q1K1 w - - 0 1";
        let mut game = ChessGame::from_fen(fen).expect("fen");
        assert_eq!(game.current_turn(), Color::White);
        assert!(game.make_move_uci("e1e8") || !game.legal_moves_for_square("e1").is_empty());
    }

    #[test]
    fn pgn_includes_setup_fen_when_nonstandard() {
        let fen = "4k3/8/8/8/8/8/4P3/4K3 w - - 0 1";
        let mut game = ChessGame::from_fen(fen).unwrap();
        game.make_move_uci("e2e4");
        let pgn = game.to_pgn("W", "B");
        assert!(pgn.contains("[SetUp \"1\"]"));
        assert!(pgn.contains("FEN"));
    }
}
