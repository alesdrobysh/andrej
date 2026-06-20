pub type SquareIndex = usize;

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