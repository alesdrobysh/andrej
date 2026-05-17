mod board;

use board::Board;

fn main() {
    let board = Board::new();
    println!("{}", board);
    println!("{}", board.pawns.both);
}
