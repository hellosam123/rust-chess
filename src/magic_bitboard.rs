#![allow(long_running_const_eval)]

#[derive(Debug, Default, Clone, Copy)]
struct MagicEntry {
    mask: u64,
    magic: u64,
    shift: u8,
    offset: usize,
}

#[rustfmt::skip]
const BISHOP_MAGICS: [MagicEntry; 64] = [
MagicEntry { mask: 0x0040201008040200, magic: 0xc002423212060200, shift: 58, offset: 0 }, 
MagicEntry { mask: 0x0000402010080400, magic: 0x4010c10600820412, shift: 59, offset: 64 }, 
MagicEntry { mask: 0x0000004020100a00, magic: 0x1408008400838000, shift: 59, offset: 96 }, 
MagicEntry { mask: 0x0000000040221400, magic: 0x0004404081081002, shift: 59, offset: 128 }, 
MagicEntry { mask: 0x0000000002442800, magic: 0x0041104080000420, shift: 59, offset: 160 }, 
MagicEntry { mask: 0x0000000204085000, magic: 0x00c2882008010000, shift: 59, offset: 192 }, 
MagicEntry { mask: 0x0000020408102000, magic: 0x100c020110a98180, shift: 59, offset: 224 }, 
MagicEntry { mask: 0x0002040810204000, magic: 0x0001088084200210, shift: 58, offset: 256 }, 
MagicEntry { mask: 0x0020100804020000, magic: 0x0000200404481040, shift: 59, offset: 320 }, 
MagicEntry { mask: 0x0040201008040000, magic: 0x0211202809490020, shift: 59, offset: 352 }, 
MagicEntry { mask: 0x00004020100a0000, magic: 0x00804800c9020000, shift: 59, offset: 384 }, 
MagicEntry { mask: 0x0000004022140000, magic: 0x0200040435800004, shift: 59, offset: 416 }, 
MagicEntry { mask: 0x0000000244280000, magic: 0x0808041044122000, shift: 59, offset: 448 }, 
MagicEntry { mask: 0x0000020408500000, magic: 0x100c020110a98180, shift: 59, offset: 480 }, 
MagicEntry { mask: 0x0002040810200000, magic: 0x4002088805482080, shift: 59, offset: 512 }, 
MagicEntry { mask: 0x0004081020400000, magic: 0x000a104201900880, shift: 59, offset: 544 }, 
MagicEntry { mask: 0x0010080402000200, magic: 0x6510802820013402, shift: 59, offset: 576 }, 
MagicEntry { mask: 0x0020100804000400, magic: 0x0020040202220201, shift: 59, offset: 608 }, 
MagicEntry { mask: 0x004020100a000a00, magic: 0x04b40a8800421200, shift: 57, offset: 640 }, 
MagicEntry { mask: 0x0000402214001400, magic: 0x4844004641020000, shift: 57, offset: 768 }, 
MagicEntry { mask: 0x0000024428002800, magic: 0x0180800400a00441, shift: 57, offset: 896 }, 
MagicEntry { mask: 0x0002040850005000, magic: 0x8082000100410444, shift: 57, offset: 1024 }, 
MagicEntry { mask: 0x0004081020002000, magic: 0x42040202c1041001, shift: 59, offset: 1152 }, 
MagicEntry { mask: 0x0008102040004000, magic: 0x4601015202490c80, shift: 59, offset: 1184 }, 
MagicEntry { mask: 0x0008040200020400, magic: 0x0220300004500201, shift: 59, offset: 1216 }, 
MagicEntry { mask: 0x0010080400040800, magic: 0x08010c0008900404, shift: 59, offset: 1248 }, 
MagicEntry { mask: 0x0020100a000a1000, magic: 0x0010220114040400, shift: 57, offset: 1280 }, 
MagicEntry { mask: 0x0040221400142200, magic: 0x0004040008410200, shift: 55, offset: 1408 }, 
MagicEntry { mask: 0x0002442800284400, magic: 0x0073080409004008, shift: 55, offset: 1920 }, 
MagicEntry { mask: 0x0004085000500800, magic: 0x200082012101088c, shift: 57, offset: 2432 }, 
MagicEntry { mask: 0x0008102000201000, magic: 0x5404024085861082, shift: 59, offset: 2560 }, 
MagicEntry { mask: 0x0010204000402000, magic: 0x0882024008804800, shift: 59, offset: 2592 }, 
MagicEntry { mask: 0x0004020002040800, magic: 0x4048080c4040c401, shift: 59, offset: 2624 }, 
MagicEntry { mask: 0x0008040004081000, magic: 0x0828141002040101, shift: 59, offset: 2656 }, 
MagicEntry { mask: 0x00100a000a102000, magic: 0x2400209000080a60, shift: 57, offset: 2688 }, 
MagicEntry { mask: 0x0022140014224000, magic: 0x8400040400080211, shift: 55, offset: 2816 }, 
MagicEntry { mask: 0x0044280028440200, magic: 0x1010220020020028, shift: 55, offset: 3328 }, 
MagicEntry { mask: 0x0008500050080400, magic: 0x401202a200050044, shift: 57, offset: 3840 }, 
MagicEntry { mask: 0x0010200020100800, magic: 0x0108024041009800, shift: 59, offset: 3968 }, 
MagicEntry { mask: 0x0020400040201000, magic: 0x0092004910614402, shift: 59, offset: 4000 }, 
MagicEntry { mask: 0x0002000204081000, magic: 0x42040202c1041001, shift: 59, offset: 4032 }, 
MagicEntry { mask: 0x0004000408102000, magic: 0x140e021004800280, shift: 59, offset: 4064 }, 
MagicEntry { mask: 0x000a000a10204000, magic: 0x0081084050000800, shift: 57, offset: 4096 }, 
MagicEntry { mask: 0x0014001422400000, magic: 0x0100060212002c00, shift: 57, offset: 4224 }, 
MagicEntry { mask: 0x0028002844020000, magic: 0x0010400493040600, shift: 57, offset: 4352 }, 
MagicEntry { mask: 0x0050005008040200, magic: 0x0204100070401200, shift: 57, offset: 4480 }, 
MagicEntry { mask: 0x0020002010080400, magic: 0x4010c10600820412, shift: 59, offset: 4608 }, 
MagicEntry { mask: 0x0040004020100800, magic: 0x18100400a48000a0, shift: 59, offset: 4640 }, 
MagicEntry { mask: 0x0000020408102000, magic: 0x100c020110a98180, shift: 59, offset: 4672 }, 
MagicEntry { mask: 0x0000040810204000, magic: 0x2800b20104200000, shift: 59, offset: 4704 }, 
MagicEntry { mask: 0x00000a1020400000, magic: 0x000a104201900880, shift: 59, offset: 4736 }, 
MagicEntry { mask: 0x0000142240000000, magic: 0x0001080042021000, shift: 59, offset: 4768 }, 
MagicEntry { mask: 0x0000284402000000, magic: 0x2900003006020000, shift: 59, offset: 4800 }, 
MagicEntry { mask: 0x0000500804020000, magic: 0x00c20408100900e4, shift: 59, offset: 4832 }, 
MagicEntry { mask: 0x0000201008040200, magic: 0xa02076840820a028, shift: 59, offset: 4864 }, 
MagicEntry { mask: 0x0000402010080400, magic: 0x4010c10600820412, shift: 59, offset: 4896 }, 
MagicEntry { mask: 0x0002040810204000, magic: 0x0001088084200210, shift: 58, offset: 4928 }, 
MagicEntry { mask: 0x0004081020400000, magic: 0x000a104201900880, shift: 59, offset: 4992 }, 
MagicEntry { mask: 0x000a102040000000, magic: 0x8400000021280800, shift: 59, offset: 5024 }, 
MagicEntry { mask: 0x0014224000000000, magic: 0x0d00502801420221, shift: 59, offset: 5056 }, 
MagicEntry { mask: 0x0028440200000000, magic: 0x0400924010020a20, shift: 59, offset: 5088 }, 
MagicEntry { mask: 0x0050080402000000, magic: 0x2280001082104908, shift: 59, offset: 5120 }, 
MagicEntry { mask: 0x0020100804020000, magic: 0x0000200404481040, shift: 59, offset: 5152 }, 
MagicEntry { mask: 0x0040201008040200, magic: 0xc002423212060200, shift: 58, offset: 5184 }, 
];

