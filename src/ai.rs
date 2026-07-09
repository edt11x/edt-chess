//! Negamax AI with iterative deepening, transposition table, alpha-beta,
//! move ordering, and quiescence search.

use std::collections::HashMap;
use std::sync::OnceLock;

use shakmaty::zobrist::Zobrist64;
use shakmaty::{Chess, Color, EnPassantMode, Move, Position, Role};

const INF: i32 = 1_000_000;
const MATE_SCORE: i32 = -9999;

pub const fn piece_value(role: Role) -> i32 {
    match role {
        Role::Pawn => 100,
        Role::Knight => 320,
        Role::Bishop => 330,
        Role::Rook => 500,
        Role::Queen => 900,
        Role::King => 20_000,
    }
}

fn make_pst(table: &[i32; 64]) -> [i32; 64] {
    let mut result = [0; 64];
    for sq in 0..64 {
        let rank = sq / 8;
        let file = sq % 8;
        let visual_idx = (7 - rank) * 8 + file;
        result[sq] = table[visual_idx];
    }
    result
}

struct PstTables {
    pawn: [i32; 64],
    knight: [i32; 64],
    bishop: [i32; 64],
    rook: [i32; 64],
    queen: [i32; 64],
    king: [i32; 64],
}

impl PstTables {
    fn new() -> Self {
        Self {
            pawn: make_pst(&[
                0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10,
                10, 5, 5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5,
                5, 5, 10, 10, -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            knight: make_pst(&[
                -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10,
                15, 15, 10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30,
                -30, 5, 10, 15, 15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30,
                -30, -30, -40, -50,
            ]),
            bishop: make_pst(&[
                -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10,
                10, 5, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10,
                10, 10, 10, 10, 10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10,
                -10, -10, -20,
            ]),
            rook: make_pst(&[
                0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5,
                0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
                0, 0, 0, 0, -5, 0, 0, 0, 5, 5, 0, 0, 0,
            ]),
            queen: make_pst(&[
                -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5,
                0, -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0,
                -10, -10, 0, 5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
            ]),
            king: make_pst(&[
                -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30,
                -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30,
                -30, -40, -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0,
                0, 0, 20, 20, 20, 30, 10, 0, 0, 10, 30, 20,
            ]),
        }
    }

    fn for_role(&self, role: Role) -> &[i32; 64] {
        match role {
            Role::Pawn => &self.pawn,
            Role::Knight => &self.knight,
            Role::Bishop => &self.bishop,
            Role::Rook => &self.rook,
            Role::Queen => &self.queen,
            Role::King => &self.king,
        }
    }
}

fn pst_tables() -> &'static PstTables {
    static TABLES: OnceLock<PstTables> = OnceLock::new();
    TABLES.get_or_init(PstTables::new)
}

/// Map a difficulty label to max search depth (iterative deepening).
pub fn depth_for_difficulty(level: &str) -> u32 {
    match level {
        "easy" => 2,
        "medium" => 3,
        "hard" => 5,
        _ => 3,
    }
}

#[derive(Clone, Copy, Debug)]
enum TtBound {
    Exact,
    Lower,
    Upper,
}

#[derive(Clone, Copy, Debug)]
struct TtEntry {
    depth: u32,
    score: i32,
    bound: TtBound,
    best: Option<Move>,
}

struct TranspositionTable {
    map: HashMap<u64, TtEntry>,
}

impl TranspositionTable {
    fn new() -> Self {
        Self {
            map: HashMap::with_capacity(64 * 1024),
        }
    }

    fn key(board: &Chess) -> u64 {
        board.zobrist_hash::<Zobrist64>(EnPassantMode::Legal).0
    }

    fn probe(&self, board: &Chess, depth: u32, alpha: i32, beta: i32) -> Option<(i32, Option<Move>)> {
        let e = self.map.get(&Self::key(board))?;
        if e.depth < depth {
            return None;
        }
        match e.bound {
            TtBound::Exact => Some((e.score, e.best)),
            TtBound::Lower if e.score >= beta => Some((e.score, e.best)),
            TtBound::Upper if e.score <= alpha => Some((e.score, e.best)),
            _ => None,
        }
    }

