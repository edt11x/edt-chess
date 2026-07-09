//! edt-chess library: board state and AI for the graphical practice app.

pub mod ai;
pub mod game;

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
