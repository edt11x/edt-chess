//! Practice modes: opening trainer and tactics puzzles.

use crate::game::ChessGame;

/// Built-in opening line for the trainer.
#[derive(Debug, Clone)]
pub struct OpeningLine {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    /// UCI moves from the starting position (both sides).
    pub moves_uci: &'static [&'static str],
}

/// Built-in tactics puzzle.
#[derive(Debug, Clone)]
pub struct Puzzle {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub fen: &'static str,
    /// Best move(s) in UCI that solve the puzzle (side to move in FEN).
    pub solution_uci: &'static [&'static str],
}

pub fn openings() -> &'static [OpeningLine] {
    &[
        OpeningLine {
            id: "italian",
            name: "Italian Game",
            description: "1.e4 e5 2.Nf3 Nc6 3.Bc4 — classic open game.",
            moves_uci: &["e2e4", "e7e5", "g1f3", "b8c6", "f1c4"],
        },
        OpeningLine {
            id: "ruy-lopez",
            name: "Ruy Lopez",
            description: "1.e4 e5 2.Nf3 Nc6 3.Bb5 — Spanish Opening.",
            moves_uci: &["e2e4", "e7e5", "g1f3", "b8c6", "f1b5"],
        },
        OpeningLine {
            id: "sicilian",
            name: "Sicilian Defence",
            description: "1.e4 c5 — Black's most popular fighting response.",
            moves_uci: &["e2e4", "c7c5", "g1f3", "d7d6", "d2d4"],
        },
        OpeningLine {
            id: "queens-gambit",
            name: "Queen's Gambit",
            description: "1.d4 d5 2.c4 — central pressure with the c-pawn.",
            moves_uci: &["d2d4", "d7d5", "c2c4"],
        },
        OpeningLine {
            id: "london",
            name: "London System",
            description: "1.d4 d5 2.Nf3 Nf6 3.Bf4 — solid system opening.",
            moves_uci: &["d2d4", "d7d5", "g1f3", "g8f6", "c1f4"],
        },
    ]
}

pub fn puzzles() -> &'static [Puzzle] {
    &[
        Puzzle {
            id: "mate1-backrank",
            title: "Mate in 1 — Back rank",
            description: "White to move and checkmate on the back rank.",
            fen: "6k1/5ppp/8/8/8/8/8/4Q1K1 w - - 0 1",
            solution_uci: &["e1e8"],
        },
        Puzzle {
            id: "mate1-qxf7",
            title: "Mate in 1 — Scholar pattern",
            description: "White to move and mate with Qxf7#.",
            fen: "r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4",
            solution_uci: &["h5f7"],
        },
        Puzzle {
            id: "mate1-smother",
            title: "Mate in 1 — Corridor",
            description: "White rook mates on the 8th rank.",
            fen: "6k1/5ppp/8/8/8/8/8/R5K1 w - - 0 1",
            solution_uci: &["a1a8"],
        },
        Puzzle {
            id: "capture-hanging",
            title: "Tactics — Take free material",
            description: "White to move: capture the unprotected black queen on d4.",
            // d-pawn removed so Qd1xd4 is legal.
            fen: "rnb1kbnr/pppppppp/8/8/3q4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1",
            solution_uci: &["d1d4"],
        },
    ]
}

/// Opening trainer session state.
#[derive(Debug, Clone)]
pub struct OpeningTrainer {
    pub line: OpeningLine,
    pub ply: usize,
    pub human_is_white: bool,
}

impl OpeningTrainer {
    pub fn start(line_id: &str, human_is_white: bool) -> Option<Self> {
        let line = openings().iter().find(|o| o.id == line_id)?.clone();
        Some(Self {
            line,
            ply: 0,
            human_is_white,
        })
    }

    pub fn complete(&self) -> bool {
        self.ply >= self.line.moves_uci.len()
    }

    pub fn expected_uci(&self) -> Option<&'static str> {
        self.line.moves_uci.get(self.ply).copied()
    }

    pub fn human_to_move(&self) -> bool {
        if self.complete() {
            return false;
        }
        let white_to_play = self.ply % 2 == 0;
        white_to_play == self.human_is_white
    }

    pub fn try_human_move(&mut self, uci: &str) -> bool {
        if !self.human_to_move() {
            return false;
        }
        match self.expected_uci() {
            Some(exp) if exp == uci => {
                self.ply += 1;
                true
            }
            _ => false,
        }
    }

    pub fn advance_computer(&mut self) -> Option<&'static str> {
        if self.human_to_move() || self.complete() {
            return None;
        }
        let mv = self.expected_uci()?;
        self.ply += 1;
        Some(mv)
    }

    pub fn progress_text(&self) -> String {
        format!(
            "{} — step {}/{}",
            self.line.name,
            self.ply.min(self.line.moves_uci.len()),
            self.line.moves_uci.len()
        )
    }
}

/// Puzzle session.
#[derive(Debug, Clone)]
pub struct PuzzleSession {
    pub puzzle: Puzzle,
    pub index: usize,
    pub solved: bool,
}

impl PuzzleSession {
    pub fn count() -> usize {
        puzzles().len()
    }

    pub fn at(index: usize) -> Option<Self> {
        let p = puzzles().get(index)?;
        Some(Self {
            puzzle: p.clone(),
            index,
            solved: false,
        })
    }

    pub fn first() -> Option<Self> {
        Self::at(0)
    }

    pub fn next(&self) -> Option<Self> {
        let n = Self::count();
        if n == 0 {
            return None;
        }
        Self::at((self.index + 1) % n)
    }

    pub fn try_solution(&mut self, uci: &str) -> bool {
        if self.solved {
            return true;
        }
        if self.puzzle.solution_uci.iter().any(|s| *s == uci) {
            self.solved = true;
            true
        } else {
            false
        }
    }

    pub fn load_game(&self) -> Result<ChessGame, String> {
        ChessGame::from_fen(self.puzzle.fen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openings_nonempty_and_legal() {
        assert!(!openings().is_empty());
        for o in openings() {
            let mut g = ChessGame::new();
            for mv in o.moves_uci {
                assert!(g.make_move_uci(mv), "{} failed on {mv}", o.id);
            }
        }
    }

    #[test]
    fn puzzles_load_and_solutions_legal() {
        for p in puzzles() {
            let g = ChessGame::from_fen(p.fen).expect(p.id);
            assert!(!g.is_game_over(), "{} already over", p.id);
            let mut any = false;
            for sol in p.solution_uci {
                let mut g2 = ChessGame::from_fen(p.fen).unwrap();
                if g2.make_move_uci(sol) {
                    any = true;
                }
            }
            assert!(any, "{} no legal solution", p.id);
        }
    }

    #[test]
    fn mate_puzzles_actually_mate() {
        for id in ["mate1-backrank", "mate1-qxf7", "mate1-smother"] {
            let p = puzzles().iter().find(|p| p.id == id).unwrap();
            let mut g = ChessGame::from_fen(p.fen).unwrap();
            assert!(g.make_move_uci(p.solution_uci[0]));
            assert!(g.is_game_over(), "{id} should mate");
            assert_eq!(g.winner_name(), Some("White"));
        }
    }

    #[test]
    fn opening_trainer_advances() {
        let mut t = OpeningTrainer::start("italian", true).unwrap();
        assert!(t.try_human_move("e2e4"));
        assert_eq!(t.advance_computer(), Some("e7e5"));
    }

    #[test]
    fn puzzle_session_accepts_solution() {
        let mut s = PuzzleSession::first().unwrap();
        assert!(s.try_solution(s.puzzle.solution_uci[0]));
        assert!(s.solved);
    }
}
