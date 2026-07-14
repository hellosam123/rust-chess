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

static RAY_BETWEEN_EXCLUSIVE: [[u64; 64]; 64] = generate_ray_table();
static RAY_BETWEEN_END_INCLUSIVE: [[u64; 64]; 64] = generate_ray_end_inclusive_table();

const fn generate_ray_table() -> [[u64; 64]; 64] {
    let mut ray_table = [[0; 64]; 64];

    let mut start_square: i8 = 0;

    while start_square < 64 {
        let start_square_rank = start_square / 8;
        let start_square_file = start_square % 8;

        let mut end_square = 0;
        while end_square < 64 {
            if start_square == end_square {
                end_square += 1;
                continue;
            }

            if ray_table[start_square as usize][end_square as usize] != 0 {
                end_square += 1;
                continue;
            }

            let end_square_rank = end_square / 8;
            let end_square_file = end_square % 8;

            let rank_diff = end_square_rank - start_square_rank;
            let file_diff = end_square_file - start_square_file;

            let is_bishop_ray = rank_diff.abs() == file_diff.abs();
            let is_rook_ray = rank_diff == 0 || file_diff == 0;

            if is_bishop_ray || is_rook_ray {
                let mut ray = 0;

                let step_rank = rank_diff.signum();
                let step_file = file_diff.signum();

                let mut current_rank = start_square_rank + step_rank;
                let mut current_file = start_square_file + step_file;

                while (current_rank >= 0 && current_rank < 8)
                    && (current_file >= 0 && current_file < 8)
                {
                    if current_rank == end_square_rank && current_file == end_square_file {
                        break;
                    }
                    let current_sq = current_rank * 8 + current_file;
                    ray |= 1 << current_sq;

                    current_rank += step_rank;
                    current_file += step_file;
                }

                ray_table[start_square as usize][end_square as usize] = ray;
                ray_table[end_square as usize][start_square as usize] = ray;
            }

            end_square += 1;
        }

        start_square += 1;
    }

    ray_table
}

const fn generate_ray_end_inclusive_table() -> [[u64; 64]; 64] {
    let mut ray_table = [[0; 64]; 64];

    let mut start_square: i8 = 0;

    while start_square < 64 {
        let start_square_rank = start_square / 8;
        let start_square_file = start_square % 8;

        let mut end_square = 0;
        while end_square < 64 {
            if start_square == end_square {
                end_square += 1;
                continue;
            }

            let end_square_rank = end_square / 8;
            let end_square_file = end_square % 8;

            let rank_diff = end_square_rank - start_square_rank;
            let file_diff = end_square_file - start_square_file;

            let is_bishop_ray = rank_diff.abs() == file_diff.abs();
            let is_rook_ray = rank_diff == 0 || file_diff == 0;

            if is_bishop_ray || is_rook_ray {
                let mut ray = 0;

                let step_rank = rank_diff.signum();
                let step_file = file_diff.signum();

                let mut current_rank = start_square_rank + step_rank;
                let mut current_file = start_square_file + step_file;

                while (current_rank >= 0 && current_rank < 8)
                    && (current_file >= 0 && current_file < 8)
                {
                    let current_sq = current_rank * 8 + current_file;
                    ray |= 1 << current_sq;

                    if current_rank == end_square_rank && current_file == end_square_file {
                        break;
                    }

                    current_rank += step_rank;
                    current_file += step_file;
                }

                ray_table[start_square as usize][end_square as usize] = ray;
            }

            end_square += 1;
        }

        start_square += 1;
    }

    ray_table
}

#[inline(always)]
pub fn get_ray_between_exclusive(start_square: u8, end_square: u8) -> u64 {
    RAY_BETWEEN_EXCLUSIVE[start_square as usize][end_square as usize]
}

#[inline(always)]
pub fn get_ray_between_end_inclusive(start_square: u8, end_square: u8) -> u64 {
    RAY_BETWEEN_END_INCLUSIVE[start_square as usize][end_square as usize]
}
