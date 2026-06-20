mod bitboard;
mod board;
mod shared;

use board::Board;

fn main() {
    let board = Board::new();
    println!("{}", board);
}
