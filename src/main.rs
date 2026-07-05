mod board;
mod evaluate;
mod movegen;
mod search;
mod zobrist;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let b = board::Board::new_starting_board()?;
    b.print_board();

    Ok(())
}
