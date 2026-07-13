pub const WHITE_PAWN_ATTACKS: [u64; 64] = generate_white_pawn_attacks();
pub const BLACK_PAWN_ATTACKS: [u64; 64] = generate_black_pawn_attacks();
pub const KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();
pub const KING_ATTACKS: [u64; 64] = generate_king_attacks();

const fn generate_white_pawn_attacks() -> [u64; 64] {
    let mut white_pawn_attacks = [0; 64];

    let mut square = 0;
    while square < 64 {
        let rank = square / 8;
        let file = square % 8;

        let mut attacks = 0;

        // not last rank
        if rank < 7 {
            // not A file
            if file > 0 {
                attacks |= 1 << (square + 7);
            }

            // not H file
            if file < 7 {
                attacks |= 1 << (square + 9);
            }
        }

        white_pawn_attacks[square] = attacks;
        square += 1;
    }

    white_pawn_attacks
}

const fn generate_black_pawn_attacks() -> [u64; 64] {
    let mut black_pawn_attacks = [0; 64];

    let mut square = 0;
    while square < 64 {
        let rank = square / 8;
        let file = square % 8;

        let mut attacks = 0;

        // not first rank
        if rank > 0 {
            // not A file
            if file > 0 {
                attacks |= 1 << (square - 9);
            }

            // not H file
            if file < 7 {
                attacks |= 1 << (square - 7);
            }
        }

        black_pawn_attacks[square] = attacks;
        square += 1;
    }

    black_pawn_attacks
}
const fn generate_knight_attacks() -> [u64; 64] {
    let rank_offsets = [2, 1, -1, -2, -2, -1, 1, 2];
    let file_offsets = [1, 2, 2, 1, -1, -2, -2, -1];

    let mut knight_attacks = [0; 64];

    let mut square = 0;
    while square < 64 {
        let rank = square / 8;
        let file = square % 8;

        let mut attacks = 0;
        let mut i = 0;
        while i < 8 {
            let attack_rank = rank + rank_offsets[i];
            let attack_file = file + file_offsets[i];

            i += 1;
            if attack_rank < 0 || attack_rank >= 8 || attack_file < 0 || attack_file >= 8 {
                continue;
            }

            let attack_square = attack_rank * 8 + attack_file;
            attacks |= 1 << attack_square;
        }
        knight_attacks[square as usize] = attacks;
        square += 1;
    }
    knight_attacks
}

const fn generate_king_attacks() -> [u64; 64] {
    let rank_offsets = [1, 1, 0, -1, -1, -1, 0, 1];
    let file_offsets = [0, 1, 1, 1, 0, -1, -1, -1];

    let mut king_attacks = [0; 64];

    let mut square = 0;
    while square < 64 {
        let rank = square / 8;
        let file = square % 8;

        let mut attacks = 0;
        let mut i = 0;
        while i < 8 {
            let attack_rank = rank + rank_offsets[i];
            let attack_file = file + file_offsets[i];

            i += 1;
            if attack_rank < 0 || attack_rank >= 8 || attack_file < 0 || attack_file >= 8 {
                continue;
            }

            let attack_square = attack_rank * 8 + attack_file;
            attacks |= 1 << attack_square;
        }
        king_attacks[square as usize] = attacks;
        square += 1;
    }
    king_attacks
}
