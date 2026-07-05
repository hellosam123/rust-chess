use crate::zobrist::ZOBRIST;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8 = 63,
}

#[derive(Debug)]
pub struct Board {
    /// Bitboards divided by Piece
    /// Pieces are in order: P, N, B, R, Q, K
    /// White pieces for indices 0-5, BLack for 6-11
    pub pieces: [u64; 12],

    /// A square centric board representation
    /// Used for constant piece lookups
    pub mailbox: [Option<Piece>; 64],

    /// History of zobrist hash keys
    pub history: Vec<u64>,

    /// Zobrist hash key for current position
    pub hash_key: u64,

    pub en_passant_square: Option<u8>,
    pub half_move_clock: u8,

    pub phase_value: u8,

    pub castling_rights: CastlingRights,
    pub active_color: Color,
}

#[derive(Debug)]
pub struct CastlingRights(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WhitePawn = 0,
    WhiteKnight = 1,
    WhiteBishop = 2,
    WhiteRook = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackKnight = 7,
    BlackBishop = 8,
    BlackRook = 9,
    BlackQueen = 10,
    BlackKing = 11,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    #[default]
    White = 0,
    Black = 1,
}

fn str_to_square(s: &str) -> Result<u8, &'static str> {
    if s.len() != 2 {
        return Err("square string length must equal 2");
    }

    let mut chars = s.chars();
    let file_char = chars.next().unwrap();
    let rank_char = chars.next().unwrap();

    if !('a'..='h').contains(&file_char) {
        return Err("file char not in range a-h");
    }

    let file = (file_char as u8) - b'a'; // Converts file character to int in range 0-7

    if !('1'..='8').contains(&rank_char) {
        return Err("rank char not in range 1-8");
    }

    let rank = (rank_char as u8) - b'1';

    Ok(rank * 8 + file)
}

impl TryFrom<usize> for Piece {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let piece = match value {
            0 => Piece::WhitePawn,
            1 => Piece::WhiteKnight,
            2 => Piece::WhiteBishop,
            3 => Piece::WhiteRook,
            4 => Piece::WhiteQueen,
            5 => Piece::WhiteKing,

            6 => Piece::BlackPawn,
            7 => Piece::BlackKnight,
            8 => Piece::BlackBishop,
            9 => Piece::BlackRook,
            10 => Piece::BlackQueen,
            11 => Piece::BlackKing,

            _ => return Err("index out of bounds for Piece enum"),
        };

        Ok(piece)
    }
}

impl Piece {
    pub const fn from_char(c: char) -> Result<Self, &'static str> {
        let piece = match c {
            'P' => Piece::WhitePawn,
            'N' => Piece::WhiteKnight,
            'B' => Piece::WhiteBishop,
            'R' => Piece::WhiteRook,
            'Q' => Piece::WhiteQueen,
            'K' => Piece::WhiteKing,

            'p' => Piece::BlackPawn,
            'n' => Piece::BlackKnight,
            'b' => Piece::BlackBishop,
            'r' => Piece::BlackRook,
            'q' => Piece::BlackQueen,
            'k' => Piece::BlackKing,

            _ => return Err("invalid piece char"),
        };

        Ok(piece)
    }

    pub const fn to_char(piece: Piece) -> char {
        match piece {
            Piece::WhitePawn => 'P',
            Piece::WhiteKnight => 'N',
            Piece::WhiteBishop => 'B',
            Piece::WhiteRook => 'R',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteKing => 'K',

            Piece::BlackPawn => 'p',
            Piece::BlackKnight => 'n',
            Piece::BlackBishop => 'b',
            Piece::BlackRook => 'r',
            Piece::BlackQueen => 'q',
            Piece::BlackKing => 'k',
        }
    }

    pub const fn get_phase_value(piece: Piece) -> u8 {
        match piece {
            Piece::WhitePawn | Piece::BlackPawn | Piece::WhiteKing | Piece::BlackKing => 0,
            Piece::WhiteKnight | Piece::BlackKnight | Piece::WhiteBishop | Piece::BlackBishop => 1,
            Piece::WhiteRook | Piece::BlackRook => 2,
            Piece::WhiteQueen | Piece::BlackQueen => 4,
        }
    }
}

impl CastlingRights {
    pub const WHITE_KINGSIDE: u8 = 1 << 0;
    pub const WHITE_QUEENSIDE: u8 = 1 << 1;
    pub const BLACK_KINGSIDE: u8 = 1 << 2;
    pub const BLACK_QUEENSIDE: u8 = 1 << 3;

    pub const fn has(&self, mask: u8) -> bool {
        self.0 & mask != 0
    }

    pub const fn put(&mut self, mask: u8) {
        self.0 |= mask;
    }

    pub const fn remove(&mut self, mask: u8) {
        self.0 &= !mask;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [0; 12],
            mailbox: [None; 64],
            history: Vec::with_capacity(512),
            hash_key: 0,
            en_passant_square: None,
            half_move_clock: 0,
            active_color: Color::White,
            phase_value: 0,
            castling_rights: CastlingRights(0),
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Board::default()
    }

    pub fn new_starting_board() -> Result<Self, &'static str> {
        let mut board = Board::default();
        board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
        Ok(board)
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    const fn set_piece_bit(&mut self, piece: Piece, square: u8) {
        self.pieces[piece as usize] |= 1u64 << square;
    }

