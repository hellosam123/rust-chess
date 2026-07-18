use std::{
    hash::{BuildHasher, Hasher, RandomState},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use crate::{
    board::Board,
    constants::{CHECKMATE, INFINITE, MAX_DEPTH},
    evaluate::Evaluate,
    movegen::{MoveGenerator, MoveList},
    movepick::MovePicker,
    mv::Move,
};

pub struct PvTable {
    table: [Move; MAX_DEPTH * MAX_DEPTH],
    length: [usize; MAX_DEPTH],
}

pub struct Search {
    nodes: u64,
    root_depth: u8,
    sel_depth: usize,
    start_time: Instant,
    max_move_time: Duration,

    pv_table: PvTable,

    is_aborted: AtomicBool,
}

pub struct SearchLimits {
    pub max_depth: Option<u8>,
    pub max_move_time: Option<Duration>,
}

impl PvTable {
    pub fn new() -> Self {
        Self {
            table: [Move::NULL; MAX_DEPTH * MAX_DEPTH],
            length: [0; MAX_DEPTH],
        }
    }

    pub fn clear(&mut self) {
        self.table = [Move::NULL; MAX_DEPTH * MAX_DEPTH];
        self.length = [0; MAX_DEPTH];
    }

    pub fn store(&mut self, ply: usize, mv: Move) {
        if ply >= MAX_DEPTH - 1 {
            return;
        }

        let index = ply * MAX_DEPTH;
        let next_index = (ply + 1) * MAX_DEPTH;
        let copy_len = self.length[ply + 1];

        self.table[index] = mv;

        let src = next_index..next_index + copy_len;
        let dest = index + 1;
        self.table.copy_within(src, dest);

        self.length[ply] = copy_len + 1;
    }

    pub fn get_pv(&self) -> &[Move] {
        let len = self.length[0];
        &self.table[..len]
    }
}

impl Search {
    pub fn new() -> Self {
        Self {
            nodes: 0,
            root_depth: 0,
            sel_depth: 0,
            start_time: Instant::now(),
            max_move_time: Duration::ZERO,
            pv_table: PvTable::new(),
            is_aborted: AtomicBool::new(false),
        }
    }

    pub fn clear(&mut self) {
        self.nodes = 0;
        self.root_depth = 0;
        self.sel_depth = 0;
        self.start_time = Instant::now();
        self.max_move_time = Duration::ZERO;
        self.pv_table.clear();
        self.is_aborted.store(false, Ordering::Relaxed);
    }

    fn random(board: Board) -> Move {
        let move_list = MoveGenerator::generate_pseudo_legal_moves(&board);
        let mut legal_moves = MoveList::new();

        let pinned_mask = board.generate_pinned_mask();
        let check_mask = board.generate_check_mask();

        for i in 0..move_list.count {
            let mv = move_list.moves[i];

            if board.is_legal(mv, pinned_mask, check_mask) {
                legal_moves.push(mv);
            }
        }

        let move_index = RandomState::new().build_hasher().finish() as usize % legal_moves.count;
        legal_moves.moves[move_index]
    }

    pub fn root_search(&mut self, board: &mut Board, limits: SearchLimits) {
        self.start_time = Instant::now();
        self.max_move_time = limits.max_move_time.unwrap_or(Duration::from_millis(10000));

        let max_root_depth = limits.max_depth.unwrap_or(20);

        let mut best_move = Move::NULL;

        let pinned_mask = board.generate_pinned_mask();
        let check_mask = board.generate_check_mask();

        for current_depth in 1..=max_root_depth {
            self.root_depth = current_depth;
            self.pv_table.clear();

            let mut alpha = -INFINITE;
            let beta = INFINITE;

            let mut best_score = -INFINITE;

            let mut move_picker = MovePicker::new(best_move);
            while let Some(mv) = move_picker.next(self, board) {
                if !board.is_legal(mv, pinned_mask, check_mask) {
                    continue;
                }

                let unmove = board.make_move(mv);
                let score =
                    -Self::alpha_beta_search(self, board, alpha, beta, current_depth - 1, 1);
                board.unmake_move(mv, unmove);

                self.nodes += 1;

                if self.is_aborted.load(Ordering::Relaxed) {
                    break;
                }

                if score > best_score {
                    best_score = score;

                    if score > alpha {
                        alpha = score;
                        self.pv_table.store(0, mv);
                    }
                }
            }

            if self.is_aborted.load(Ordering::Relaxed) {
                break;
            }

            if best_score == -INFINITE {
                best_score = -CHECKMATE;
            }

            let pv = self.pv_table.get_pv();
            best_move = pv[0];
            let mut pv_str = String::with_capacity(pv.len() * 6);
            for mv in pv {
                pv_str.push_str(&mv.to_string());
                pv_str.push(' ');
            }
            pv_str.pop();

            let mut elapsed = self.start_time.elapsed().as_millis();
            if elapsed == 0 {
                elapsed = 1;
            }
            let nps = self.nodes * 1000 / elapsed as u64;

            println!(
                "info depth {} seldepth {} score cp {} nodes {} nps {} time {} pv {}",
                current_depth, self.sel_depth, best_score, self.nodes, nps, elapsed, pv_str
            );
        }

        println!("bestmove {}", best_move);
    }

    fn alpha_beta_search(
        &mut self,
        board: &mut Board,
        mut alpha: i32,
        beta: i32,
        depth: u8,
        ply: usize,
    ) -> i32 {
        if depth == 0 {
            return Self::quiescence_search(self, board, alpha, beta, ply);
        }

        if self.is_aborted.load(Ordering::Relaxed) {
            return 0;
        }

        if ply > self.sel_depth {
            self.sel_depth = ply;
        }

        let mut best_score = -INFINITE;

        let pinned_mask = board.generate_pinned_mask();
        let check_mask = board.generate_check_mask();

        let mut move_picker = MovePicker::new(Move::NULL);
        while let Some(mv) = move_picker.next(self, board) {
            if !board.is_legal(mv, pinned_mask, check_mask) {
                continue;
            }

            let unmove = board.make_move(mv);
            let score = -Self::alpha_beta_search(self, board, -beta, -alpha, depth - 1, ply + 1);
            board.unmake_move(mv, unmove);

            self.nodes += 1;
            if self.nodes.is_multiple_of(2048) && self.start_time.elapsed() > self.max_move_time {
                self.is_aborted.store(true, Ordering::Relaxed);
            }

            if score > best_score {
                best_score = score;

                if score > alpha {
                    alpha = score;
                    self.pv_table.store(ply, mv);
                }
            }

            if score >= beta {
                return best_score;
            }
        }

        if best_score == -INFINITE {
            return -CHECKMATE;
        }

        best_score
    }

    fn quiescence_search(
        &mut self,
        board: &mut Board,
        mut alpha: i32,
        beta: i32,
        ply: usize,
    ) -> i32 {
        let static_eval = Evaluate::sided_eval(board);

        let mut best_score = static_eval;
        if best_score >= beta {
            return best_score;
        }
        if best_score > alpha {
            alpha = best_score;
        }

        if ply > self.sel_depth {
            self.sel_depth = ply;
        }

        let move_list = MoveGenerator::generate_pseudo_legal_moves(board);

        let pinned_mask = board.generate_pinned_mask();
        let check_mask = board.generate_check_mask();
        for i in 0..move_list.count {
            let mv = move_list.moves[i];
            if !mv.is_capture() {
                continue;
            }

            if !board.is_legal(mv, pinned_mask, check_mask) {
                continue;
            }

            let unmove = board.make_move(mv);
            let score = -Self::quiescence_search(self, board, -beta, -alpha, ply + 1);
            board.unmake_move(mv, unmove);

            self.nodes += 1;

            if score >= beta {
                return beta;
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
            }
        }

        best_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_search() {
        let mut board = Board::new();
        board
            .parse_fen("r1bk1b1r/ppp2ppp/2p5/4Pn2/8/5N2/PPP2PPP/RNB2RK1 w - - 0 9")
            .unwrap();
        board.print_board();

        let random_move = Search::random(board);
        println!("{}", random_move);
    }

    #[test]
    fn test_search() {
        let mut board = Board::new_starting_board().unwrap();
        board.print_board();

        let mut search = Search::new();
        let limits = SearchLimits {
            max_depth: Some(7),
            max_move_time: Some(Duration::MAX),
        };
        search.root_search(&mut board, limits);
    }
}
