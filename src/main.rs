#![allow(dead_code)]

use std::fmt::Display;

type SquareIndex = u8;

#[derive(Debug, Default)]
struct Bitboard(u64);

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
#[derive(Debug, Copy, Clone)]
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

impl Default for Square {
    fn default() -> Self {
        Square::OffBoard
    }
}

#[derive(Debug, Default)]
struct PieceKindCounts {
    pawns: u8,
    knights: u8,
    bishops: u8,
    rooks: u8,
    queens: u8,
    kings: u8,
}

#[derive(Debug, Default)]
struct ColoredData<T> {
    white: T,
    black: T,
    both: T,
}

#[derive(Debug, Default)]
struct ColoredPair<T> {
    white: T,
    black: T,
}

#[derive(Debug)]
struct Board {
    squares: [Square; 120],
    turn: Color,
    en_passant_target: Option<Position>,
    pawns: ColoredData<Bitboard>,
    pieces: ColoredData<PieceKindCounts>,
    big_pieces: ColoredData<u8>,
    major_pieces: ColoredData<u8>,
    minor_pieces: ColoredData<u8>,
    kings: ColoredPair<Position>,
}

impl Board {
    fn new() -> Self {
        Board {
            squares: [Square::OffBoard; 120],
            turn: Color::White,
            en_passant_target: None,
            pawns: ColoredData::default(),
            pieces: ColoredData::default(),
            big_pieces: ColoredData::default(),
            major_pieces: ColoredData::default(),
            minor_pieces: ColoredData::default(),
            kings: ColoredPair {
                white: Position::new(File::E, Rank::One),
                black: Position::new(File::E, Rank::Eight),
            },
        }
    }
}

#[derive(Debug)]
struct State {
    board: Board,
    ply: u32,
    history_ply: u32,
    fifty_moves: u8,
}

fn main() {
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

    #[test]
    fn test_board_initialization() {
        let board = Board::new();

        // Verify turn is initialized to White
        assert!(matches!(board.turn, Color::White));

        // Verify en_passant_target is None
        assert!(board.en_passant_target.is_none());

        // Verify all squares are initialized to OffBoard
        for square in board.squares.iter() {
            assert!(matches!(square, Square::OffBoard));
        }
    }

    #[test]
    fn test_board_kings_initialization() {
        let board = Board::new();

        // Verify kings are initialized at proper starting positions
        // White king at e1
        assert_eq!(board.kings.white.file, File::E);
        assert_eq!(board.kings.white.rank, Rank::One);

        // Black king at e8
        assert_eq!(board.kings.black.file, File::E);
        assert_eq!(board.kings.black.rank, Rank::Eight);
    }

    #[test]
    fn test_colored_data_default() {
        let data: ColoredData<Bitboard> = ColoredData::default();

        // Verify all bitboards are initialized to 0
        assert_eq!(data.white.0, 0);
        assert_eq!(data.black.0, 0);
        assert_eq!(data.both.0, 0);
    }

    #[test]
    fn test_piece_kind_counts_default() {
        let counts = PieceKindCounts::default();

        // Verify all piece counts are initialized to 0
        assert_eq!(counts.pawns, 0);
        assert_eq!(counts.knights, 0);
        assert_eq!(counts.bishops, 0);
        assert_eq!(counts.rooks, 0);
        assert_eq!(counts.queens, 0);
        assert_eq!(counts.kings, 0);
    }

    #[test]
    fn test_colored_pair_default() {
        let pair: ColoredPair<u8> = ColoredPair::default();

        // Verify both values are initialized to 0
        assert_eq!(pair.white, 0);
        assert_eq!(pair.black, 0);
    }

    #[test]
    fn test_bitboard_default() {
        let bitboard = Bitboard::default();

        // Verify bitboard is initialized to 0
        assert_eq!(bitboard.0, 0);
    }

    #[test]
    fn test_board_piece_counts_initialization() {
        let board = Board::new();

        // Verify piece counts are all initialized to 0
        assert_eq!(board.pieces.white.pawns, 0);
        assert_eq!(board.pieces.white.knights, 0);
        assert_eq!(board.pieces.black.pawns, 0);
        assert_eq!(board.pieces.both.queens, 0);

        // Verify big/major/minor piece counts are 0
        assert_eq!(board.big_pieces.white, 0);
        assert_eq!(board.major_pieces.black, 0);
        assert_eq!(board.minor_pieces.both, 0);
    }
}
