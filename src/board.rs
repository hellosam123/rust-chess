use crate::{
    attacks,
    bitboard::CASTLING_PERMUTATIONS,
    magic_bitboard,
    mv::{Move, MoveFlag},
    zobrist::ZOBRIST,
};
use std::ops::Not;
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

const HISTORY_SIZE: usize = 1024;

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
    pub history: [u64; HISTORY_SIZE],
    pub history_ply: usize,

    /// Zobrist hash key for current position
    pub hash_key: u64,

    pub white_pieces: u64,
    pub black_pieces: u64,
    pub all_pieces: u64,

    pub en_passant_square: Option<u8>,
    pub half_move_clock: u8,

    pub phase_value: u8,

    pub castling_rights: CastlingRights,
    pub active_color: Color,
}

pub struct UnMove {
    captured_piece: Option<Piece>,
    castling_rights: CastlingRights,
    en_passant_square: Option<u8>,
    half_move_clock: u8,
}

#[derive(Debug, Clone, Copy)]
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

impl PartialEq<u8> for Square {
    fn eq(&self, other: &u8) -> bool {
        *self as u8 == *other
    }
}

impl PartialEq<Square> for u8 {
    fn eq(&self, other: &Square) -> bool {
        *self == *other as u8
    }
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
    const PAWN_PHASE_VALUE: u8 = 0;
    const KNIGHT_PHASE_VALUE: u8 = 1;
    const BISHOP_PHASE_VALUE: u8 = 1;
    const ROOK_PHASE_VALUE: u8 = 2;
    const QUEEN_PHASE_VALUE: u8 = 4;
    const KING_PHASE_VALUE: u8 = 0;

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
            Piece::WhitePawn | Piece::BlackPawn => Self::PAWN_PHASE_VALUE,
            Piece::WhiteKnight | Piece::BlackKnight => Self::KNIGHT_PHASE_VALUE,
            Piece::WhiteBishop | Piece::BlackBishop => Self::BISHOP_PHASE_VALUE,
            Piece::WhiteRook | Piece::BlackRook => Self::ROOK_PHASE_VALUE,
            Piece::WhiteQueen | Piece::BlackQueen => Self::QUEEN_PHASE_VALUE,
            Piece::WhiteKing | Piece::BlackKing => Self::KING_PHASE_VALUE,
        }
    }
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
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
            history: [0; 1024],
            history_ply: 0,
            hash_key: 0,
            white_pieces: 0,
            black_pieces: 0,
            all_pieces: 0,
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

    fn set_piece_bit(&mut self, piece: Piece, square: u8) {
        self.pieces[piece as usize] |= 1u64 << square;
    }

    fn put_piece(&mut self, piece: Piece, square: u8) {
        self.set_piece_bit(piece, square);
        self.mailbox[square as usize] = Some(piece);
        self.hash_key ^= ZOBRIST.piece_square[piece as usize][square as usize];
    }

    fn clear_piece_bit(&mut self, piece: Piece, square: u8) {
        self.pieces[piece as usize] &= !(1 << square);
    }

    fn remove_piece(&mut self, piece: Piece, square: u8) {
        self.clear_piece_bit(piece, square);
        self.mailbox[square as usize] = None;
        self.hash_key ^= ZOBRIST.piece_square[piece as usize][square as usize];
    }

    fn get_piece(&self, square: u8) -> Option<Piece> {
        self.mailbox[square as usize]
    }

    fn set_general_bitboards(&mut self) {
        self.white_pieces = self.pieces[Piece::WhitePawn as usize]
            | self.pieces[Piece::WhiteKnight as usize]
            | self.pieces[Piece::WhiteBishop as usize]
            | self.pieces[Piece::WhiteRook as usize]
            | self.pieces[Piece::WhiteQueen as usize]
            | self.pieces[Piece::WhiteKing as usize];

        self.black_pieces = self.pieces[Piece::BlackPawn as usize]
            | self.pieces[Piece::BlackKnight as usize]
            | self.pieces[Piece::BlackBishop as usize]
            | self.pieces[Piece::BlackRook as usize]
            | self.pieces[Piece::BlackQueen as usize]
            | self.pieces[Piece::BlackKing as usize];

        self.all_pieces = self.white_pieces | self.black_pieces;
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

        self.set_general_bitboards();

        Ok(())
    }

    fn push_history(&mut self) {
        self.history[self.history_ply % HISTORY_SIZE] = self.hash_key;
        self.history_ply += 1;
    }

    fn pop_history(&mut self) -> u64 {
        self.history_ply -= 1;
        self.history[self.history_ply % HISTORY_SIZE]
    }

    pub fn is_square_attacked_by(
        &self,
        square: u8,
        attacking_color: Color,
        king_xray: bool,
    ) -> bool {
        let mut occupancy = self.all_pieces;

        let attacking_pawns: u64;
        let attacking_knights: u64;
        let attacking_bishops: u64;
        let attacking_rooks: u64;
        let attacking_queens: u64;
        let attacking_king: u64;

        if attacking_color == Color::White {
            attacking_pawns = self.pieces[Piece::WhitePawn as usize];
            attacking_knights = self.pieces[Piece::WhiteKnight as usize];
            attacking_bishops = self.pieces[Piece::WhiteBishop as usize];
            attacking_rooks = self.pieces[Piece::WhiteRook as usize];
            attacking_queens = self.pieces[Piece::WhiteQueen as usize];
            attacking_king = self.pieces[Piece::WhiteKing as usize];
        } else {
            attacking_pawns = self.pieces[Piece::BlackPawn as usize];
            attacking_knights = self.pieces[Piece::BlackKnight as usize];
            attacking_bishops = self.pieces[Piece::BlackBishop as usize];
            attacking_rooks = self.pieces[Piece::BlackRook as usize];
            attacking_queens = self.pieces[Piece::BlackQueen as usize];
            attacking_king = self.pieces[Piece::BlackKing as usize];
        }
        if king_xray {
            let defending_king = if attacking_color == Color::White {
                self.pieces[Piece::BlackKing as usize]
            } else {
                self.pieces[Piece::WhiteKing as usize]
            };
            occupancy &= !defending_king;
        }
        if attacks::KNIGHT_ATTACKS[square as usize] & attacking_knights != 0 {
            return true;
        }

        if attacks::KING_ATTACKS[square as usize] & attacking_king != 0 {
            return true;
        }

        if attacking_color == Color::White {
            if attacks::BLACK_PAWN_ATTACKS[square as usize] & attacking_pawns != 0 {
                return true;
            }
        } else {
            if attacks::WHITE_PAWN_ATTACKS[square as usize] & attacking_pawns != 0 {
                return true;
            }
        }

        let bishop_attacks = magic_bitboard::get_bishop_attacks_mask(occupancy, square);
        if bishop_attacks & (attacking_bishops | attacking_queens) != 0 {
            return true;
        }

        let rook_attacks = magic_bitboard::get_rook_attacks_mask(occupancy, square);
        if rook_attacks & (attacking_rooks | attacking_queens) != 0 {
            return true;
        }

        false
    }

    pub fn make_move(&mut self, mv: Move, check_legality: bool) -> (bool, UnMove) {
        let mut unmove = UnMove {
            captured_piece: None,
            castling_rights: self.castling_rights,
            en_passant_square: self.en_passant_square,
            half_move_clock: self.half_move_clock,
        };

        let mut is_legal = true;

        self.push_history();

        if let Some(en_passant_square) = self.en_passant_square {
            self.hash_key ^= ZOBRIST.en_passant[en_passant_square as usize % 8];
            self.en_passant_square = None;
        }

        let from = mv.get_from();
        let to = mv.get_to();
        let flag = mv.get_flag();

        match flag {
            MoveFlag::DoublePawnPush => {
                let en_passant_square = if self.active_color == Color::White {
                    to - 8
                } else {
                    to + 8
                };
                self.en_passant_square = Some(en_passant_square);
                self.hash_key ^= ZOBRIST.en_passant[en_passant_square as usize % 8];
            }
            MoveFlag::KingCastle => {
                if self.active_color == Color::White {
                    self.remove_piece(Piece::WhiteRook, Square::H1 as u8);
                    self.put_piece(Piece::WhiteRook, Square::F1 as u8);

                    if check_legality
                        && (self.is_square_attacked_by(Square::E1 as u8, Color::Black, true)
                            || self.is_square_attacked_by(Square::F1 as u8, Color::Black, true))
                    {
                        is_legal = false;
                    }
                } else {
                    self.remove_piece(Piece::BlackRook, Square::H8 as u8);
                    self.put_piece(Piece::BlackRook, Square::F8 as u8);

                    if check_legality
                        && (self.is_square_attacked_by(Square::E8 as u8, Color::White, true)
                            || self.is_square_attacked_by(Square::F8 as u8, Color::White, true))
                    {
                        is_legal = false;
                    }
                }
            }
            MoveFlag::QueenCastle => {
                if self.active_color == Color::White {
                    self.remove_piece(Piece::WhiteRook, Square::A1 as u8);
                    self.put_piece(Piece::WhiteRook, Square::D1 as u8);

                    if check_legality
                        && (self.is_square_attacked_by(Square::E1 as u8, Color::Black, true)
                            || self.is_square_attacked_by(Square::D1 as u8, Color::Black, true))
                    {
                        is_legal = false;
                    }
                } else {
                    self.remove_piece(Piece::BlackRook, Square::A8 as u8);
                    self.put_piece(Piece::BlackRook, Square::D8 as u8);

                    if check_legality
                        && (self.is_square_attacked_by(Square::E8 as u8, Color::White, true)
                            || self.is_square_attacked_by(Square::D8 as u8, Color::White, true))
                    {
                        is_legal = false;
                    }
                }
            }
            MoveFlag::EnPassant => {
                if self.active_color == Color::White {
                    if let Some(captured_pawn) = self.get_piece(to - 8)
                        && captured_pawn == Piece::BlackPawn
                    {
                        self.remove_piece(Piece::BlackPawn, to - 8);
                        unmove.captured_piece = Some(Piece::BlackPawn);
                    }
                } else {
                    if let Some(captured_pawn) = self.get_piece(to + 8)
                        && captured_pawn == Piece::WhitePawn
                    {
                        self.remove_piece(Piece::WhitePawn, to + 8);
                        unmove.captured_piece = Some(Piece::WhitePawn);
                    }
                }
            }
            _ => {}
        }

        if mv.is_capture()
            && flag != MoveFlag::EnPassant
            && let Some(captured_piece) = self.get_piece(to)
        {
            self.remove_piece(captured_piece, to);
            unmove.captured_piece = Some(captured_piece);

            self.phase_value -= Piece::get_phase_value(captured_piece);
        }

        if let Some(move_piece) = self.get_piece(from) {
            self.remove_piece(move_piece, from);

            if !mv.is_promotion() {
                self.put_piece(move_piece, to);
            } else {
                match flag {
                    MoveFlag::PromoteN | MoveFlag::PromoteCaptureN => {
                        if self.active_color == Color::White {
                            self.put_piece(Piece::WhiteKnight, to);
                        } else {
                            self.put_piece(Piece::BlackKnight, to);
                        }

                        self.phase_value += Piece::KNIGHT_PHASE_VALUE;
                    }
                    MoveFlag::PromoteB | MoveFlag::PromoteCaptureB => {
                        if self.active_color == Color::White {
                            self.put_piece(Piece::WhiteBishop, to);
                        } else {
                            self.put_piece(Piece::BlackBishop, to);
                        }

                        self.phase_value += Piece::BISHOP_PHASE_VALUE;
                    }
                    MoveFlag::PromoteR | MoveFlag::PromoteCaptureR => {
                        if self.active_color == Color::White {
                            self.put_piece(Piece::WhiteRook, to);
                        } else {
                            self.put_piece(Piece::BlackRook, to);
                        }

                        self.phase_value += Piece::ROOK_PHASE_VALUE;
                    }
                    MoveFlag::PromoteQ | MoveFlag::PromoteCaptureQ => {
                        if self.active_color == Color::White {
                            self.put_piece(Piece::WhiteQueen, to);
                        } else {
                            self.put_piece(Piece::BlackQueen, to);
                        }

                        self.phase_value += Piece::QUEEN_PHASE_VALUE;
                    }
                    _ => {}
                }
            }

            self.hash_key ^= ZOBRIST.castling_rights[self.castling_rights.0 as usize];
            self.castling_rights.0 &= CASTLING_PERMUTATIONS[from as usize];
            self.castling_rights.0 &= CASTLING_PERMUTATIONS[to as usize];
            self.hash_key ^= ZOBRIST.castling_rights[self.castling_rights.0 as usize];

            if move_piece == Piece::WhitePawn || move_piece == Piece::BlackPawn || mv.is_capture() {
                self.half_move_clock = 0;
            } else {
                self.half_move_clock += 1;
            }
        }

        self.set_general_bitboards();

        if check_legality && is_legal {
            if self.active_color == Color::White
                && self.is_square_attacked_by(
                    self.pieces[Piece::WhiteKing as usize].trailing_zeros() as u8,
                    Color::Black,
                    false,
                )
            {
                is_legal = false;
            }

            if self.active_color == Color::Black
                && self.is_square_attacked_by(
                    self.pieces[Piece::BlackKing as usize].trailing_zeros() as u8,
                    Color::White,
                    false,
                )
            {
                is_legal = false;
            }
        }

        self.active_color = !self.active_color;
        self.hash_key ^= ZOBRIST.active_color;

        (is_legal, unmove)
    }

    pub fn unmake_move(&mut self, mv: Move, unmove: UnMove) {
        self.castling_rights = unmove.castling_rights;
        self.en_passant_square = unmove.en_passant_square;
        self.half_move_clock = unmove.half_move_clock;

        self.active_color = !self.active_color;
        self.hash_key = self.pop_history();

        let from = mv.get_from();
        let to = mv.get_to();
        let flag = mv.get_flag();

        if let Some(move_piece) = self.get_piece(to) {
            if !mv.is_promotion() {
                self.put_piece(move_piece, from);
                self.remove_piece(move_piece, to);
            } else {
                if self.active_color == Color::White {
                    self.put_piece(Piece::WhitePawn, from);
                } else {
                    self.put_piece(Piece::BlackPawn, from);
                }
                match flag {
                    MoveFlag::PromoteN | MoveFlag::PromoteCaptureN => {
                        if self.active_color == Color::White {
                            self.remove_piece(Piece::WhiteKnight, to);
                        } else {
                            self.remove_piece(Piece::BlackKnight, to);
                        }

                        self.phase_value -= Piece::KNIGHT_PHASE_VALUE;
                    }
                    MoveFlag::PromoteB | MoveFlag::PromoteCaptureB => {
                        if self.active_color == Color::White {
                            self.remove_piece(Piece::WhiteBishop, to);
                        } else {
                            self.remove_piece(Piece::BlackBishop, to);
                        }

                        self.phase_value -= Piece::BISHOP_PHASE_VALUE;
                    }
                    MoveFlag::PromoteR | MoveFlag::PromoteCaptureR => {
                        if self.active_color == Color::White {
                            self.remove_piece(Piece::WhiteRook, to);
                        } else {
                            self.remove_piece(Piece::BlackRook, to);
                        }

                        self.phase_value -= Piece::ROOK_PHASE_VALUE;
                    }
                    MoveFlag::PromoteQ | MoveFlag::PromoteCaptureQ => {
                        if self.active_color == Color::White {
                            self.remove_piece(Piece::WhiteQueen, to);
                        } else {
                            self.remove_piece(Piece::BlackQueen, to);
                        }

                        self.phase_value -= Piece::QUEEN_PHASE_VALUE;
                    }
                    _ => {}
                }
            }
        }

        if mv.is_capture()
            && flag != MoveFlag::EnPassant
            && let Some(captured_piece) = unmove.captured_piece
        {
            self.put_piece(captured_piece, to);
            self.phase_value += Piece::get_phase_value(captured_piece);
        }

        match flag {
            MoveFlag::KingCastle => {
                if self.active_color == Color::White {
                    self.remove_piece(Piece::WhiteRook, Square::F1 as u8);
                    self.put_piece(Piece::WhiteRook, Square::H1 as u8);
                } else {
                    self.remove_piece(Piece::BlackRook, Square::F8 as u8);
                    self.put_piece(Piece::BlackRook, Square::H8 as u8);
                }
            }
            MoveFlag::QueenCastle => {
                if self.active_color == Color::White {
                    self.remove_piece(Piece::WhiteRook, Square::D1 as u8);
                    self.put_piece(Piece::WhiteRook, Square::A1 as u8);
                } else {
                    self.remove_piece(Piece::BlackRook, Square::D8 as u8);
                    self.put_piece(Piece::BlackRook, Square::A8 as u8);
                }
            }
            MoveFlag::EnPassant => {
                if self.active_color == Color::White {
                    self.put_piece(Piece::BlackPawn, to - 8);
                } else {
                    self.put_piece(Piece::WhitePawn, to + 8);
                }
            }
            _ => {}
        }

        self.set_general_bitboards();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_detection() {
        let mut board = Board::new();
        board
            .parse_fen("rnbqkbnr/1ppppppp/p7/8/Q7/2P5/PP1PPPPP/RNB1KBNR b KQkq - 0 1")
            .unwrap();

        board.print_board();

        let is_attacked_test = board.is_square_attacked_by(Square::D7 as u8, Color::White, false);
        println!("{}", is_attacked_test);

        let test_mv = Move::new(Square::D7 as u8, Square::D6 as u8, MoveFlag::QuietMove);
        let (is_legal, unmove) = board.make_move(test_mv, true);
        board.print_board();
        println!("{}", is_legal);

        board.unmake_move(test_mv, unmove);
        board.print_board();
    }
}
