use crate::board::{Board, Color, Piece};

pub struct Evaluate;

impl Evaluate {
    pub fn sided_eval(board: &Board) -> i32 {
        let score = Self::eval(board);
        if board.active_color == Color::White {
            score
        } else {
            -score
        }
    }

    pub fn eval(board: &Board) -> i32 {
        Self::count_material(board) * 100
    }

    fn count_material(board: &Board) -> i32 {
        let mut material = 0;
        material += board.pieces[Piece::WhitePawn as usize].count_ones() as i32;
        material += board.pieces[Piece::WhiteKnight as usize].count_ones() as i32 * 3;
        material += board.pieces[Piece::WhiteBishop as usize].count_ones() as i32 * 3;
        material += board.pieces[Piece::WhiteRook as usize].count_ones() as i32 * 5;
        material += board.pieces[Piece::WhiteQueen as usize].count_ones() as i32 * 9;

        material -= board.pieces[Piece::BlackPawn as usize].count_ones() as i32;
        material -= board.pieces[Piece::BlackKnight as usize].count_ones() as i32 * 3;
        material -= board.pieces[Piece::BlackBishop as usize].count_ones() as i32 * 3;
        material -= board.pieces[Piece::BlackRook as usize].count_ones() as i32 * 5;
        material -= board.pieces[Piece::BlackQueen as usize].count_ones() as i32 * 9;

        material
    }
}
