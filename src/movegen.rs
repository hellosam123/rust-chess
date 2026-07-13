use crate::{
    attacks, bitboard,
    board::{Board, CastlingRights, Color, Piece, Square},
    magic_bitboard,
    mv::{Move, MoveFlag},
};

pub struct MoveGenerator;

impl MoveGenerator {
    pub fn generate_pseudo_legal_moves(board: &Board) -> Vec<Move> {
        let mut move_list: Vec<Move> = Vec::with_capacity(48);
        Self::generate_pawn_moves(board, &mut move_list);
        Self::generate_knight_moves(board, &mut move_list);
        Self::generate_bishop_moves(board, &mut move_list);
        Self::generate_rook_moves(board, &mut move_list);
        Self::generate_queen_moves(board, &mut move_list);
        Self::generate_king_moves(board, &mut move_list);

        move_list
    }

    pub fn generate_pawn_moves(board: &Board, move_list: &mut Vec<Move>) {
        if board.active_color == Color::White {
            Self::generate_white_pawn_moves(board, move_list);
        } else {
            Self::generate_black_pawn_moves(board, move_list);
        }
    }

    fn generate_white_pawn_moves(board: &Board, move_list: &mut Vec<Move>) {
        let pawns_mask = board.pieces[Piece::WhitePawn as usize];
        let them_mask = board.black_pieces;
        let empty_mask = !board.all_pieces;

        let mut push_mask = pawns_mask << 8 & empty_mask;
        let mut double_push_mask = push_mask << 8 & empty_mask & bitboard::RANK_4;
        let mut capture_left_mask = ((pawns_mask & !bitboard::FILE_A) << 7) & them_mask;
        let mut capture_right_mask = ((pawns_mask & !bitboard::FILE_H) << 9) & them_mask;

        while push_mask != 0 {
            let to = push_mask.trailing_zeros() as u8;
            push_mask &= push_mask - 1;

            let from = to - 8;

            if 1 << to & bitboard::RANK_8 != 0 {
                Self::append_promotions(from, to, move_list);
            } else {
                move_list.push(Move::new(from, to, MoveFlag::QuietMove));
            }
        }

        while double_push_mask != 0 {
            let to = double_push_mask.trailing_zeros() as u8;
            double_push_mask &= double_push_mask - 1;

            let from = to - 16;

            move_list.push(Move::new(from, to, MoveFlag::DoublePawnPush));
        }

        while capture_left_mask != 0 {
            let to = capture_left_mask.trailing_zeros() as u8;
            capture_left_mask &= capture_left_mask - 1;

            let from = to - 7;

            if 1 << to & bitboard::RANK_8 != 0 {
                Self::append_capture_promotions(from, to, move_list);
            } else {
                move_list.push(Move::new(from, to, MoveFlag::Capture));
            }
        }

        while capture_right_mask != 0 {
            let to = capture_right_mask.trailing_zeros() as u8;
            capture_right_mask &= capture_right_mask - 1;

            let from = to - 9;

            if 1 << to & bitboard::RANK_8 != 0 {
                Self::append_capture_promotions(from, to, move_list);
            } else {
                move_list.push(Move::new(from, to, MoveFlag::Capture));
            }
        }

        if let Some(en_passant_square) = board.en_passant_square {
            let mut can_en_passant_mask =
                attacks::BLACK_PAWN_ATTACKS[en_passant_square as usize] & pawns_mask;

            while can_en_passant_mask != 0 {
                let from = can_en_passant_mask.trailing_zeros() as u8;
                can_en_passant_mask &= can_en_passant_mask - 1;

                move_list.push(Move::new(from, en_passant_square, MoveFlag::EnPassant))
            }
        }
    }