    fn store(&mut self, board: &Chess, depth: u32, score: i32, bound: TtBound, best: Option<Move>) {
        let key = Self::key(board);
        if let Some(old) = self.map.get(&key) {
            if old.depth > depth {
                return;
            }
        }
        self.map.insert(
            key,
            TtEntry {
                depth,
                score,
                bound,
                best,
            },
        );
    }
}

#[derive(Debug, Clone)]
pub struct ChessAI {
    pub depth: u32,
}

impl Default for ChessAI {
    fn default() -> Self {
        Self { depth: 3 }
    }
}

impl ChessAI {
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }

    pub fn set_difficulty(&mut self, level: &str) {
        self.depth = depth_for_difficulty(level);
    }

    pub fn evaluate_board(&self, board: &Chess) -> i32 {
        if board.is_checkmate() {
            return MATE_SCORE;
        }
        if board.is_stalemate() || board.is_insufficient_material() {
            return 0;
        }

        let turn = board.turn();
        let tables = pst_tables();
        let mut score = 0i32;
        for sq in board.board().occupied() {
            let Some(piece) = board.board().piece_at(sq) else {
                continue;
            };
            let sq_idx = u32::from(sq) as usize;
            let pst_sq = if piece.color == Color::White {
                sq_idx
            } else {
                sq_idx ^ 56
            };
            let value = piece_value(piece.role) + tables.for_role(piece.role)[pst_sq];
            if piece.color == turn {
                score += value;
            } else {
                score -= value;
            }
        }
        score
    }

    fn move_order_score(&self, board: &Chess, mv: Move, tt_best: Option<Move>) -> i32 {
        if tt_best == Some(mv) {
            return 1_000_000;
        }
        let mut score = 0;
        if mv.is_capture() {
            if mv.is_en_passant() {
                score = 10 * piece_value(Role::Pawn) - piece_value(Role::Pawn);
            } else if let Some(from) = mv.from() {
                let attacker = board.board().piece_at(from);
                let victim = board.board().piece_at(mv.to());
                if let (Some(att), Some(vic)) = (attacker, victim) {
                    score = 10 * piece_value(vic.role) - piece_value(att.role);
                }
            }
        }
        if let Some(promo) = mv.promotion() {
            score += piece_value(promo);
        }
        score
    }

    fn order_moves(&self, board: &Chess, captures_only: bool, tt_best: Option<Move>) -> Vec<Move> {
        let mut scored: Vec<(i32, Move)> = board
            .legal_moves()
            .into_iter()
            .filter(|mv| !captures_only || mv.is_capture())
            .map(|mv| (self.move_order_score(board, mv, tt_best), mv))
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored.into_iter().map(|(_, m)| m).collect()
    }

    fn quiescence(&self, board: &Chess, mut alpha: i32, beta: i32) -> i32 {
        let stand_pat = self.evaluate_board(board);
        if stand_pat >= beta {
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }
        for mv in self.order_moves(board, true, None) {
            let next = board.clone().play(mv).expect("legal");
            let score = -self.quiescence(&next, -beta, -alpha);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }
        alpha
    }

    fn negamax(
        &self,
        board: &Chess,
        depth: u32,
        mut alpha: i32,
        beta: i32,
        tt: &mut TranspositionTable,
    ) -> i32 {
        let alpha_orig = alpha;

        if let Some((score, _)) = tt.probe(board, depth, alpha, beta) {
            return score;
        }

        if board.is_game_over() {
            return if board.is_checkmate() { MATE_SCORE } else { 0 };
        }
        if depth == 0 {
            return self.quiescence(board, alpha, beta);
        }

        let tt_best = tt.map.get(&TranspositionTable::key(board)).and_then(|e| e.best);
        let mut best_move = None;
        let mut best_score = -INF;

        for mv in self.order_moves(board, false, tt_best) {
            let next = board.clone().play(mv).expect("legal");
            let score = -self.negamax(&next, depth - 1, -beta, -alpha, tt);
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }

        let bound = if best_score <= alpha_orig {
            TtBound::Upper
        } else if best_score >= beta {
            TtBound::Lower
        } else {
            TtBound::Exact
        };
        tt.store(board, depth, best_score, bound, best_move);
        best_score
    }

