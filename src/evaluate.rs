use crate::{
    board::{Board, Color, Piece},
    constants::MAX_PHASE,
    params::{
        BISHOP_MAT, BISHOP_PSQT, KING_PSQT, KNIGHT_MAT, KNIGHT_PSQT, PAWN_MAT, PAWN_PSQT,
        QUEEN_MAT, QUEEN_PSQT, ROOK_MAT, ROOK_PSQT,
    },
};

pub struct Evaluate;

fn flip_square(square: u8) -> u8 {
    square ^ 56
}

impl Evaluate {
    #[inline(always)]
    pub fn sided_eval(board: &Board) -> i32 {
        let score = Self::eval(board);
        if board.active_color == Color::White {
            score
        } else {
            -score
        }
    }

    pub fn eval(board: &Board) -> i32 {
        let mut mg_score = 0;
        let mut eg_score = 0;

        let (mg_material, eg_material) = Self::get_material_scores(board);
        mg_score += mg_material;
        eg_score += eg_material;

        let (mg_psqt, eg_psqt) = Self::get_psqt_scores(board);
        mg_score += mg_psqt;
        eg_score += eg_psqt;

        Self::get_tapered_score(mg_score, eg_score, board.phase_value)
    }

    fn get_material_scores(board: &Board) -> (i32, i32) {
        let mut mg_material = 0;
        let mut eg_material = 0;
        let num_pawns = board.pieces[Piece::WhitePawn as usize].count_ones() as i32
            - board.pieces[Piece::BlackPawn as usize].count_ones() as i32;
        let num_knights = board.pieces[Piece::WhiteKnight as usize].count_ones() as i32
            - board.pieces[Piece::BlackKnight as usize].count_ones() as i32;
        let num_bishops = board.pieces[Piece::WhiteBishop as usize].count_ones() as i32
            - board.pieces[Piece::BlackBishop as usize].count_ones() as i32;
        let num_rooks = board.pieces[Piece::WhiteRook as usize].count_ones() as i32
            - board.pieces[Piece::BlackRook as usize].count_ones() as i32;
        let num_queens = board.pieces[Piece::WhiteQueen as usize].count_ones() as i32
            - board.pieces[Piece::BlackQueen as usize].count_ones() as i32;

        mg_material += PAWN_MAT.mg * num_pawns;
        mg_material += KNIGHT_MAT.mg * num_knights;
        mg_material += BISHOP_MAT.mg * num_bishops;
        mg_material += ROOK_MAT.mg * num_rooks;
        mg_material += QUEEN_MAT.mg * num_queens;

        eg_material += PAWN_MAT.eg * num_pawns;
        eg_material += KNIGHT_MAT.eg * num_knights;
        eg_material += BISHOP_MAT.eg * num_bishops;
        eg_material += ROOK_MAT.eg * num_rooks;
        eg_material += QUEEN_MAT.eg * num_queens;

        (mg_material, eg_material)
    }

    fn get_psqt_scores(board: &Board) -> (i32, i32) {
        let mut mg_psqt = 0;
        let mut eg_psqt = 0;

        let mut all_pieces_mask = board.all_pieces;
        while all_pieces_mask != 0 {
            let square = all_pieces_mask.trailing_zeros() as u8;
            all_pieces_mask &= all_pieces_mask - 1;

            if let Some(piece) = board.get_piece(square) {
                let index = if piece.get_color() == Color::White {
                    square as usize
                } else {
                    flip_square(square) as usize
                };
                let psqt_score = match piece {
                    Piece::WhitePawn | Piece::BlackPawn => PAWN_PSQT[index],

                    Piece::WhiteKnight | Piece::BlackKnight => KNIGHT_PSQT[index],

                    Piece::WhiteBishop | Piece::BlackBishop => BISHOP_PSQT[index],

                    Piece::WhiteRook | Piece::BlackRook => ROOK_PSQT[index],

                    Piece::WhiteQueen | Piece::BlackQueen => QUEEN_PSQT[index],

                    Piece::WhiteKing | Piece::BlackKing => KING_PSQT[index],
                };

                if piece.get_color() == Color::White {
                    mg_psqt += psqt_score.mg;
                    eg_psqt += psqt_score.eg;
                } else {
                    mg_psqt -= psqt_score.mg;
                    eg_psqt -= psqt_score.eg;
                }
            }
        }

        (mg_psqt, eg_psqt)
    }

    #[inline(always)]
    fn get_tapered_score(mg_score: i32, eg_score: i32, mut phase_value: u8) -> i32 {
        if phase_value > MAX_PHASE {
            phase_value = MAX_PHASE;
        }

        let eg_phase = MAX_PHASE - phase_value;
        (mg_score * phase_value as i32 + eg_score * eg_phase as i32) / MAX_PHASE as i32
    }
}
