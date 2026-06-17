#![allow(dead_code)]

use colored::Colorize;
use std::{fmt::Display, ops::{Add, BitAnd}};

const BRD_SQ_NUM: usize = 120;
const MAX_SQ_NUM: usize = 64;

pub type SquareIndex = usize;

#[derive(Debug, Default, Copy, Clone)]
pub struct Bitboard(pub u64);


impl Bitboard {
    pub fn set(&mut self, index: SquareIndex) -> &mut Self {
        self.0 |= 1 << index;
        self
    }

    pub fn clear(&mut self, index: SquareIndex) -> &mut Self {
        self.0 &= !(1 << index);
        self
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn pop(&mut self) -> SquareIndex {
        let pop_index = self.0.trailing_zeros() as SquareIndex;
        self.clear(pop_index);
        pop_index
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Display for  Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shift_me: u64 = 1;
        let mut result = String::new();

        for rank in Rank::iter().rev() {
            for file in File::iter() {
                let index = file_rank_to_64_index(file.to_char(), rank.to_char());
                let bb = *self;
                let bb_shift = shift_me << (index - 1);
                
                if (bb_shift & bb.0) > 0 {
                    result = result.add("X");
                } else {
                    result = result.add("-");
                } 
            }

            result = result.add("\n");

        }

        writeln!(f, "{}", result)
    }
}

#[derive(Debug, Default)]
pub struct ZobristKey(pub u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl PieceKind {
    pub fn to_unicode(&self, _color: Color) -> &'static str {
        match self {
            PieceKind::Pawn => "♟",
            PieceKind::Knight => "♞",
            PieceKind::Bishop => "♝",
            PieceKind::Rook => "♜",
            PieceKind::Queen => "♛",
            PieceKind::King => "♚",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = self.kind.to_unicode(self.color);
        let emoji = match self.color {
            Color::White => symbol.truecolor(255, 255, 255).bold(),
            Color::Black => symbol.truecolor(0, 0, 0).bold(),
        };
        write!(f, "{}", emoji)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum File {
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
    pub fn to_char(self) -> char {
        (b'a' + self as u8) as char
    }

    pub fn iter() -> impl Iterator<Item = File> {
        (0..8).map(|i| unsafe { std::mem::transmute::<u8, File>(i) })
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a' => Some(File::A),
            'b' => Some(File::B),
            'c' => Some(File::C),
            'd' => Some(File::D),
            'e' => Some(File::E),
            'f' => Some(File::F),
            'g' => Some(File::G),
            'h' => Some(File::H),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
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
    pub fn to_char(self) -> char {
        (b'1' + self as u8) as char
    }

    pub fn iter() -> impl DoubleEndedIterator<Item = Rank> {
        (0..8).map(|i| unsafe { std::mem::transmute::<u8, Rank>(i) })
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Rank::One),
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            _ => None,
        }
    }
}

#[inline(always)]
pub fn file_rank_to_120_index(file: char, rank: char) -> SquareIndex {
    (rank as SquareIndex - '1' as SquareIndex + 2) * 10
        + (file as SquareIndex - 'a' as SquareIndex + 1)
}

#[inline(always)]
pub fn file_rank_to_64_index(file: char, rank: char) -> SquareIndex {
    (rank as SquareIndex - '1' as SquareIndex) * 8
        + (file as SquareIndex - 'a' as SquareIndex + 1)
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub file: File,
    pub rank: Rank,
}

impl Position {
    pub fn new(file: File, rank: Rank) -> Self {
        Position { file, rank }
    }

    pub fn to_index(&self) -> SquareIndex {
        file_rank_to_120_index(self.file.to_char(), self.rank.to_char())
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file.to_char(), self.rank.to_char())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Square {
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
pub struct PieceKindCounts {
    pub pawns: u8,
    pub knights: u8,
    pub bishops: u8,
    pub rooks: u8,
    pub queens: u8,
    pub kings: u8,
}

#[derive(Debug, Default)]
pub struct ColoredData<T> {
    pub white: T,
    pub black: T,
    pub both: T,
}

#[derive(Debug, Default)]
pub struct ColoredPair<T> {
    pub white: T,
    pub black: T,
}

#[derive(Debug)]
pub enum CastlingRight {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8,
}

#[derive(Debug)]
pub struct Undo {
    pub move_: u32,
    pub castling_rights: u8,
    pub en_passant_target: Option<Position>,
    pub fifty_move_counter: u8,
    pub position_key: ZobristKey,
}

#[derive(Debug)]
pub struct Board {
    pub squares: [Square; BRD_SQ_NUM],
    pub turn: Color,
    pub en_passant_target: Option<Position>,
    pub pawns: ColoredData<Bitboard>,
    pub pieces: ColoredData<PieceKindCounts>,
    pub big_pieces: ColoredData<u8>,
    pub major_pieces: ColoredData<u8>,
    pub minor_pieces: ColoredData<u8>,
    pub kings: ColoredPair<Position>,
    pub position_key: ZobristKey,
    pub castling_rights: u8,
    pub fifty_moves: u8,
    pub history: Vec<Undo>,
    pub ply: u32,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in ('1'..='8').rev() {
            write!(f, "{} ", rank)?;

            for file in 'a'..='h' {
                let index = file_rank_to_120_index(file, rank);
                let square = self.squares[index];

                let file = File::from_char(file).unwrap() as u8;
                let rank = Rank::from_char(rank).unwrap() as u8;

                let is_light = (file + rank) % 2 == 0;

                let (bg_r, bg_g, bg_b) = if is_light {
                    (180, 180, 180)
                } else {
                    (120, 120, 120)
                };

                let colored_cell = match square {
                    Square::Empty => "   ".on_truecolor(bg_r, bg_g, bg_b),
                    Square::Occupied(piece) => {
                        let content = format!(" {} ", piece);
                        content.on_truecolor(bg_r, bg_g, bg_b)
                    }
                    Square::OffBoard => "   ".on_truecolor(bg_r, bg_g, bg_b),
                };

                write!(f, "{}", colored_cell)?;
            }

            writeln!(f)?;
        }

        writeln!(f, "   a  b  c  d  e  f  g  h")?;
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [Square::OffBoard; BRD_SQ_NUM],
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
            position_key: ZobristKey::default(),
            castling_rights: CastlingRight::WhiteKingSide as u8
                | CastlingRight::WhiteQueenSide as u8
                | CastlingRight::BlackKingSide as u8
                | CastlingRight::BlackQueenSide as u8,
            fifty_moves: 0,
            history: Vec::new(),
            ply: 0,
        };

        for rank in Rank::iter() {
            for file in File::iter() {
                let index = file_rank_to_120_index(file.to_char(), rank.to_char());
                board.squares[index] = Square::Empty;
            }
        }

        let place = |board: &mut Board, file: File, rank: Rank, piece: Piece| {
            let pos = Position::new(file, rank);
            let index = pos.to_index() as usize;
            board.squares[index] = Square::Occupied(piece);
        };

        let white_back_rank = [
            (File::A, PieceKind::Rook),
            (File::B, PieceKind::Knight),
            (File::C, PieceKind::Bishop),
            (File::D, PieceKind::Queen),
            (File::E, PieceKind::King),
            (File::F, PieceKind::Bishop),
            (File::G, PieceKind::Knight),
            (File::H, PieceKind::Rook),
        ];

        for (file, kind) in white_back_rank {
            place(
                &mut board,
                file,
                Rank::One,
                Piece {
                    kind,
                    color: Color::White,
                },
            );
        }

        for file in File::iter() {
            place(
                &mut board,
                file,
                Rank::Two,
                Piece {
                    kind: PieceKind::Pawn,
                    color: Color::White,
                },
            );
        }

        let black_back_rank = [
            (File::A, PieceKind::Rook),
            (File::B, PieceKind::Knight),
            (File::C, PieceKind::Bishop),
            (File::D, PieceKind::Queen),
            (File::E, PieceKind::King),
            (File::F, PieceKind::Bishop),
            (File::G, PieceKind::Knight),
            (File::H, PieceKind::Rook),
        ];

        for (file, kind) in black_back_rank {
            place(
                &mut board,
                file,
                Rank::Eight,
                Piece {
                    kind,
                    color: Color::Black,
                },
            );
        }

        for file in File::iter() {
            place(
                &mut board,
                file,
                Rank::Seven,
                Piece {
                    kind: PieceKind::Pawn,
                    color: Color::Black,
                },
            );
        }

        board
    }
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
        assert_eq!(Position::new(File::A, Rank::One).to_index(), 21);
        assert_eq!(Position::new(File::H, Rank::One).to_index(), 28);
        assert_eq!(Position::new(File::A, Rank::Eight).to_index(), 91);
        assert_eq!(Position::new(File::H, Rank::Eight).to_index(), 98);

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

        assert!(matches!(board.turn, Color::White));
        assert!(board.en_passant_target.is_none());

        for i in 0..10 {
            assert!(matches!(board.squares[i], Square::OffBoard));
        }

        for i in 100..120 {
            assert!(matches!(board.squares[i], Square::OffBoard));
        }

        for rank in Rank::iter() {
            for file in File::iter() {
                let index = file_rank_to_120_index(file.to_char(), rank.to_char());
                assert!(
                    matches!(board.squares[index], Square::Empty | Square::Occupied(_)),
                    "Square at index {} should be Empty or Occupied",
                    index
                );
            }
        }
    }

    #[test]
    fn test_board_kings_initialization() {
        let board = Board::new();

        assert_eq!(board.kings.white.file, File::E);
        assert_eq!(board.kings.white.rank, Rank::One);

        assert_eq!(board.kings.black.file, File::E);
        assert_eq!(board.kings.black.rank, Rank::Eight);
    }

    #[test]
    fn test_colored_data_default() {
        let data: ColoredData<Bitboard> = ColoredData::default();

        assert_eq!(data.white.0, 0);
        assert_eq!(data.black.0, 0);
        assert_eq!(data.both.0, 0);
    }

    #[test]
    fn test_piece_kind_counts_default() {
        let counts = PieceKindCounts::default();

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

        assert_eq!(pair.white, 0);
        assert_eq!(pair.black, 0);
    }

    #[test]
    fn test_bitboard_default() {
        let bitboard = Bitboard::default();

        assert_eq!(bitboard.0, 0);
    }

    #[test]
    fn test_board_piece_counts_initialization() {
        let board = Board::new();

        assert_eq!(board.pieces.white.pawns, 0);
        assert_eq!(board.pieces.white.knights, 0);
        assert_eq!(board.pieces.black.pawns, 0);
        assert_eq!(board.pieces.both.queens, 0);

        assert_eq!(board.big_pieces.white, 0);
        assert_eq!(board.major_pieces.black, 0);
        assert_eq!(board.minor_pieces.both, 0);
    }

    #[test]
    fn test_piece_display() {
        fn piece_string(kind: PieceKind, color: Color) -> String {
            let symbol = kind.to_unicode(color);
            match color {
                Color::White => symbol.truecolor(255, 255, 255).bold().to_string(),
                Color::Black => symbol.truecolor(0, 0, 0).bold().to_string(),
            }
        }

        assert_eq!(
            piece_string(PieceKind::King, Color::White),
            piece_string(PieceKind::King, Color::Black)
        );
        assert_eq!(
            piece_string(PieceKind::Queen, Color::White),
            piece_string(PieceKind::Queen, Color::Black)
        );
        assert_eq!(
            piece_string(PieceKind::Rook, Color::White),
            piece_string(PieceKind::Rook, Color::Black)
        );
        assert_eq!(
            piece_string(PieceKind::Bishop, Color::White),
            piece_string(PieceKind::Bishop, Color::Black)
        );
        assert_eq!(
            piece_string(PieceKind::Knight, Color::White),
            piece_string(PieceKind::Knight, Color::Black)
        );
        assert_eq!(
            piece_string(PieceKind::Pawn, Color::White),
            piece_string(PieceKind::Pawn, Color::Black)
        );

        assert_eq!(piece_string(PieceKind::King, Color::White), "♚");
        assert_eq!(piece_string(PieceKind::Queen, Color::White), "♛");
        assert_eq!(piece_string(PieceKind::Rook, Color::White), "♜");
        assert_eq!(piece_string(PieceKind::Bishop, Color::White), "♝");
        assert_eq!(piece_string(PieceKind::Knight, Color::White), "♞");
        assert_eq!(piece_string(PieceKind::Pawn, Color::White), "♟");
    }

    #[test]
    fn test_board_starting_position() {
        let board = Board::new();

        let get_piece = |file: File, rank: Rank| {
            let pos = Position::new(file, rank);
            let index = pos.to_index() as usize;
            board.squares[index]
        };

        assert!(matches!(
            get_piece(File::A, Rank::One),
            Square::Occupied(Piece {
                kind: PieceKind::Rook,
                color: Color::White
            })
        ));
        assert!(matches!(
            get_piece(File::B, Rank::One),
            Square::Occupied(Piece {
                kind: PieceKind::Knight,
                color: Color::White
            })
        ));
        assert!(matches!(
            get_piece(File::C, Rank::One),
            Square::Occupied(Piece {
                kind: PieceKind::Bishop,
                color: Color::White
            })
        ));
        assert!(matches!(
            get_piece(File::D, Rank::One),
            Square::Occupied(Piece {
                kind: PieceKind::Queen,
                color: Color::White
            })
        ));
        assert!(matches!(
            get_piece(File::E, Rank::One),
            Square::Occupied(Piece {
                kind: PieceKind::King,
                color: Color::White
            })
        ));
        assert!(matches!(
            get_piece(File::F, Rank::One),
            Square::Occupied(Piece {
                kind: PieceKind::Bishop,
                color: Color::White
            })
        ));
        assert!(matches!(
            get_piece(File::G, Rank::One),
            Square::Occupied(Piece {
                kind: PieceKind::Knight,
                color: Color::White
            })
        ));
        assert!(matches!(
            get_piece(File::H, Rank::One),
            Square::Occupied(Piece {
                kind: PieceKind::Rook,
                color: Color::White
            })
        ));

        for file in File::iter() {
            assert!(matches!(
                get_piece(file, Rank::Two),
                Square::Occupied(Piece {
                    kind: PieceKind::Pawn,
                    color: Color::White
                })
            ));
        }

        assert!(matches!(
            get_piece(File::A, Rank::Eight),
            Square::Occupied(Piece {
                kind: PieceKind::Rook,
                color: Color::Black
            })
        ));
        assert!(matches!(
            get_piece(File::B, Rank::Eight),
            Square::Occupied(Piece {
                kind: PieceKind::Knight,
                color: Color::Black
            })
        ));
        assert!(matches!(
            get_piece(File::C, Rank::Eight),
            Square::Occupied(Piece {
                kind: PieceKind::Bishop,
                color: Color::Black
            })
        ));
        assert!(matches!(
            get_piece(File::D, Rank::Eight),
            Square::Occupied(Piece {
                kind: PieceKind::Queen,
                color: Color::Black
            })
        ));
        assert!(matches!(
            get_piece(File::E, Rank::Eight),
            Square::Occupied(Piece {
                kind: PieceKind::King,
                color: Color::Black
            })
        ));
        assert!(matches!(
            get_piece(File::F, Rank::Eight),
            Square::Occupied(Piece {
                kind: PieceKind::Bishop,
                color: Color::Black
            })
        ));
        assert!(matches!(
            get_piece(File::G, Rank::Eight),
            Square::Occupied(Piece {
                kind: PieceKind::Knight,
                color: Color::Black
            })
        ));
        assert!(matches!(
            get_piece(File::H, Rank::Eight),
            Square::Occupied(Piece {
                kind: PieceKind::Rook,
                color: Color::Black
            })
        ));

        for file in File::iter() {
            assert!(matches!(
                get_piece(file, Rank::Seven),
                Square::Occupied(Piece {
                    kind: PieceKind::Pawn,
                    color: Color::Black
                })
            ));
        }

        for rank in [Rank::Three, Rank::Four, Rank::Five, Rank::Six] {
            for file in File::iter() {
                assert!(matches!(get_piece(file, rank), Square::Empty));
            }
        }
    }
}
