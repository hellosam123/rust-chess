pub struct Move(u16);

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

impl TryFrom<usize> for MoveFlag {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let move_flag = match value {
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

            _ => return Err("index out of bounds for MoveFlag enum"),
        };

        Ok(move_flag)
    }
}

impl Move {
    const FROM_MASK: u16 = 0x3F; // 0000 0000 0011 1111 (bits 0-5)
    const TO_MASK: u16 = 0xFC0; // 0000 1111 1100 0000 (bits 6-11)
    const FLAG_MASK: u16 = 0xF000; // 1111 0000 0000 0000 (bits 12-15)

    pub fn new(from: u8, to: u8, flag: MoveFlag) -> Self {
        Move((from | to << 6 | (flag as u8) << 12) as u16)
    }

    pub fn get_from(&self) -> u8 {
        (self.0 & Self::FROM_MASK) as u8
    }

    pub fn get_to(&self) -> u8 {
        ((self.0 & Self::TO_MASK) >> 6) as u8
    }

    pub fn get_flag(&self) -> Result<MoveFlag, &'static str> {
        let flag_index = ((self.0 & Self::FLAG_MASK) >> 12) as usize;
        let move_flag = MoveFlag::try_from(flag_index)?;
        Ok(move_flag)
    }

    pub fn is_capture(&self) -> bool {
        self.0 & 4 << 12 != 0
    }

    pub fn is_promotion(&self) -> bool {
        self.0 & 8 << 12 != 0
    }
}