const BISHOP_TABLE_SIZE: usize = 5248;

#[rustfmt::skip]
const ROOK_MAGICS: [MagicEntry; 64] = [
MagicEntry { mask: 0x000101010101017e, magic: 0x02800020c0001880, shift: 52, offset: 0 }, 
MagicEntry { mask: 0x000202020202027c, magic: 0x4040200240003000, shift: 53, offset: 4096 }, 
MagicEntry { mask: 0x000404040404047a, magic: 0x0980200188801000, shift: 53, offset: 6144 }, 
MagicEntry { mask: 0x0008080808080876, magic: 0x0080041800100280, shift: 53, offset: 8192 }, 
MagicEntry { mask: 0x001010101010106e, magic: 0x1200100805200600, shift: 53, offset: 10240 }, 
MagicEntry { mask: 0x002020202020205e, magic: 0x8600100508040200, shift: 53, offset: 12288 }, 
MagicEntry { mask: 0x004040404040403e, magic: 0x170002000081000c, shift: 53, offset: 14336 }, 
MagicEntry { mask: 0x008080808080807e, magic: 0x0200002190440601, shift: 52, offset: 16384 }, 
MagicEntry { mask: 0x0001010101017e00, magic: 0x0800800240006086, shift: 53, offset: 20480 }, 
MagicEntry { mask: 0x0002020202027c00, magic: 0x0224402000401000, shift: 54, offset: 22528 }, 
MagicEntry { mask: 0x0004040404047a00, magic: 0x1858802000900080, shift: 54, offset: 23552 }, 
MagicEntry { mask: 0x0008080808087600, magic: 0x5128801004280180, shift: 54, offset: 24576 }, 
MagicEntry { mask: 0x0010101010106e00, magic: 0x000100500800a500, shift: 54, offset: 25600 }, 
MagicEntry { mask: 0x0020202020205e00, magic: 0x0011000900228400, shift: 54, offset: 26624 }, 
MagicEntry { mask: 0x0040404040403e00, magic: 0x8004001004010802, shift: 54, offset: 27648 }, 
MagicEntry { mask: 0x0080808080807e00, magic: 0x8080800500044080, shift: 53, offset: 28672 }, 
MagicEntry { mask: 0x00010101017e0100, magic: 0x4420228000914002, shift: 53, offset: 30720 }, 
MagicEntry { mask: 0x00020202027c0200, magic: 0x0010084000e00040, shift: 54, offset: 32768 }, 
MagicEntry { mask: 0x00040404047a0400, magic: 0xc1010500200040b4, shift: 54, offset: 33792 }, 
MagicEntry { mask: 0x0008080808760800, magic: 0x4120808010010800, shift: 54, offset: 34816 }, 
MagicEntry { mask: 0x00101010106e1000, magic: 0x00080180288c0080, shift: 54, offset: 35840 }, 
MagicEntry { mask: 0x00202020205e2000, magic: 0x4001010008620400, shift: 54, offset: 36864 }, 
MagicEntry { mask: 0x00404040403e4000, magic: 0x0080540022080130, shift: 54, offset: 37888 }, 
MagicEntry { mask: 0x00808080807e8000, magic: 0x0015060004008041, shift: 53, offset: 38912 }, 
MagicEntry { mask: 0x000101017e010100, magic: 0x020020808004c009, shift: 53, offset: 40960 }, 
MagicEntry { mask: 0x000202027c020200, magic: 0x2a00400080200080, shift: 54, offset: 43008 }, 
MagicEntry { mask: 0x000404047a040400, magic: 0x4060001900210040, shift: 54, offset: 44032 }, 
MagicEntry { mask: 0x0008080876080800, magic: 0x0001100480080080, shift: 54, offset: 45056 }, 
MagicEntry { mask: 0x001010106e101000, magic: 0x8218020040040040, shift: 54, offset: 46080 }, 
MagicEntry { mask: 0x002020205e202000, magic: 0x0011000900228400, shift: 54, offset: 47104 }, 
MagicEntry { mask: 0x004040403e404000, magic: 0x1000100400830a08, shift: 54, offset: 48128 }, 
MagicEntry { mask: 0x008080807e808000, magic: 0x003028820021004c, shift: 53, offset: 49152 }, 
MagicEntry { mask: 0x0001017e01010100, magic: 0x000122c004800084, shift: 53, offset: 51200 }, 
MagicEntry { mask: 0x0002027c02020200, magic: 0x1210002000404004, shift: 54, offset: 53248 }, 
MagicEntry { mask: 0x0004047a04040400, magic: 0x0084200145001100, shift: 54, offset: 54272 }, 
MagicEntry { mask: 0x0008087608080800, magic: 0x10100400c0400800, shift: 54, offset: 55296 }, 
MagicEntry { mask: 0x0010106e10101000, magic: 0x4200041101000800, shift: 54, offset: 56320 }, 
MagicEntry { mask: 0x0020205e20202000, magic: 0x0106020080800400, shift: 54, offset: 57344 }, 
MagicEntry { mask: 0x0040403e40404000, magic: 0x0221023024000198, shift: 54, offset: 58368 }, 
MagicEntry { mask: 0x0080807e80808000, magic: 0x0000304386001904, shift: 53, offset: 59392 }, 
MagicEntry { mask: 0x00017e0101010100, magic: 0x1020902040008001, shift: 53, offset: 61440 }, 
MagicEntry { mask: 0x00027c0202020200, magic: 0x1210002000404004, shift: 54, offset: 63488 }, 
MagicEntry { mask: 0x00047a0404040400, magic: 0x0006402001010052, shift: 54, offset: 64512 }, 
MagicEntry { mask: 0x0008760808080800, magic: 0x0006020820120040, shift: 54, offset: 65536 }, 
MagicEntry { mask: 0x00106e1010101000, magic: 0x1022040008008080, shift: 54, offset: 66560 }, 
MagicEntry { mask: 0x00205e2020202000, magic: 0x0442000450420008, shift: 54, offset: 67584 }, 
MagicEntry { mask: 0x00403e4040404000, magic: 0x10011201080c0030, shift: 54, offset: 68608 }, 
MagicEntry { mask: 0x00807e8080808000, magic: 0x04180484014a0001, shift: 53, offset: 69632 }, 
MagicEntry { mask: 0x007e010101010100, magic: 0x8801104180022100, shift: 53, offset: 71680 }, 
MagicEntry { mask: 0x007c020202020200, magic: 0x0a04400023038100, shift: 54, offset: 73728 }, 
MagicEntry { mask: 0x007a040404040400, magic: 0x010010c060820200, shift: 54, offset: 74752 }, 
MagicEntry { mask: 0x0076080808080800, magic: 0x5128801004280180, shift: 54, offset: 75776 }, 
MagicEntry { mask: 0x006e101010101000, magic: 0x000100500800a500, shift: 54, offset: 76800 }, 
MagicEntry { mask: 0x005e202020202000, magic: 0x10148c00803a0080, shift: 54, offset: 77824 }, 
MagicEntry { mask: 0x003e404040404000, magic: 0x8800081a1001b400, shift: 54, offset: 78848 }, 
MagicEntry { mask: 0x007e808080808000, magic: 0x8080800500044080, shift: 53, offset: 79872 }, 
MagicEntry { mask: 0x7e01010101010100, magic: 0x0421081080024021, shift: 52, offset: 81920 }, 
MagicEntry { mask: 0x7c02020202020200, magic: 0x2101001280400021, shift: 53, offset: 86016 }, 
MagicEntry { mask: 0x7a04040404040400, magic: 0x0423014014200009, shift: 53, offset: 88064 }, 
MagicEntry { mask: 0x7608080808080800, magic: 0x8120200d10000901, shift: 53, offset: 90112 }, 
MagicEntry { mask: 0x6e10101010101000, magic: 0x8021002800020411, shift: 53, offset: 92160 }, 
MagicEntry { mask: 0x5e20202020202000, magic: 0x1c05000204000821, shift: 53, offset: 94208 }, 
MagicEntry { mask: 0x3e40404040404000, magic: 0x110002300800a104, shift: 53, offset: 96256 }, 
MagicEntry { mask: 0x7e80808080808000, magic: 0x000421a504004082, shift: 52, offset: 98304 }, 
];

