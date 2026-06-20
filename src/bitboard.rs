#![allow(dead_code)]

use std::{fmt::Display, ops::{Add, BitAnd}};

use crate::shared::{SquareIndex, File, Rank, file_rank_to_64_index};

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
