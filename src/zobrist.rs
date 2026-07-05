pub struct ZobristTable {
    pub piece_square: [[u64; 64]; 12],
    pub castling_rights: [u64; 16],
    pub en_passant: [u64; 8],
    pub active_color: u64,
}

struct ConstRng {
    state: u64,
}

impl ConstRng {
    const fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}

const fn generate_zobrist_table() -> ZobristTable {
    let mut rng = ConstRng { state: 31415926535 };

    let mut piece_square = [[0u64; 64]; 12];
    let mut piece = 0;
    while piece < 12 {
        let mut square = 0;
        while square < 64 {
            piece_square[piece][square] = rng.next();
            square += 1;
        }
        piece += 1;
    }

    let mut castling_rights = [0u64; 16];
    let mut i = 0;
    while i < 16 {
        castling_rights[i] = rng.next();
        i += 1;
    }

    let mut en_passant = [0u64; 8];
    let mut j = 0;
    while j < 8 {
        en_passant[j] = rng.next();
        j += 1;
    }

    ZobristTable {
        piece_square,
        castling_rights,
        en_passant,
        active_color: rng.next(),
    }
}

pub static ZOBRIST: ZobristTable = generate_zobrist_table();
