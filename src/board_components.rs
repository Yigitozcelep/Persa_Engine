use std::ops::{BitAnd, BitOr, Not, Add, Mul};
use crate::{impl_square_index, impl_op, impl_indv_bit_op};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board (pub u64);

impl_op!(Board, BitAnd, bitand, 0);
impl_op!(Board, BitOr, bitor, 0);
impl_indv_bit_op!(Board, Not, not, 0);

 
impl Mul<MagicNum> for Board {
    type Output = u64;
    #[inline(always)]
    fn mul(self, rhs: MagicNum) -> Self::Output {
        self.0 * rhs.0
    }
}

impl Board {
    #[inline(always)]
    pub fn new() -> Self { Self(0) }

    #[inline(always)]
    pub fn is_square_set(&self, square: Square) -> bool {
        self.0 & (1 << square.0) != 0
    }

    #[inline(always)]
    pub fn set_bit(&mut self, square: Square) {
        self.0 |= 1 << square.0;
    }

    #[inline(always)]
    pub fn toggle_bit(&mut self, square: Square) {
        self.0 ^= 1 << square.0;
    }

    #[inline(always)]
    pub fn shift_board_left(&mut self, shift: u8) {
        self.0 = self.0 >> shift;
    }

    #[inline(always)]
    pub fn shift_board_right(&mut self, shift: u8) {
        self.0 = self.0 << shift;
    }

    #[inline(always)]
    pub fn shift_board_up(&mut self, shift: u8) {
        self.shift_board_right(shift * 8);
    }

    #[inline(always)]
    pub fn shift_board_down(&mut self, shift: u8) {
        self.shift_board_left(shift * 8);
    }
    
    #[inline(always)]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
    
    #[inline(always)]
    pub fn trailing_zeros(&self) -> u32 {
        self.0.trailing_zeros()
    }

    #[inline(always)]
    pub fn pop_square(&mut self) -> Square {
        debug_assert_ne!(self.0, 0);
        let result = Square(self.0.trailing_zeros() as u8);
        self.0 = self.0 & self.0 - 1;
        result
    }
}

impl Iterator for Board {
    type Item = Square;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            Some(self.pop_square())
        }
        else {None}
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_space = " ".repeat(20);
        let second_space = " ".repeat(2);
        let mut result = format!("\n{} {}+-----------------+\n", first_space, second_space);
        for i in (0..8).rev() {
            result += &format!("{}{}{}| ", first_space, (i + 1).to_string(), second_space);
            for j in 0..8 {
                let shift = i * 8 + j;
                let bit = (self.0 & (1 << shift)) >> shift;
                result += &format!("{} ", bit);
    
            }
            result += &format!("|\n");
        }
        result += &format!("{} {}+-----------------+\n\n", first_space, second_space);
        let files = ["A","B","C","D","E","F","G","H"];
        result += &format!("{} {}  ", first_space, second_space);
        for i in 0..8 {
            result += &format!("{} ", files[i]);
        }
        result += &format!("\n\n");
        write!(f, "{}", result)
    }
}
#[derive(Clone, Copy)]
pub struct Square(pub u8);

impl Square {   
    #[inline(always)]
    pub fn get_file(&self) -> u8 {self.0 % 8}
    #[inline(always)]
    pub fn get_rank(&self) -> u8 {self.0 / 8}

    pub fn get_name(&self) -> String{
        let files = ["A","B","C","D","E","F","G","H"];
        files[self.get_file() as usize].to_string() + &(self.get_rank() + 1).to_string()
    }

    #[inline(always)]
    pub fn create_squares(start: u8, end: u8) -> impl Iterator<Item=Square> {
        (start..end).map(Square)
    }
}
impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

impl Add<Direction> for Square {
    type Output = Square;
    #[inline(always)]
    fn add(self, rhs: Direction) -> Self::Output {
        Square(self.0 + rhs.0)
    }
}


#[derive(Clone, Copy)]
pub struct Direction(pub u8);

impl Add<Square> for Direction {
    type Output = Square;
    #[inline(always)]
    fn add(self, rhs: Square) -> Self::Output {
        Square(self.0 + rhs.0)
    }
}
impl Mul<u8> for Direction {
    type Output = Direction;
    #[inline(always)]
    fn mul(self, rhs: u8) -> Self::Output {
        Direction(self.0 * rhs)
    }
}

#[repr(usize)]
pub enum Color {
    White = 0,
    Black = 1,
}


pub struct MoveCounts([u8; 64]);
impl_square_index!(MoveCounts, u8, 0);

pub fn create_bishop_move_counts() -> MoveCounts {
    MoveCounts([
        6, 5, 5, 5, 5, 5, 5, 6, 
        5, 5, 5, 5, 5, 5, 5, 5, 
        5, 5, 7, 7, 7, 7, 5, 5, 
        5, 5, 7, 9, 9, 7, 5, 5, 
        5, 5, 7, 9, 9, 7, 5, 5, 
        5, 5, 7, 7, 7, 7, 5, 5, 
        5, 5, 5, 5, 5, 5, 5, 5, 
        6, 5, 5, 5, 5, 5, 5, 6
        ])
}

pub fn create_rook_move_counts() -> MoveCounts {
    MoveCounts([
        12, 11, 11, 11, 11, 11, 11, 12, 
        11, 10, 10, 10, 10, 10, 10, 11, 
        11, 10, 10, 10, 10, 10, 10, 11, 
        11, 10, 10, 10, 10, 10, 10, 11, 
        11, 10, 10, 10, 10, 10, 10, 11, 
        11, 10, 10, 10, 10, 10, 10, 11, 
        11, 10, 10, 10, 10, 10, 10, 11, 
        12, 11, 11, 11, 11, 11, 11, 12
        ])
}

pub struct MagicNumGenerator(u32);
impl MagicNumGenerator {
    pub fn new() -> Self {
       Self(1804289383)
    }
    #[inline(always)]
    pub fn get_random_u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        return self.0
    }
    
    #[inline(always)]
    pub fn get_random_u64(&mut self) -> u64 {
        let n1 = (self.get_random_u32() as u64) & 0xFFFF;
        let n2 = (self.get_random_u32() as u64) & 0xFFFF;
        let n3 = (self.get_random_u32() as u64) & 0xFFFF;
        let n4 = (self.get_random_u32() as u64) & 0xFFFF;
        n1 | (n2 << 16) | (n3 << 32) | (n4 << 48)
    }

    #[inline(always)]
    pub fn gen(&mut self) -> MagicNum {
        MagicNum(self.get_random_u64() & self.get_random_u64() & self.get_random_u64())
    }
}
#[derive(Clone, Copy, Debug)]
pub struct MagicNum(pub u64);


