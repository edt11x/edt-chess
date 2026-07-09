//! edt-chess — graphical chess practice app (Rust + Slint).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use edt_chess::ai::ChessAI;
use edt_chess::game::{role_from_promo_char, square_from_index, square_name, ChessGame};
use edt_chess::practice::{OpeningTrainer, PuzzleSession};
use edt_chess::{color_name, move_to_uci, APP_DESCRIPTION, APP_NAME, VERSION};
use shakmaty::{Color, Move, Position, Square};
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppMode {
    Play,
    Opening,
    Tactics,
}

impl AppMode {
    fn as_str(self) -> &'static str {
        match self {
            AppMode::Play => "play",
            AppMode::Opening => "openings",
            AppMode::Tactics => "tactics",
        }
    }
}

struct PendingPromotion {
    from: Square,
    to: Square,
}

struct AppState {
    game: ChessGame,
    ai: ChessAI,
    player_color: Color,
    selected: Option<Square>,
    legal_targets: Vec<Square>,
    difficulty: String,
    theme: String,
    mode: AppMode,
    pending_promo: Option<PendingPromotion>,
    opening: Option<OpeningTrainer>,
    puzzle: Option<PuzzleSession>,
    pgn_status: String,
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
            theme: "classic".into(),
            mode: AppMode::Play,
            pending_promo: None,
            opening: None,
            puzzle: None,
            pgn_status: String::new(),
        }
    }

    fn clear_selection(&mut self) {
        self.selected = None;
        self.legal_targets.clear();
    }

    fn reset_play_game(&mut self) {
        self.game = ChessGame::new();
        self.clear_selection();
        self.pending_promo = None;
        self.opening = None;
        self.puzzle = None;
        self.mode = AppMode::Play;
        self.pgn_status.clear();
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
    Modes: Play vs Computer, Opening trainer, Tactics puzzles
    Click a piece, then a destination square to move
    Promotion dialog for queen/rook/bishop/knight
    Themes: Blue, Wood, Green boards
    Side panel: difficulty, hint, undo, export PGN, modes

INSTALL:
    ./scripts/build-and-install.sh
    make install

ENVIRONMENT:
    Uses the system display (X11 or Wayland) via Slint/winit.
"
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    for arg in std::env::args().skip(1) {
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
        let st = state.lock().expect("lock");
        refresh_ui(&ui, &st);
    }
    ui.set_show_color_picker(true);
    ui.set_difficulty(SharedString::from("medium"));
    ui.set_theme(SharedString::from("classic"));
    ui.set_mode(SharedString::from("play"));

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_choose_color(move |color| {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            st.player_color = if color.as_str() == "black" {
                Color::Black
            } else {
                Color::White
            };
            if st.mode == AppMode::Play {
                st.game = ChessGame::new();
                st.clear_selection();
                st.pending_promo = None;
            }
            ui.set_show_color_picker(false);
            ui.set_board_flipped(st.player_color == Color::Black);
            refresh_ui(&ui, &st);
            drop(st);
            maybe_continue(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_square_clicked(move |index| {
            let ui = ui_weak.unwrap();
            if ui.get_ai_thinking()
                || ui.get_show_color_picker()
                || ui.get_show_promotion()
                || ui.get_show_mode_picker()
            {
                return;
            }

            let mut st = state.lock().expect("lock");
            if st.mode == AppMode::Play && st.game.is_game_over() {
                return;
            }
            if st.mode == AppMode::Play && st.game.current_turn() != st.player_color {
                return;
            }
            let Some(sq) = square_from_index(index) else {
                return;
            };

            if let Some(from) = st.selected {
                if from == sq {
                    st.clear_selection();
                    refresh_ui(&ui, &st);
                    return;
                }
                if st.legal_targets.contains(&sq) {
                    if st.game.requires_promotion(from, sq) {
                        st.pending_promo = Some(PendingPromotion { from, to: sq });
                        st.clear_selection();
                        ui.set_show_promotion(true);
                        refresh_ui(&ui, &st);
                        return;
                    }
                    if let Some(mv) = st.game.find_legal_move(from, sq, None) {
                        drop(st);
                        apply_human_move(&ui, &state, mv);
                        return;
                    }
                }
            }

            if let Some(piece) = st.game.board().board().piece_at(sq) {
                let can_select = match st.mode {
                    AppMode::Play => {
                        piece.color == st.player_color
                            && st.game.current_turn() == st.player_color
                    }
                    AppMode::Opening | AppMode::Tactics => piece.color == st.game.current_turn(),
                };
                if can_select {
                    let dests = st.game.legal_destinations(sq);
                    if !dests.is_empty() {
                        st.selected = Some(sq);
                        st.legal_targets = dests;
                        refresh_ui(&ui, &st);
                        return;
                    }
                }
            }
            st.clear_selection();
            refresh_ui(&ui, &st);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_promote(move |code| {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            let Some(role) = role_from_promo_char(code.chars().next().unwrap_or('q')) else {
                return;
            };
            let Some(pending) = st.pending_promo.take() else {
                return;
            };
            ui.set_show_promotion(false);
            if let Some(mv) = st.game.find_legal_move(pending.from, pending.to, Some(role)) {
                drop(st);
                apply_human_move(&ui, &state, mv);
            } else {
                refresh_ui(&ui, &st);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_cancel_promote(move || {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            st.pending_promo = None;
            ui.set_show_promotion(false);
            refresh_ui(&ui, &st);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_new_game(move || {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            st.reset_play_game();
            ui.set_show_promotion(false);
            ui.set_show_mode_picker(false);
            ui.set_show_color_picker(true);
            ui.set_mode(SharedString::from("play"));
            ui.set_hint_text(SharedString::from(""));
            refresh_ui(&ui, &st);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_undo(move || {
            let ui = ui_weak.unwrap();
            if ui.get_ai_thinking() {
                return;
            }
            let mut st = state.lock().expect("lock");
            let _ = st.game.undo_move();
            if st.mode == AppMode::Play
                && st.game.current_turn() != st.player_color
                && !st.game.move_history().is_empty()
            {
                let _ = st.game.undo_move();
            }
            let hist_len = st.game.move_history().len();
            if let Some(ref mut op) = st.opening {
                op.ply = hist_len.min(op.line.moves_uci.len());
            }
            st.clear_selection();
            st.pending_promo = None;
            ui.set_show_promotion(false);
            ui.set_hint_text(SharedString::from(""));
            refresh_ui(&ui, &st);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_hint(move || {
            let ui = ui_weak.unwrap();
            if ui.get_ai_thinking() {
                return;
            }
            let mut st = state.lock().expect("lock");
            if st.mode != AppMode::Play
                || st.game.current_turn() != st.player_color
                || st.game.is_game_over()
            {
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
                    let dests = st.game.legal_destinations(from);
                    // Prefer highlighting the actual move destination.
                    let to = if mv.is_castle() {
                        dests
                            .iter()
                            .copied()
                            .find(|d| {
                                st.game
                                    .find_legal_move(from, *d, None)
                                    .map(|m| m.is_castle())
                                    .unwrap_or(false)
                            })
                            .unwrap_or(mv.to())
                    } else {
                        mv.to()
                    };
                    st.legal_targets = vec![to];
                }
                refresh_ui(&ui, &st);
            } else {
                ui.set_hint_text(SharedString::from("No hint available."));
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_set_difficulty(move |level| {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            st.difficulty = level.to_string();
            st.ai.set_difficulty(level.as_str());
            ui.set_difficulty(level);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_set_theme(move |theme| {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            st.theme = theme.to_string();
            ui.set_theme(theme);
            refresh_ui(&ui, &st);
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_show_modes(move || {
            ui_weak.unwrap().set_show_mode_picker(true);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_set_mode(move |mode| {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            ui.set_show_mode_picker(false);
            match mode.as_str() {
                "tactics" => {
                    st.mode = AppMode::Tactics;
                    st.opening = None;
                    st.puzzle = PuzzleSession::first();
                    if let Some(ref p) = st.puzzle {
                        match p.load_game() {
                            Ok(g) => {
                                st.game = g;
                                st.player_color = st.game.current_turn();
                            }
                            Err(e) => st.pgn_status = e,
                        }
                    }
                    st.clear_selection();
                    ui.set_board_flipped(st.player_color == Color::Black);
                    ui.set_show_color_picker(false);
                }
                "play" => {
                    st.reset_play_game();
                    ui.set_show_color_picker(true);
                }
                _ => {}
            }
            ui.set_mode(SharedString::from(st.mode.as_str()));
            refresh_ui(&ui, &st);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_start_opening(move |id| {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            ui.set_show_mode_picker(false);
            let Some(trainer) = OpeningTrainer::start(id.as_str(), true) else {
                return;
            };
            st.mode = AppMode::Opening;
            st.opening = Some(trainer);
            st.puzzle = None;
            st.game = ChessGame::new();
            st.player_color = Color::White;
            st.clear_selection();
            ui.set_board_flipped(false);
            ui.set_show_color_picker(false);
            ui.set_mode(SharedString::from("openings"));
            refresh_ui(&ui, &st);
            drop(st);
            maybe_continue(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_next_puzzle(move || {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            st.puzzle = st
                .puzzle
                .take()
                .and_then(|p| p.next())
                .or_else(PuzzleSession::first);
            if let Some(ref p) = st.puzzle {
                if let Ok(g) = p.load_game() {
                    st.game = g;
                    st.player_color = st.game.current_turn();
                    ui.set_board_flipped(st.player_color == Color::Black);
                }
            }
            st.clear_selection();
            st.mode = AppMode::Tactics;
            ui.set_hint_text(SharedString::from(""));
            ui.set_mode(SharedString::from("tactics"));
            refresh_ui(&ui, &st);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_export_pgn(move || {
            let ui = ui_weak.unwrap();
            let mut st = state.lock().expect("lock");
            let (white, black) = match st.player_color {
                Color::White => ("Human", "Computer"),
                Color::Black => ("Computer", "Human"),
            };
            let pgn = st.game.to_pgn(white, black);
            let path = default_pgn_path();
            st.pgn_status = match std::fs::write(&path, &pgn) {
                Ok(()) => format!("Saved {}", path.display()),
                Err(e) => format!("PGN save failed: {e}"),
            };
            refresh_ui(&ui, &st);
        });
    }

    ui.run()?;
    Ok(())
}

fn apply_human_move(ui: &AppWindow, state: &Arc<Mutex<AppState>>, mv: Move) {
    let uci = move_to_uci(mv);
    let mut st = state.lock().expect("lock");

    match st.mode {
        AppMode::Opening => {
            if let Some(ref mut op) = st.opening {
                if !op.try_human_move(&uci) {
                    ui.set_hint_text(SharedString::from(format!(
                        "Book expects {}.",
                        op.expected_uci().unwrap_or("?")
                    )));
                    st.clear_selection();
                    refresh_ui(ui, &st);
                    return;
                }
            }
            st.game.make_move(mv);
            st.clear_selection();
            if st.opening.as_ref().map(|o| o.complete()).unwrap_or(false) {
                ui.set_hint_text(SharedString::from("Opening line complete!"));
            } else {
                ui.set_hint_text(SharedString::from(""));
            }
            refresh_ui(ui, &st);
            drop(st);
            maybe_continue(ui, state);
        }
        AppMode::Tactics => {
            st.game.make_move(mv);
            if let Some(ref mut pz) = st.puzzle {
                if pz.try_solution(&uci) {
                    ui.set_hint_text(SharedString::from("Solved! Click Next Puzzle."));
                } else {
                    ui.set_hint_text(SharedString::from("Not the key move."));
                }
            }
            st.clear_selection();
            refresh_ui(ui, &st);
        }
        AppMode::Play => {
            st.game.make_move(mv);
            st.clear_selection();
            ui.set_hint_text(SharedString::from(""));
            refresh_ui(ui, &st);
            drop(st);
            maybe_continue(ui, state);
        }
    }
}

fn maybe_continue(ui: &AppWindow, state: &Arc<Mutex<AppState>>) {
    // Opening book replies for the computer side.
    loop {
        let mut st = state.lock().expect("lock");
        if st.mode != AppMode::Opening {
            break;
        }
        let Some(ref mut op) = st.opening else {
            break;
        };
        if op.complete() || op.human_to_move() {
            break;
        }
        let Some(uci) = op.advance_computer() else {
            break;
        };
        if !st.game.make_move_uci(uci) {
            break;
        }
        refresh_ui(ui, &st);
    }

    maybe_start_ai_move(ui, state);
}

fn maybe_start_ai_move(ui: &AppWindow, state: &Arc<Mutex<AppState>>) {
    let (board, depth) = {
        let st = state.lock().expect("lock");
        if st.mode != AppMode::Play {
            return;
        }
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
            let mut st = state.lock().expect("lock");
            if let Some(mv) = best {
                st.game.make_move(mv);
            }
            st.clear_selection();
            ui.set_ai_thinking(false);
            ui.set_hint_text(SharedString::from(""));
            refresh_ui(&ui, &st);
        });
    });
}

fn refresh_ui(ui: &AppWindow, st: &AppState) {
    let last = st.game.last_move();
    let last_from = last.and_then(|m| m.from());
    let last_to = last.map(|m| {
        if m.is_castle() {
            // highlight king destination approx via board state after move is hard;
            // use rook square or from/to both.
            m.to()
        } else {
            m.to()
        }
    });

    let mut squares = Vec::with_capacity(64);
    for idx in 0..64i32 {
        let sq = square_from_index(idx).unwrap();
        let file = u32::from(sq) % 8;
        let rank = u32::from(sq) / 8;
        let is_light = (file + rank) % 2 == 1;
        squares.push(SquareData {
            piece: SharedString::from(st.game.piece_glyph_at(sq)),
            is_light,
            is_selected: st.selected == Some(sq),
            is_target: st.legal_targets.contains(&sq),
            is_last_move: last_from == Some(sq) || last_to == Some(sq),
            index: idx,
        });
    }
    ui.set_squares(ModelRc::new(VecModel::from(squares)));
    ui.set_status_text(SharedString::from(st.game.status_text()));
    ui.set_eval_text(SharedString::from(format!(
        "Eval: {:+} (side to move)",
        st.ai.evaluate_board(st.game.board())
    )));
    ui.set_move_list_text(SharedString::from(st.game.move_list_text()));
    ui.set_theme(SharedString::from(st.theme.as_str()));
    ui.set_mode(SharedString::from(st.mode.as_str()));
    ui.set_pgn_status(SharedString::from(st.pgn_status.as_str()));

    if let Some(san) = st.game.last_san() {
        ui.set_last_move_text(SharedString::from(format!("Last: {san}")));
    } else if let Some(mv) = last {
        let from = mv.from().map(square_name).unwrap_or_else(|| "?".into());
        let to = square_name(mv.to());
        ui.set_last_move_text(SharedString::from(format!("Last: {from}{to}")));
    } else {
        ui.set_last_move_text(SharedString::from(""));
    }

    ui.set_player_color_text(SharedString::from(format!(
        "You: {}  ·  Computer: {}",
        color_name(st.player_color),
        color_name(!st.player_color)
    )));

    let practice = match st.mode {
        AppMode::Opening => st
            .opening
            .as_ref()
            .map(|o| {
                if o.complete() {
                    format!("{} — complete!", o.line.name)
                } else {
                    format!(
                        "{} — play {} (book)",
                        o.progress_text(),
                        o.expected_uci().unwrap_or("?")
                    )
                }
            })
            .unwrap_or_default(),
        AppMode::Tactics => st
            .puzzle
            .as_ref()
            .map(|p| {
                format!(
                    "{}: {}{}",
                    p.puzzle.title,
                    p.puzzle.description,
                    if p.solved { " ✓" } else { "" }
                )
            })
            .unwrap_or_default(),
        AppMode::Play => String::new(),
    };
    ui.set_practice_text(SharedString::from(practice));
}

fn default_pgn_path() -> PathBuf {
    let mut dir = if let Ok(home) = std::env::var("HOME") {
        let mut p = PathBuf::from(home);
        p.push(".local/share/edt-chess");
        let _ = std::fs::create_dir_all(&p);
        p
    } else {
        PathBuf::from(".")
    };
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    dir.push(format!("edt-chess-{secs}.pgn"));
    dir
}