    /// Iterative deepening search up to `self.depth`.
    pub fn get_best_move(&self, board: &Chess) -> Option<Move> {
        if board.is_game_over() {
            return None;
        }

        let mut tt = TranspositionTable::new();
        let mut best_move = None;

        for d in 1..=self.depth.max(1) {
            let mut local_best = None;
            let mut best_value = -INF;
            let mut alpha = -INF;
            let beta = INF;

            let tt_hint = best_move;
            for mv in self.order_moves(board, false, tt_hint) {
                let next = board.clone().play(mv).expect("legal");
                let score = -self.negamax(&next, d - 1, -beta, -alpha, &mut tt);
                if score > best_value {
                    best_value = score;
                    local_best = Some(mv);
                }
                if score > alpha {
                    alpha = score;
                }
            }
            if let Some(m) = local_best {
                best_move = Some(m);
            }
        }
        best_move
    }

    pub fn get_hint(&self, board: &Chess) -> Option<(Move, i32)> {
        let mv = self.get_best_move(board)?;
        let next = board.clone().play(mv).expect("legal");
        let eval = self.evaluate_board(&next);
        Some((mv, eval))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::{Chess, Position};

    #[test]
    fn ai_finds_a_legal_move() {
        let board = Chess::default();
        let ai = ChessAI::new(2);
        let mv = ai.get_best_move(&board).unwrap();
        assert!(board.is_legal(mv));
    }

    #[test]
    fn starting_eval_is_near_zero() {
        let board = Chess::default();
        let ai = ChessAI::new(2);
        let eval = ai.evaluate_board(&board);
        assert!(eval.abs() < 50, "unexpected opening eval {eval}");
    }

    #[test]
    fn hint_returns_legal_move() {
        let board = Chess::default();
        let ai = ChessAI::new(2);
        let (mv, _) = ai.get_hint(&board).unwrap();
        assert!(board.is_legal(mv));
    }

    #[test]
    fn difficulty_sets_depth() {
        let mut ai = ChessAI::default();
        ai.set_difficulty("easy");
        assert_eq!(ai.depth, 2);
        ai.set_difficulty("medium");
        assert_eq!(ai.depth, 3);
        ai.set_difficulty("hard");
        assert_eq!(ai.depth, 5);
    }

    #[test]
    fn depth_for_difficulty_table() {
        assert_eq!(depth_for_difficulty("easy"), 2);
        assert_eq!(depth_for_difficulty("medium"), 3);
        assert_eq!(depth_for_difficulty("hard"), 5);
    }

    #[test]
    fn piece_values_ordering() {
        assert!(piece_value(Role::Pawn) < piece_value(Role::Knight));
        assert!(piece_value(Role::Rook) < piece_value(Role::Queen));
    }

    #[test]
    fn checkmate_eval_is_losing_for_side_to_move() {
        let mut board = Chess::default();
        for uci in ["f2f3", "e7e5", "g2g4", "d8h4"] {
            let mv = shakmaty::uci::UciMove::from_ascii(uci.as_bytes())
                .unwrap()
                .to_move(&board)
                .unwrap();
            board = board.play(mv).unwrap();
        }
        assert!(board.is_checkmate());
        assert_eq!(ChessAI::new(1).evaluate_board(&board), MATE_SCORE);
    }

    #[test]
    fn ai_prefers_mate_in_one_when_available() {
        let mut board = Chess::default();
        for uci in ["e2e4", "e7e5", "f1c4", "b8c6", "d1h5", "g8f6"] {
            let mv = shakmaty::uci::UciMove::from_ascii(uci.as_bytes())
                .unwrap()
                .to_move(&board)
                .unwrap();
            board = board.play(mv).unwrap();
        }
        let mv = ChessAI::new(2).get_best_move(&board).unwrap();
        let next = board.play(mv).unwrap();
        assert!(next.is_checkmate(), "expected mate-in-one, got {mv:?}");
    }

    #[test]
    fn no_move_on_finished_game() {
        let mut board = Chess::default();
        for uci in ["f2f3", "e7e5", "g2g4", "d8h4"] {
            let mv = shakmaty::uci::UciMove::from_ascii(uci.as_bytes())
                .unwrap()
                .to_move(&board)
                .unwrap();
            board = board.play(mv).unwrap();
        }
        let ai = ChessAI::new(2);
        assert!(ai.get_best_move(&board).is_none());
        assert!(ai.get_hint(&board).is_none());
    }

    #[test]
    fn iterative_deepening_finds_opening_move() {
        let board = Chess::default();
        let ai = ChessAI::new(3);
        assert!(ai.get_best_move(&board).is_some());
    }
}
