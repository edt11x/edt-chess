//! edt-chess — graphical chess practice app (Rust + Slint).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

use edt_chess::ai::ChessAI;
use edt_chess::game::{square_from_index, square_name, ChessGame};
use edt_chess::{color_name, APP_DESCRIPTION, APP_NAME, VERSION};
use shakmaty::{Color, Position, Square};
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

struct AppState {
    game: ChessGame,
    ai: ChessAI,
    /// Color the human plays.
    player_color: Color,
    selected: Option<Square>,
    legal_targets: Vec<Square>,
    difficulty: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            game: ChessGame::new(),
            ai: ChessAI::new(3),
            player_color: Color::White,
            selected: None,
            legal_targets: Vec::new(),
            difficulty: "medium".into(),
        }
    }

    fn clear_selection(&mut self) {
        self.selected = None;
        self.legal_targets.clear();
    }

    fn reset_game(&mut self) {
        self.game = ChessGame::new();
        self.clear_selection();
    }
}

fn print_help() {
    println!(
        "{APP_NAME} {VERSION}
{APP_DESCRIPTION}

USAGE:
    edt-chess [OPTIONS]

OPTIONS:
    -h, --help       Print this help message and exit
    -V, --version    Print version information and exit

CONTROLS (GUI):
    Choose White or Black to start a game.
    Click a piece, then a destination square to move.
    Promotion defaults to queen.
    Side panel: Easy/Medium/Hard difficulty, Hint, Undo, New Game.

INSTALL:
    See README.md or run: ./scripts/build-and-install.sh

ENVIRONMENT:
    Uses the system display (X11 or Wayland) via Slint/winit.
"
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" | "help" => {
                print_help();
                return Ok(());
            }
            "-V" | "--version" | "version" => {
                println!("{APP_NAME} {VERSION}");
                return Ok(());
            }
            other => {
                eprintln!("Unknown option: {other}");
                eprintln!("Try 'edt-chess --help' for more information.");
                std::process::exit(2);
            }
        }
    }

    run_gui()
}

fn run_gui() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let state = Arc::new(Mutex::new(AppState::new()));

    {
        let st = state.lock().expect("state lock");
        refresh_board(&ui, &st);
    }
    ui.set_show_color_picker(true);
    ui.set_difficulty(SharedString::from("medium"));

    // --- Color selection ---
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_choose_color(move |color| {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("state lock");
            st.player_color = if color.as_str() == "black" {
                Color::Black
            } else {
                Color::White
            };
            st.reset_game();
            ui.set_show_color_picker(false);
            ui.set_board_flipped(st.player_color == Color::Black);
            ui.set_player_color_text(SharedString::from(format!(
                "You: {}  ·  Computer: {}",
                color_name(st.player_color),
                color_name(!st.player_color)
            )));
            ui.set_hint_text(SharedString::from(""));
            refresh_board(&ui, &st);
            drop(st);
            // If human is Black, computer (White) moves first.
            maybe_start_ai_move(&ui, &state);
        });
    }

    // --- Square clicks ---
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_square_clicked(move |index| {
            let ui = ui_weak.unwrap();
            if ui.get_ai_thinking() || ui.get_show_color_picker() {
                return;
            }
            let mut st = state.lock().expect("state lock");
            if st.game.is_game_over() {
                return;
            }
            // Only allow human moves on their turn.
            if st.game.current_turn() != st.player_color {
                return;
            }
            let Some(sq) = square_from_index(index) else {
                return;
            };

            // Second click: try to move to target.
            if let Some(from) = st.selected {
                if from == sq {
                    st.clear_selection();
                    refresh_board(&ui, &st);
                    return;
                }
                if st.legal_targets.contains(&sq) {
                    if let Some(mv) = st.game.find_legal_move(from, sq, None) {
                        st.game.make_move(mv);
                        st.clear_selection();
                        ui.set_hint_text(SharedString::from(""));
                        refresh_board(&ui, &st);
                        drop(st);
                        maybe_start_ai_move(&ui, &state);
                        return;
                    }
                }
                // Clicked another own piece — fall through to reselect.
            }

            // First click / reselect: select own piece with legal moves.
            let piece = st.game.board().board().piece_at(sq);
            if let Some(piece) = piece {
                if piece.color == st.player_color && st.game.current_turn() == st.player_color {
                    let dests = st.game.legal_destinations(sq);
                    if !dests.is_empty() {
                        st.selected = Some(sq);
                        st.legal_targets = dests;
                        refresh_board(&ui, &st);
                        return;
                    }
                }
            }

            // Empty / invalid: clear selection.
            st.clear_selection();
            refresh_board(&ui, &st);
        });
    }

    // --- New game ---
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_new_game(move || {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("state lock");
            st.reset_game();
            ui.set_hint_text(SharedString::from(""));
            ui.set_show_color_picker(true);
            refresh_board(&ui, &st);
        });
    }

    // --- Undo (undo player + AI reply when possible) ---
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_undo(move || {
            let ui = ui_weak.unwrap();
            if ui.get_ai_thinking() {
                return;
            }
            let mut st = state.lock().expect("state lock");
            // Undo until it is the player's turn again, or no history left.
            let _ = st.game.undo_move();
            if st.game.current_turn() != st.player_color && !st.game.move_history().is_empty() {
                let _ = st.game.undo_move();
            }
            st.clear_selection();
            ui.set_hint_text(SharedString::from(""));
            refresh_board(&ui, &st);
        });
    }

    // --- Hint ---
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_hint(move || {
            let ui = ui_weak.unwrap();
            if ui.get_ai_thinking() {
                return;
            }
            let mut st = state.lock().expect("state lock");
            if st.game.current_turn() != st.player_color || st.game.is_game_over() {
                ui.set_hint_text(SharedString::from("No hint available."));
                return;
            }
            if let Some((mv, eval)) = st.ai.get_hint(st.game.board()) {
                let san = st.game.move_san(mv);
                ui.set_hint_text(SharedString::from(format!(
                    "Hint: {san} (eval after: {eval})"
                )));
                if let Some(from) = mv.from() {
                    st.selected = Some(from);
                    st.legal_targets = vec![mv.to()];
                }
                refresh_board(&ui, &st);
            } else {
                ui.set_hint_text(SharedString::from("No hint available."));
            }
        });
    }

    // --- Difficulty ---
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_set_difficulty(move |level| {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("state lock");
            st.difficulty = level.to_string();
            st.ai.set_difficulty(level.as_str());
            ui.set_difficulty(level);
        });
    }

    ui.run()?;
    Ok(())
}