const ROOK_TABLE_SIZE: usize = 102400;

static BISHOP_ATTACK_TABLE: [u64; BISHOP_TABLE_SIZE] = generate_bishop_table();
static ROOK_ATTACK_TABLE: [u64; ROOK_TABLE_SIZE] = generate_rook_table();

const fn generate_bishop_table() -> [u64; BISHOP_TABLE_SIZE] {
    let mut bishop_table = [0; BISHOP_TABLE_SIZE];

    let mut square = 0;
    while square < 64 {
        let entry = BISHOP_MAGICS[square as usize];
        let blocker_bits = 64 - entry.shift;
        let num_permutations = 1 << blocker_bits;

        let mut permutation_index = 0;
        while permutation_index < num_permutations {
            let blockers = create_blocker_permutation(entry.mask, permutation_index);
            let attacks_mask = generate_bishop_attacks_mask(square, blockers);
            let hash_index = blockers.wrapping_mul(entry.magic) >> entry.shift;
            let table_index = hash_index as usize + entry.offset;

            bishop_table[table_index] = attacks_mask;

            permutation_index += 1;
        }

        square += 1;
    }

    bishop_table
}

const fn generate_rook_table() -> [u64; ROOK_TABLE_SIZE] {
    let mut rook_table = [0; ROOK_TABLE_SIZE];

    let mut square = 0;
    while square < 64 {
        let entry = ROOK_MAGICS[square as usize];
        let blocker_bits = 64 - entry.shift;
        let num_permutations = 1 << blocker_bits;

        let mut permutation_index = 0;
        while permutation_index < num_permutations {
            let blockers = create_blocker_permutation(entry.mask, permutation_index);
            let attacks_mask = generate_rook_attacks_mask(square, blockers);
            let hash_index = blockers.wrapping_mul(entry.magic) >> entry.shift;
            let table_index = hash_index as usize + entry.offset;

            rook_table[table_index] = attacks_mask;

            permutation_index += 1;
        }

        square += 1;
    }

    rook_table
}

