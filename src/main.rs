#![allow(dead_code)]

use std::fmt::Display;

type SquareIndex = u8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Piece {
    kind: PieceKind,
    color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    fn to_char(self) -> char {
        (b'a' + self as u8) as char
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Rank {
    One = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    fn to_char(self) -> char {
        (b'1' + self as u8) as char
    }
}

struct Position {
    file: File,
    rank: Rank,
}

impl Position {
    fn new(file: File, rank: Rank) -> Self {
        Position { file, rank }
    }

    fn to_index(&self) -> SquareIndex {
        (self.rank as SquareIndex + 2) * 10 + (self.file as SquareIndex + 1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file.to_char(), self.rank.to_char())
    }
}

#[derive(Copy, Clone, Debug)]
enum Square {
    Empty,
    Occupied(Piece),
    OffBoard,
}

#[derive(Debug)]
struct Board([Square; 120]);

impl Default for Square {
    fn default() -> Self {
        Square::OffBoard
    }
}

impl Board {
    fn new() -> Self {
        Board([Square::OffBoard; 120])
    }
}

fn main() {
    let position = Position::new(File::A, Rank::One);
    println!("{} index is {}", position, position.to_index());

    let position = Position::new(File::H, Rank::Eight);
    println!("{} index is {}", position, position.to_index());

    let position = Position::new(File::D, Rank::Four);
    println!("{} index is {}", position, position.to_index());

    let board = Board::new();
    println!("{:?}", board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_to_char() {
        assert_eq!(File::A.to_char(), 'a');
        assert_eq!(File::B.to_char(), 'b');
        assert_eq!(File::C.to_char(), 'c');
        assert_eq!(File::D.to_char(), 'd');
        assert_eq!(File::E.to_char(), 'e');
        assert_eq!(File::F.to_char(), 'f');
        assert_eq!(File::G.to_char(), 'g');
        assert_eq!(File::H.to_char(), 'h');
    }

    #[test]
    fn test_rank_to_char() {
        assert_eq!(Rank::One.to_char(), '1');
        assert_eq!(Rank::Two.to_char(), '2');
        assert_eq!(Rank::Three.to_char(), '3');
        assert_eq!(Rank::Four.to_char(), '4');
        assert_eq!(Rank::Five.to_char(), '5');
        assert_eq!(Rank::Six.to_char(), '6');
        assert_eq!(Rank::Seven.to_char(), '7');
        assert_eq!(Rank::Eight.to_char(), '8');
    }

    #[test]
    fn test_position_to_index() {
        // Corner squares
        assert_eq!(Position::new(File::A, Rank::One).to_index(), 21);
        assert_eq!(Position::new(File::H, Rank::One).to_index(), 28);
        assert_eq!(Position::new(File::A, Rank::Eight).to_index(), 91);
        assert_eq!(Position::new(File::H, Rank::Eight).to_index(), 98);

        // Center square
        assert_eq!(Position::new(File::E, Rank::Four).to_index(), 55);
    }

    #[test]
    fn test_position_display() {
        assert_eq!(Position::new(File::A, Rank::One).to_string(), "a1");
        assert_eq!(Position::new(File::E, Rank::Four).to_string(), "e4");
        assert_eq!(Position::new(File::H, Rank::Eight).to_string(), "h8");
    }
}