fn refresh_board(ui: &AppWindow, st: &AppState) {
    let last = st.game.last_move();
    let last_from = last.and_then(|m| m.from());
    let last_to = last.map(|m| m.to());

    let mut squares = Vec::with_capacity(64);
    for idx in 0..64i32 {
        let sq = square_from_index(idx).unwrap();
        let file = u32::from(sq) % 8;
        let rank = u32::from(sq) / 8;
        let is_light = (file + rank) % 2 == 1;

        let is_selected = st.selected == Some(sq);
        let is_target = st.legal_targets.contains(&sq);
        let is_last_move = last_from == Some(sq) || last_to == Some(sq);

        squares.push(SquareData {
            piece: SharedString::from(st.game.piece_glyph_at(sq)),
            is_light,
            is_selected,
            is_target,
            is_last_move,
            index: idx,
        });
    }

    let model: ModelRc<SquareData> = ModelRc::new(VecModel::from(squares));
    ui.set_squares(model);
    ui.set_status_text(SharedString::from(st.game.status_text()));
    ui.set_eval_text(SharedString::from(format!(
        "Eval: {:+} (side to move)",
        st.ai.evaluate_board(st.game.board())
    )));

    if let Some(mv) = last {
        let from = mv.from().map(square_name).unwrap_or_else(|| "?".into());
        let to = square_name(mv.to());
        ui.set_last_move_text(SharedString::from(format!("Last: {from}{to}")));
    } else {
        ui.set_last_move_text(SharedString::from(""));
    }
}

/// If it is the computer's turn, search for a move on a background thread.
fn maybe_start_ai_move(ui: &AppWindow, state: &Arc<Mutex<AppState>>) {
    let (board, depth) = {
        let st = state.lock().expect("state lock");
        if st.game.is_game_over() {
            return;
        }
        if st.game.current_turn() == st.player_color {
            return;
        }
        (st.game.board().clone(), st.ai.depth)
    };

    ui.set_ai_thinking(true);
    let ui_weak = ui.as_weak();
    let state = state.clone();

    thread::spawn(move || {
        let ai = ChessAI::new(depth);
        let best = ai.get_best_move(&board);

        let _ = slint::invoke_from_event_loop(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut st = state.lock().expect("state lock");
            if let Some(mv) = best {
                st.game.make_move(mv);
            }
            st.clear_selection();
            ui.set_ai_thinking(false);
            ui.set_hint_text(SharedString::from(""));
            refresh_board(&ui, &st);
        });
    });
}
