#![allow(dead_code)]

use colored::Colorize;
use std::fmt::Display;

type SquareIndex = u8;

#[derive(Debug, Default)]
struct Bitboard(u64);

#[derive(Debug, Default)]
struct ZobristKey(u64);

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

impl PieceKind {
    fn to_unicode(&self, _color: Color) -> &'static str {
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
struct Piece {
    kind: PieceKind,
    color: Color,
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
enum CastlingRight {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8,
}

#[derive(Debug)]
struct Undo {
    move_: u32,
    castling_rights: u8,
    en_passant_target: Option<Position>,
    fifty_move_counter: u8,
    position_key: ZobristKey,
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
    position_key: ZobristKey,
    castling_rights: u8,
    fifty_moves: u8,
    history: Vec<Undo>,
    ply: u32,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Iterate from rank 8 down to rank 1 (top to bottom of display)
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;

            for file in 0..8 {
                let index = ((rank + 2) * 10 + (file + 1)) as usize;
                let square = self.squares[index];
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
    fn new() -> Self {
        let mut board = Board {
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
            position_key: ZobristKey::default(),
            castling_rights: CastlingRight::WhiteKingSide as u8
                | CastlingRight::WhiteQueenSide as u8
                | CastlingRight::BlackKingSide as u8
                | CastlingRight::BlackQueenSide as u8,
            fifty_moves: 0,
            history: Vec::new(),
            ply: 0,
        };

        for rank in 0..8 {
            for file in 0..8 {
                let index = ((rank + 2) * 10 + (file + 1)) as usize;
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

        for file in [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ] {
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

        for file in [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ] {
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

fn main() {
    let board = Board::new();
    println!("{}", board);
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

        // Verify off-board squares are OffBoard (sentinel squares)
        // Check first row (indices 0-9)
        for i in 0..10 {
            assert!(matches!(board.squares[i], Square::OffBoard));
        }

        // Check last two rows (indices 100-119)
        for i in 100..120 {
            assert!(matches!(board.squares[i], Square::OffBoard));
        }

        // Verify valid board squares are either Empty or Occupied
        for rank in 0..8 {
            for file in 0..8 {
                let index = ((rank + 2) * 10 + (file + 1)) as usize;
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

    #[test]
    fn test_piece_display() {
        // Both white and black pieces use the same filled symbols
        // Test white pieces
        assert_eq!(
            Piece {
                kind: PieceKind::King,
                color: Color::White
            }
            .to_string(),
            "♚"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Queen,
                color: Color::White
            }
            .to_string(),
            "♛"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Rook,
                color: Color::White
            }
            .to_string(),
            "♜"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Bishop,
                color: Color::White
            }
            .to_string(),
            "♝"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Knight,
                color: Color::White
            }
            .to_string(),
            "♞"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Pawn,
                color: Color::White
            }
            .to_string(),
            "♟"
        );

        // Test black pieces
        assert_eq!(
            Piece {
                kind: PieceKind::King,
                color: Color::Black
            }
            .to_string(),
            "♚"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Queen,
                color: Color::Black
            }
            .to_string(),
            "♛"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Rook,
                color: Color::Black
            }
            .to_string(),
            "♜"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Bishop,
                color: Color::Black
            }
            .to_string(),
            "♝"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Knight,
                color: Color::Black
            }
            .to_string(),
            "♞"
        );
        assert_eq!(
            Piece {
                kind: PieceKind::Pawn,
                color: Color::Black
            }
            .to_string(),
            "♟"
        );
    }

    #[test]
    fn test_board_starting_position() {
        let board = Board::new();

        // Helper to get piece at position
        let get_piece = |file: File, rank: Rank| {
            let pos = Position::new(file, rank);
            let index = pos.to_index() as usize;
            board.squares[index]
        };

        // Test white back rank
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

        // Test white pawns
        for file in [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ] {
            assert!(matches!(
                get_piece(file, Rank::Two),
                Square::Occupied(Piece {
                    kind: PieceKind::Pawn,
                    color: Color::White
                })
            ));
        }

        // Test black back rank
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

        // Test black pawns
        for file in [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ] {
            assert!(matches!(
                get_piece(file, Rank::Seven),
                Square::Occupied(Piece {
                    kind: PieceKind::Pawn,
                    color: Color::Black
                })
            ));
        }

        // Test empty squares (ranks 3-6)
        for rank in [Rank::Three, Rank::Four, Rank::Five, Rank::Six] {
            for file in [
                File::A,
                File::B,
                File::C,
                File::D,
                File::E,
                File::F,
                File::G,
                File::H,
            ] {
                assert!(matches!(get_piece(file, rank), Square::Empty));
            }
        }
    }
}