    pub const fn put_piece(&mut self, piece: Piece, square: u8) {
        self.set_piece_bit(piece, square);
        self.mailbox[square as usize] = Some(piece);
        self.hash_key ^= ZOBRIST.piece_square[piece as usize][square as usize];
    }

    const fn clear_piece_bit(&mut self, piece: Piece, square: u8) {
        self.pieces[piece as usize] &= !(1 << square);
    }

    pub const fn remove_piece(&mut self, piece: Piece, square: u8) {
        self.clear_piece_bit(piece, square);
        self.mailbox[square as usize] = None;
        self.hash_key ^= ZOBRIST.piece_square[piece as usize][square as usize];
    }

    pub const fn get_piece(&self, square: u8) -> Option<Piece> {
        self.mailbox[square as usize]
    }

    pub fn print_board(&self) {
        println!("  +-----------------+");
        for rank in (0..8).rev() {
            print!("{} | ", rank + 1);
            for file in 0..8 {
                let square = rank * 8 + file;

                let mut piece_char = '.';
                if let Some(piece) = self.get_piece(square) {
                    piece_char = Piece::to_char(piece);
                }

                print!("{} ", piece_char);
            }
            println!("|");
        }
        println!("  +-----------------+");
        println!("    a b c d e f g h");
    }

    fn generate_phase_value(&self) -> u8 {
        let phase_value_pieces = [
            Piece::WhiteKnight,
            Piece::WhiteBishop,
            Piece::WhiteRook,
            Piece::WhiteQueen,
            Piece::BlackKnight,
            Piece::BlackBishop,
            Piece::BlackRook,
            Piece::BlackQueen,
        ];

        let mut phase_value = 0;
        for piece in phase_value_pieces {
            phase_value +=
                self.pieces[piece as usize].count_ones() as u8 * Piece::get_phase_value(piece);
        }

        phase_value
    }

    fn generate_hash_key(&self) -> u64 {
        let mut hash_key = 0u64;

        for square in 0..64 {
            let Some(piece) = self.get_piece(square) else {
                continue;
            };

            hash_key ^= ZOBRIST.piece_square[piece as usize][square as usize];
        }

        hash_key ^= ZOBRIST.castling_rights[self.castling_rights.0 as usize];

        if let Some(en_passant_square) = self.en_passant_square {
            let file = en_passant_square % 8;
            hash_key ^= ZOBRIST.en_passant[file as usize];
        }

        if self.active_color == Color::White {
            hash_key ^= ZOBRIST.active_color;
        }

        hash_key
    }

    fn generate_mailbox(&self) -> Result<[Option<Piece>; 64], &'static str> {
        let mut mailbox = [None; 64];

        for square in 0..64 {
            let square_mask = 1 << square;

            for (piece_index, piece_mask) in self.pieces.iter().enumerate() {
                if square_mask & piece_mask != 0 {
                    let piece = Piece::try_from(piece_index)?;
                    mailbox[square as usize] = Some(piece);
                }
            }
        }

        Ok(mailbox)
    }

    pub fn parse_fen(&mut self, fen: &str) -> Result<(), &'static str> {
        self.reset();

        let mut fields = fen.split_whitespace();

        let piece_placement = fields.next().ok_or("missing piece placement field")?;

        let active_color = fields.next().ok_or("missing active color field")?;

        let castling_rights = fields.next().ok_or("missing castling rights field")?;

        let en_passant_square = fields.next().ok_or("missing en passant square")?;

        let rank_placement_sections = piece_placement.split('/');

        for (i, rank_placement) in rank_placement_sections.enumerate() {
            let rank = 7 - i as u8; // Reversed since fen notation starts at rank 8

            let mut file = 0;

            for c in rank_placement.chars() {
                if ('1'..='8').contains(&c) {
                    let empty_squares = c.to_digit(10).unwrap_or(0) as u8;
                    file += empty_squares;
                } else {
                    if file >= 8 {
                        return Err("fen rank field overflows 8 files");
                    }
                    let piece = Piece::from_char(c)?;
                    let square = rank * 8 + file;
                    if square >= 64 {
                        return Err("fen rank or file fields overflow 8 files");
                    }
                    self.set_piece_bit(piece, square);
                    file += 1;
                }
            }
        }

        match active_color {
            "w" => self.active_color = Color::White,
            "b" => self.active_color = Color::Black,

            _ => return Err("invalid fen active color field"),
        }

        if castling_rights != "-" {
            for c in castling_rights.chars() {
                match c {
                    'K' => self.castling_rights.put(CastlingRights::WHITE_KINGSIDE),
                    'Q' => self.castling_rights.put(CastlingRights::WHITE_QUEENSIDE),
                    'k' => self.castling_rights.put(CastlingRights::BLACK_KINGSIDE),
                    'q' => self.castling_rights.put(CastlingRights::BLACK_QUEENSIDE),

                    _ => return Err("invalid fen castling rights char"),
                }
            }
        }

        if en_passant_square != "-" {
            let square = str_to_square(en_passant_square)?;

            // 16-23 and 40-47 are the equivalent of the 3rd and 6th ranks
            if !(matches!(square, 16..=23) || matches!(square, 40..=47)) {
                return Err("invalid fen en passant square");
            }

            self.en_passant_square = Some(square);
        } else {
            self.en_passant_square = None;
        }

        let mailbox = self.generate_mailbox()?;
        self.mailbox = mailbox;
        self.hash_key = self.generate_hash_key();
        self.phase_value = self.generate_phase_value();

        Ok(())
    }
}
