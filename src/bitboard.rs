pub const FILE_A: u64 = 0x101010101010101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_C: u64 = FILE_A << 2;
pub const FILE_D: u64 = FILE_A << 3;
pub const FILE_E: u64 = FILE_A << 4;
pub const FILE_F: u64 = FILE_A << 5;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;

pub const RANK_1: u64 = 0xFF;
pub const RANK_2: u64 = RANK_1 << 8;
pub const RANK_3: u64 = RANK_1 << 16;
pub const RANK_4: u64 = RANK_1 << 24;
pub const RANK_5: u64 = RANK_1 << 32;
pub const RANK_6: u64 = RANK_1 << 40;
pub const RANK_7: u64 = RANK_1 << 48;
pub const RANK_8: u64 = RANK_1 << 56;

pub const WHITE_KINGSIDE_CASTLING: u64 = 0x60;
pub const WHITE_QUEENSIDE_CASTLING: u64 = 0xE;
pub const BLACK_KINGSIDE_CASTLING: u64 = 0x6000000000000000;
pub const BLACK_QUEENSIDE_CASTLING: u64 = 0xE00000000000000;

pub const CASTLING_PERMUTATIONS: [u8; 64] = generate_castling_permutations();

const fn generate_castling_permutations() -> [u8; 64] {
    let mut castling_permutations = [0b1111; 64];

    castling_permutations[7] = 0b1110; // H1 -> WK
    castling_permutations[4] = 0b1100; // E1 -> WK & WQ
    castling_permutations[0] = 0b1101; // A1 -> WQ

    castling_permutations[63] = 0b1011; // H8 -> BK
    castling_permutations[60] = 0b0011; // E8 -> BK & BQ
    castling_permutations[56] = 0b0111; // A8 -> BQ

    castling_permutations
}

static RAY_BETWEEN: [[u64; 64]; 64] = generate_ray_table();

const fn generate_ray_table() -> [[u64; 64]; 64] {
    let mut ray_table = [[0; 64]; 64];

    let mut square1: i8 = 0;

    while square1 < 64 {
        let sq1rank = square1 / 8;
        let sq1file = square1 % 8;

        let mut square2 = 0;
        while square2 < 64 {
            if square1 == square2 {
                square2 += 1;
                continue;
            }

            if ray_table[square1 as usize][square2 as usize] != 0 {
                square2 += 1;
                continue;
            }

            let sq2rank = square2 / 8;
            let sq2file = square2 % 8;

            let rank_diff = sq2rank - sq1rank;
            let file_diff = sq2file - sq1file;

            let is_bishop_ray = rank_diff.abs() == file_diff.abs();
            let is_rook_ray = rank_diff == 0 || file_diff == 0;

            if is_bishop_ray || is_rook_ray {
                let mut ray = 0;

                let step_rank = rank_diff.signum();
                let step_file = file_diff.signum();

                let mut current_rank = sq1rank + step_rank;
                let mut current_file = sq1file + step_file;

                while (current_rank >= 0 && current_rank < 8)
                    && (current_file >= 0 && current_file < 8)
                {
                    if current_rank == sq2rank && current_file == sq2file {
                        break;
                    }
                    let current_sq = current_rank * 8 + current_file;
                    ray |= 1 << current_sq;

                    current_rank += step_rank;
                    current_file += step_file;
                }

                ray_table[square1 as usize][square2 as usize] = ray;
                ray_table[square2 as usize][square1 as usize] = ray;
            }

            square2 += 1;
        }

        square1 += 1;
    }

    ray_table
}

#[inline(always)]
pub fn get_ray_between_exclusive(square1: u8, square2: u8) -> u64 {
    RAY_BETWEEN[square1 as usize][square2 as usize]
}
