use crate::{board::Board, constants::MAX_MOVES, movegen::MoveGenerator, mv::Move, search::Search};

#[derive(Debug, Clone, Copy)]
struct MoveEntry {
    mv: Move,
    score: i32,
}

struct EntryList {
    entries: [MoveEntry; MAX_MOVES],
    count: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScoreStage {
    HashMove,
    GenerateAll,
    Yielding,
    Done,
}

pub struct MovePicker {
    entry_list: EntryList,
    tt_move: Move,
    stage: ScoreStage,
}

impl Default for EntryList {
    fn default() -> Self {
        Self {
            entries: [MoveEntry {
                mv: Move::NULL,
                score: 0,
            }; MAX_MOVES],
            count: 0,
        }
    }
}

impl EntryList {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, entry: MoveEntry) {
        self.entries[self.count] = entry;
        self.count += 1;
    }

    fn swap_remove(&mut self, index: usize) -> Option<MoveEntry> {
        if index >= self.count {
            return None;
        }

        let removed_entry = self.entries[index];
        self.entries[index] = self.entries[self.count - 1];
        self.count -= 1;

        Some(removed_entry)
    }
}

impl MovePicker {
    pub fn new(tt_move: Move) -> Self {
        Self {
            entry_list: EntryList::new(),
            tt_move,
            stage: if tt_move.is_present() {
                ScoreStage::HashMove
            } else {
                ScoreStage::GenerateAll
            },
        }
    }

    pub fn next(&mut self, search: &Search, board: &Board) -> Option<Move> {
        loop {
            match self.stage {
                ScoreStage::HashMove => {
                    self.stage = ScoreStage::GenerateAll;
                    return Some(self.tt_move);
                }

                ScoreStage::GenerateAll => {
                    self.stage = ScoreStage::Yielding;
                    let move_list = MoveGenerator::generate_pseudo_legal_moves(board);
                    for i in 0..move_list.count {
                        let mv = move_list.moves[i];
                        if mv == self.tt_move {
                            continue;
                        }

                        let score = if mv.is_capture() {
                            self.score_capture(mv, search, board)
                        } else {
                            self.score_quiet(mv, search, board)
                        };
                        self.entry_list.push(MoveEntry { mv, score });
                    }
                }

                ScoreStage::Yielding => {
                    if self.entry_list.count == 0 {
                        self.stage = ScoreStage::Done;
                        continue;
                    }

                    let entry = self.pop_best_entry();
                    return entry.map(|e| e.mv);
                }

                ScoreStage::Done => {
                    return None;
                }
            }
        }
    }

    fn pop_best_entry(&mut self) -> Option<MoveEntry> {
        let mut best_index = 0;
        let mut best_score = i32::MIN;

        for i in 0..self.entry_list.count {
            let entry = self.entry_list.entries[i];
            if entry.score > best_score {
                best_score = entry.score;
                best_index = i;
            }
        }

        self.entry_list.swap_remove(best_index)
    }

    fn score_capture(&self, mv: Move, search: &Search, board: &Board) -> i32 {
        let victim_value = if mv.is_en_passant() {
            100
        } else {
            board
                .get_piece(mv.get_to())
                .map(|p| p.get_value())
                .unwrap_or(0)
        };

        let aggressor_value = board
            .get_piece(mv.get_from())
            .map(|p| p.get_value())
            .unwrap_or(0);

        1000000 + victim_value * 10 - aggressor_value
    }

    fn score_quiet(&self, mv: Move, search: &Search, board: &Board) -> i32 {
        0
    }
}
