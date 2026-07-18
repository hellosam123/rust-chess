use crate::{
    constants::{CHECKMATE, CHECKMATE_THRESHOLD},
    mv::Move,
};

pub struct TranspositionTable {
    entry_list: Vec<TtEntry>,
    pub size_mask: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashFlag {
    Exact,
    Alpha,
    Beta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TtEntry {
    hash_key: u64,
    mv: Move,
    score: i16,
    age: u16,
    depth: u8,
    flag: HashFlag,
}

impl TtEntry {
    pub const NULL: Self = Self {
        hash_key: 0,
        mv: Move::NULL,
        score: 0,
        age: 0,
        depth: 0,
        flag: HashFlag::Alpha,
    };

    #[inline(always)]
    pub fn new(hash_key: u64, mv: Move, score: i16, age: u16, depth: u8, flag: HashFlag) -> Self {
        Self {
            hash_key,
            mv,
            score,
            age,
            depth,
            flag,
        }
    }

    #[inline(always)]
    pub fn to_tt_score(score: i32, ply: usize) -> i16 {
        if score > CHECKMATE_THRESHOLD {
            score as i16 - ply as i16
        } else if score < -CHECKMATE_THRESHOLD {
            score as i16 + ply as i16
        } else {
            score as i16
        }
    }

    #[inline(always)]
    pub fn get_score(&self, ply: usize) -> i32 {
        if (self.score as i32) > CHECKMATE_THRESHOLD {
            self.score as i32 + ply as i32
        } else if (self.score as i32) < -CHECKMATE_THRESHOLD {
            self.score as i32 - ply as i32
        } else {
            self.score as i32
        }
    }

    #[inline(always)]
    pub fn get_depth(&self) -> u8 {
        self.depth
    }

    #[inline(always)]
    pub fn get_flag(&self) -> HashFlag {
        self.flag
    }
}

impl TranspositionTable {
    #[inline(always)]
    pub fn new(megabytes: usize) -> Self {
        let entry_size = size_of::<TtEntry>();
        let bytes = megabytes * 1024 * 1024;

        let mut num_entries = bytes / entry_size;

        if !num_entries.is_power_of_two() {
            num_entries = num_entries.next_power_of_two() >> 1;
        }

        Self {
            entry_list: vec![TtEntry::NULL; num_entries],
            size_mask: num_entries - 1,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.entry_list.fill(TtEntry::NULL);
    }

    #[inline(always)]
    pub fn read(&self, hash_key: u64) -> Option<TtEntry> {
        let index = hash_key as usize & self.size_mask;
        let entry = self.entry_list[index];

        if entry.hash_key == hash_key {
            Some(entry)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn write(&mut self, entry: TtEntry) {
        let index = entry.hash_key as usize & self.size_mask;
        let old_entry = self.entry_list[index];
        if old_entry == TtEntry::NULL
            || entry.depth >= old_entry.depth
            || entry.age != old_entry.age
        {
            self.entry_list[index] = entry
        }
    }
}