const fn generate_bishop_attacks_mask(square: u8, blockers: u64) -> u64 {
    let mut bishop_attacks = 0;
    let bishop_directions = [9, -7, -9, 7];

    let mut i = 0;
    while i < 4 {
        let step = bishop_directions[i];
        let mut current_square = square as i32;

        loop {
            if step == 9 && (current_square >= 56 || current_square % 8 >= 7) {
                break;
            }
            if step == -7 && (current_square <= 7 || current_square % 8 >= 7) {
                break;
            }
            if step == -9 && (current_square <= 7 || current_square % 8 <= 0) {
                break;
            }
            if step == 7 && (current_square >= 56 || current_square % 8 <= 0) {
                break;
            }

            current_square += step;

            bishop_attacks |= 1 << current_square;
            if 1 << current_square & blockers != 0 {
                break;
            }
        }

        i += 1;
    }

    bishop_attacks
}

const fn generate_rook_attacks_mask(square: u8, blockers: u64) -> u64 {
    let mut rook_attacks = 0;
    let rook_directions = [8, 1, -8, -1];

    let mut i = 0;
    while i < 4 {
        let step = rook_directions[i];
        let mut current_square = square as i32;

        loop {
            if step == 8 && current_square >= 56 {
                break;
            }
            if step == 1 && current_square % 8 >= 7 {
                break;
            }
            if step == -8 && current_square <= 7 {
                break;
            }
            if step == -1 && current_square % 8 <= 0 {
                break;
            }

            current_square += step;

            rook_attacks |= 1 << current_square;
            if 1 << current_square & blockers != 0 {
                break;
            }
        }

        i += 1;
    }

    rook_attacks
}

