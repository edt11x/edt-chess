//! End-to-end style tests for the edt-chess library (game + AI).

use edt_chess::ai::{depth_for_difficulty, ChessAI};
use edt_chess::game::ChessGame;
use edt_chess::{color_name, APP_NAME, VERSION};
use shakmaty::{Color, Position};

#[test]
fn package_metadata_present() {
    assert_eq!(APP_NAME, "edt-chess");
    assert!(!VERSION.is_empty());
    assert!(VERSION.chars().next().unwrap().is_ascii_digit());
}

#[test]
fn color_name_helpers() {
    assert_eq!(color_name(Color::White), "White");
    assert_eq!(color_name(Color::Black), "Black");
}

#[test]
fn game_and_ai_play_a_short_game() {
    let mut game = ChessGame::new();
    let ai = ChessAI::new(2);

    // Human (White) opens, AI replies as Black.
    assert!(game.make_move_uci("e2e4"));
    let reply = ai.get_best_move(game.board()).expect("AI move");
    assert!(game.make_move(reply));
    assert_eq!(game.current_turn(), Color::White);
    assert_eq!(game.move_history().len(), 2);
}

#[test]
fn undo_after_human_and_ai_pair() {
    let mut game = ChessGame::new();
    let ai = ChessAI::new(2);
    let start = game.fen();

    game.make_move_uci("e2e4");
    let reply = ai.get_best_move(game.board()).unwrap();
    game.make_move(reply);

    // Mimic UI undo: take back AI reply then human move.
    game.undo_move();
    game.undo_move();
    assert_eq!(game.fen(), start);
    assert_eq!(game.current_turn(), Color::White);
}

#[test]
fn hint_matches_legal_move_set() {
    let game = ChessGame::new();
    let ai = ChessAI::new(2);
    let (hint, eval) = ai.get_hint(game.board()).expect("hint");
    assert!(game.board().is_legal(hint));
    // Eval is after the move, from opponent's perspective — just ensure finite.
    assert!(eval.abs() < 50_000);
}

#[test]
fn difficulty_depths_are_ordered() {
    assert!(depth_for_difficulty("easy") < depth_for_difficulty("medium"));
    assert!(depth_for_difficulty("medium") < depth_for_difficulty("hard"));
}

#[test]
fn legal_moves_e2_via_library() {
    let game = ChessGame::new();
    let moves = game.legal_moves_for_square("e2");
    assert_eq!(moves.len(), 2);
    assert!(moves.contains(&"e3".to_string()));
    assert!(moves.contains(&"e4".to_string()));
}

#[test]
fn fools_mate_via_library() {
    let mut game = ChessGame::new();
    for mv in ["f2f3", "e7e5", "g2g4", "d8h4"] {
        assert!(game.make_move_uci(mv), "{mv}");
    }
    assert!(game.is_game_over());
    assert_eq!(game.winner_name(), Some("Black"));
}