    fn generate_black_pawn_moves(board: &Board, move_list: &mut Vec<Move>) {
        let pawns_mask = board.pieces[Piece::BlackPawn as usize];
        let them_mask = board.white_pieces;
        let empty_mask = !board.all_pieces;

        let mut push_mask = pawns_mask >> 8 & empty_mask;
        let mut double_push_mask = push_mask >> 8 & empty_mask & bitboard::RANK_5;
        let mut capture_left_mask = ((pawns_mask & !bitboard::FILE_A) >> 9) & them_mask;
        let mut capture_right_mask = ((pawns_mask & !bitboard::FILE_H) >> 7) & them_mask;

        while push_mask != 0 {
            let to = push_mask.trailing_zeros() as u8;
            push_mask &= push_mask - 1;

            let from = to + 8;

            if 1 << to & bitboard::RANK_1 != 0 {
                Self::append_promotions(from, to, move_list);
            } else {
                move_list.push(Move::new(from, to, MoveFlag::QuietMove));
            }
        }

        while double_push_mask != 0 {
            let to = double_push_mask.trailing_zeros() as u8;
            double_push_mask &= double_push_mask - 1;

            let from = to + 16;

            move_list.push(Move::new(from, to, MoveFlag::DoublePawnPush));
        }

        while capture_left_mask != 0 {
            let to = capture_left_mask.trailing_zeros() as u8;
            capture_left_mask &= capture_left_mask - 1;

            let from = to + 9;

            if 1 << to & bitboard::RANK_1 != 0 {
                Self::append_capture_promotions(from, to, move_list);
            } else {
                move_list.push(Move::new(from, to, MoveFlag::Capture));
            }
        }

        while capture_right_mask != 0 {
            let to = capture_right_mask.trailing_zeros() as u8;
            capture_right_mask &= capture_right_mask - 1;

            let from = to + 7;

            if 1 << to & bitboard::RANK_1 != 0 {
                Self::append_capture_promotions(from, to, move_list);
            } else {
                move_list.push(Move::new(from, to, MoveFlag::Capture));
            }
        }

        if let Some(en_passant_square) = board.en_passant_square {
            let mut can_en_passant_mask =
                attacks::WHITE_PAWN_ATTACKS[en_passant_square as usize] & pawns_mask;

            while can_en_passant_mask != 0 {
                let from = can_en_passant_mask.trailing_zeros() as u8;
                can_en_passant_mask &= can_en_passant_mask - 1;

                move_list.push(Move::new(from, en_passant_square, MoveFlag::EnPassant))
            }
        }
    }

    pub fn generate_knight_moves(board: &Board, move_list: &mut Vec<Move>) {
        let mut knights_mask: u64;
        let us_mask: u64;
        let them_mask: u64;

        if board.active_color == Color::White {
            knights_mask = board.pieces[Piece::WhiteKnight as usize];
            us_mask = board.white_pieces;
            them_mask = board.black_pieces;
        } else {
            knights_mask = board.pieces[Piece::BlackKnight as usize];
            us_mask = board.black_pieces;
            them_mask = board.white_pieces;
        }

        while knights_mask != 0 {
            let from = knights_mask.trailing_zeros() as u8;
            knights_mask &= knights_mask - 1;

            let mut attacks_mask = attacks::KNIGHT_ATTACKS[from as usize];
            attacks_mask &= !us_mask;

            while attacks_mask != 0 {
                let to = attacks_mask.trailing_zeros() as u8;
                attacks_mask &= attacks_mask - 1;

                let flag = if 1 << to & them_mask != 0 {
                    MoveFlag::Capture
                } else {
                    MoveFlag::QuietMove
                };

                move_list.push(Move::new(from, to, flag));
            }
        }
    }

    fn generate_bishop_moves(board: &Board, move_list: &mut Vec<Move>) {
        let mut bishops_mask: u64;
        let us_mask: u64;
        let them_mask: u64;

        if board.active_color == Color::White {
            bishops_mask = board.pieces[Piece::WhiteBishop as usize];
            us_mask = board.white_pieces;
            them_mask = board.black_pieces;
        } else {
            bishops_mask = board.pieces[Piece::BlackBishop as usize];
            us_mask = board.black_pieces;
            them_mask = board.white_pieces;
        }

        while bishops_mask != 0 {
            let from = bishops_mask.trailing_zeros() as u8;
            bishops_mask &= bishops_mask - 1;

            let mut attacks_mask = magic_bitboard::get_bishop_attacks_mask(board.all_pieces, from);
            attacks_mask &= !us_mask;

            while attacks_mask != 0 {
                let to = attacks_mask.trailing_zeros() as u8;
                attacks_mask &= attacks_mask - 1;

                let flag = if 1 << to & them_mask != 0 {
                    MoveFlag::Capture
                } else {
                    MoveFlag::QuietMove
                };

                move_list.push(Move::new(from, to, flag));
            }
        }
    }

    fn generate_rook_moves(board: &Board, move_list: &mut Vec<Move>) {
        let mut rooks_mask: u64;
        let us_mask: u64;
        let them_mask: u64;

        if board.active_color == Color::White {
            rooks_mask = board.pieces[Piece::WhiteRook as usize];
            us_mask = board.white_pieces;
            them_mask = board.black_pieces;
        } else {
            rooks_mask = board.pieces[Piece::BlackRook as usize];
            us_mask = board.black_pieces;
            them_mask = board.white_pieces;
        }

        while rooks_mask != 0 {
            let from = rooks_mask.trailing_zeros() as u8;
            rooks_mask &= rooks_mask - 1;

            let mut attacks_mask = magic_bitboard::get_rook_attacks_mask(board.all_pieces, from);
            attacks_mask &= !us_mask;

            while attacks_mask != 0 {
                let to = attacks_mask.trailing_zeros() as u8;
                attacks_mask &= attacks_mask - 1;

                let flag = if 1 << to & them_mask != 0 {
                    MoveFlag::Capture
                } else {
                    MoveFlag::QuietMove
                };

                move_list.push(Move::new(from, to, flag));
            }
        }
    }

