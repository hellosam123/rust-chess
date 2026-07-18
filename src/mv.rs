use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(u16);

#[derive(Debug, PartialEq, Eq)]
pub enum MoveFlag {
    QuietMove = 0b0000,
    DoublePawnPush = 0b0001,
    KingCastle = 0b0010,
    QueenCastle = 0b0011,
    Capture = 0b0100,
    EnPassant = 0b0101,
    PromoteN = 0b1000,
    PromoteB = 0b1001,
    PromoteR = 0b1010,
    PromoteQ = 0b1011,
    PromoteCaptureN = 0b1100,
    PromoteCaptureB = 0b1101,
    PromoteCaptureR = 0b1110,
    PromoteCaptureQ = 0b1111,
}

fn square_to_str(square: u8) -> String {
    let rank = square / 8;
    let file = square % 8;

    let rank_char = (b'1' + rank) as char;
    let file_char = (b'a' + file) as char;

    format!("{}{}", file_char, rank_char)
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from_str = square_to_str(self.get_from());
        let to_str = square_to_str(self.get_to());
        let mut promotion_str = String::with_capacity(1);

        if self.is_promotion() {
            match self.get_flag() {
                MoveFlag::PromoteN | MoveFlag::PromoteCaptureN => promotion_str.push('n'),
                MoveFlag::PromoteB | MoveFlag::PromoteCaptureB => promotion_str.push('b'),
                MoveFlag::PromoteR | MoveFlag::PromoteCaptureR => promotion_str.push('r'),
                MoveFlag::PromoteQ | MoveFlag::PromoteCaptureQ => promotion_str.push('q'),

                _ => {}
            }
        }

        write!(f, "{}{}{}", from_str, to_str, promotion_str)
    }
}

impl Move {
    pub const NULL: Self = Self(0);

    const FROM_MASK: u16 = 0x3F; // 0000 0000 0011 1111 (bits 0-5)
    const TO_MASK: u16 = 0xFC0; // 0000 1111 1100 0000 (bits 6-11)
    const FLAG_MASK: u16 = 0xF000; // 1111 0000 0000 0000 (bits 12-15)

    #[inline(always)]
    pub fn new(from: u8, to: u8, flag: MoveFlag) -> Self {
        Move(from as u16 | (to as u16) << 6 | (flag as u16) << 12)
    }

    #[inline(always)]
    pub fn get_from(&self) -> u8 {
        (self.0 & Self::FROM_MASK) as u8
    }

    #[inline(always)]
    pub fn get_to(&self) -> u8 {
        ((self.0 & Self::TO_MASK) >> 6) as u8
    }

    #[inline(always)]
    pub fn get_flag(&self) -> MoveFlag {
        let flag_index = ((self.0 & Self::FLAG_MASK) >> 12) as usize;
        match flag_index {
            0b0000 => MoveFlag::QuietMove,
            0b0001 => MoveFlag::DoublePawnPush,
            0b0010 => MoveFlag::KingCastle,
            0b0011 => MoveFlag::QueenCastle,
            0b0100 => MoveFlag::Capture,
            0b0101 => MoveFlag::EnPassant,
            0b1000 => MoveFlag::PromoteN,
            0b1001 => MoveFlag::PromoteB,
            0b1010 => MoveFlag::PromoteR,
            0b1011 => MoveFlag::PromoteQ,
            0b1100 => MoveFlag::PromoteCaptureN,
            0b1101 => MoveFlag::PromoteCaptureB,
            0b1110 => MoveFlag::PromoteCaptureR,
            0b1111 => MoveFlag::PromoteCaptureQ,

            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        self.0 & 4 << 12 != 0
    }

    #[inline(always)]
    pub fn is_en_passant(&self) -> bool {
        self.get_flag() == MoveFlag::EnPassant
    }

    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        self.0 & 8 << 12 != 0
    }

    #[inline(always)]
    pub fn is_present(&self) -> bool {
        !self.is_null()
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0 == Self::NULL.0
    }
}
