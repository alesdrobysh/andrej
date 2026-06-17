mod board;

use board::{Board, Bitboard};

fn main() {
    // let board = Board::new();
    // println!("{}", board);
    // println!("{}", board.pawns.both);

    let mut bb = Bitboard::default();

    bb.set(9);
    bb.set(12);

    println!("{}count:{}", bb, bb.count());

    bb.set(63);

    println!("{}count:{}", bb, bb.count());

    bb.clear(9);

    println!("{}count:{}", bb, bb.count());

    println!("popping: {}", bb.pop());
    println!("popping: {}", bb.pop());

    println!("{}count:{}", bb, bb.count());

    println!("popping: {}", bb.pop());

    println!("{}count:{}", bb, bb.count());
}