    fn generate_queen_moves(board: &Board, move_list: &mut Vec<Move>) {
        let mut queens_mask: u64;
        let us_mask: u64;
        let them_mask: u64;

        if board.active_color == Color::White {
            queens_mask = board.pieces[Piece::WhiteQueen as usize];
            us_mask = board.white_pieces;
            them_mask = board.black_pieces;
        } else {
            queens_mask = board.pieces[Piece::BlackQueen as usize];
            us_mask = board.black_pieces;
            them_mask = board.white_pieces;
        }

        while queens_mask != 0 {
            let from = queens_mask.trailing_zeros() as u8;
            queens_mask &= queens_mask - 1;

            let mut attacks_mask = magic_bitboard::get_queen_attacks_mask(board.all_pieces, from);
            attacks_mask &= !us_mask;

            while attacks_mask != 0 {
                let to = attacks_mask.trailing_zeros() as u8;
                attacks_mask &= attacks_mask - 1;

                let flag = if 1 << to & them_mask != 0 {
                    MoveFlag::Capture
                } else {
                    MoveFlag::QuietMove
                };

                move_list.push(Move::new(from, to, flag));
            }
        }
    }

    fn generate_king_moves(board: &Board, move_list: &mut Vec<Move>) {
        let king_mask: u64;
        let us_mask: u64;
        let them_mask: u64;

        if board.active_color == Color::White {
            king_mask = board.pieces[Piece::WhiteKing as usize];
            us_mask = board.white_pieces;
            them_mask = board.black_pieces;
        } else {
            king_mask = board.pieces[Piece::BlackKing as usize];
            us_mask = board.black_pieces;
            them_mask = board.white_pieces;
        }

        let from = king_mask.trailing_zeros() as u8;

        let mut attacks_mask = attacks::KING_ATTACKS[from as usize];
        attacks_mask &= !us_mask;

        while attacks_mask != 0 {
            let to = attacks_mask.trailing_zeros() as u8;
            attacks_mask &= attacks_mask - 1;

            let flag = if 1 << to & them_mask != 0 {
                MoveFlag::Capture
            } else {
                MoveFlag::QuietMove
            };

            move_list.push(Move::new(from, to, flag));
        }

        if board.active_color == Color::White {
            if from != Square::E1 {
                return;
            }

            if board.castling_rights.has(CastlingRights::WHITE_KINGSIDE)
                && board.all_pieces & bitboard::WHITE_KINGSIDE_CASTLING == 0
            {
                move_list.push(Move::new(from, Square::G1 as u8, MoveFlag::KingCastle));
            }

            if board.castling_rights.has(CastlingRights::WHITE_QUEENSIDE)
                && board.all_pieces & bitboard::WHITE_QUEENSIDE_CASTLING == 0
            {
                move_list.push(Move::new(from, Square::C1 as u8, MoveFlag::QueenCastle));
            }
        } else {
            if from != Square::E8 {
                return;
            }

            if board.castling_rights.has(CastlingRights::BLACK_KINGSIDE)
                && board.all_pieces & bitboard::BLACK_KINGSIDE_CASTLING == 0
            {
                move_list.push(Move::new(from, Square::G8 as u8, MoveFlag::KingCastle));
            }

            if board.castling_rights.has(CastlingRights::BLACK_QUEENSIDE)
                && board.all_pieces & bitboard::BLACK_QUEENSIDE_CASTLING == 0
            {
                move_list.push(Move::new(from, Square::C8 as u8, MoveFlag::QueenCastle));
            }
        }
    }

    #[inline(always)]
    fn append_promotions(from: u8, to: u8, move_list: &mut Vec<Move>) {
        move_list.extend([
            Move::new(from, to, MoveFlag::PromoteN),
            Move::new(from, to, MoveFlag::PromoteB),
            Move::new(from, to, MoveFlag::PromoteR),
            Move::new(from, to, MoveFlag::PromoteQ),
        ])
    }

    #[inline(always)]
    fn append_capture_promotions(from: u8, to: u8, move_list: &mut Vec<Move>) {
        move_list.extend([
            Move::new(from, to, MoveFlag::PromoteCaptureN),
            Move::new(from, to, MoveFlag::PromoteCaptureB),
            Move::new(from, to, MoveFlag::PromoteCaptureR),
            Move::new(from, to, MoveFlag::PromoteCaptureQ),
        ])
    }
}
