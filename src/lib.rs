//! edt-chess library: board state, AI, and practice modes.

pub mod ai;
pub mod game;
pub mod practice;

/// Application version (from Cargo package metadata).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Human-readable color name.
pub fn color_name(color: shakmaty::Color) -> &'static str {
    match color {
        shakmaty::Color::White => "White",
        shakmaty::Color::Black => "Black",
    }
}

/// Format a move as UCI (including castling as king destination, and promotion).
pub fn move_to_uci(mv: shakmaty::Move) -> String {
    use shakmaty::{CastlingSide, Color, Rank, Square};
    if mv.is_castle() {
        let from = mv.from().unwrap();
        let color = if from.rank() == Rank::First {
            Color::White
        } else {
            Color::Black
        };
        let to = match (color, mv.castling_side().unwrap()) {
            (Color::White, CastlingSide::KingSide) => Square::G1,
            (Color::White, CastlingSide::QueenSide) => Square::C1,
            (Color::Black, CastlingSide::KingSide) => Square::G8,
            (Color::Black, CastlingSide::QueenSide) => Square::C8,
        };
        return format!(
            "{}{}",
            game::square_name(from),
            game::square_name(to)
        );
    }
    let from = mv.from().map(game::square_name).unwrap_or_else(|| "??".into());
    let to = game::square_name(mv.to());
    let mut s = format!("{from}{to}");
    if let Some(role) = mv.promotion() {
        let c = match role {
            shakmaty::Role::Queen => 'q',
            shakmaty::Role::Rook => 'r',
            shakmaty::Role::Bishop => 'b',
            shakmaty::Role::Knight => 'n',
            _ => 'q',
        };
        s.push(c);
    }
    s
}
