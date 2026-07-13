mod attacks;
mod bitboard;
mod board;
mod evaluate;
mod magic_bitboard;
mod movegen;
mod mv;
mod search;
mod zobrist;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let b = board::Board::new_starting_board()?;
    b.print_board();

    Ok(())
}