const fn create_blocker_permutation(mask: u64, index: usize) -> u64 {
    let mut blockers = 0;
    let mut temp_mask = mask;
    let mut i = 0;

    while temp_mask != 0 {
        let square = temp_mask.trailing_zeros() as u8;
        temp_mask &= temp_mask - 1;

        if (index & 1 << i) != 0 {
            blockers |= 1 << square;
        }

        i += 1;
    }

    blockers
}

#[inline(always)]
pub fn get_bishop_attacks_mask(occupancy: u64, from: u8) -> u64 {
    let entry = BISHOP_MAGICS[from as usize];
    let blockers = entry.mask & occupancy;
    let hash_index = blockers.wrapping_mul(entry.magic) >> entry.shift;
    let table_index = hash_index as usize + entry.offset;

    BISHOP_ATTACK_TABLE[table_index]
}

#[inline(always)]
pub fn get_rook_attacks_mask(occupancy: u64, from: u8) -> u64 {
    let entry = ROOK_MAGICS[from as usize];
    let blockers = entry.mask & occupancy;
    let hash_index = blockers.wrapping_mul(entry.magic) >> entry.shift;
    let table_index = hash_index as usize + entry.offset;

    ROOK_ATTACK_TABLE[table_index]
}

#[inline(always)]
pub fn get_queen_attacks_mask(occupancy: u64, from: u8) -> u64 {
    get_bishop_attacks_mask(occupancy, from) | get_rook_attacks_mask(occupancy, from)
}
