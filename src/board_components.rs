use std::ops::{BitAnd, BitOr, Not, Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Board (pub u64);

impl BitAnd for Board {
    type Output = Board;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Board(self.0 & rhs.0)
    }
}

impl BitOr for Board {
    type Output = Board;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Board(self.0 | rhs.0)
    }
}

impl Not for Board {
    type Output = Board;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Board(!self.0)
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

    pub fn print_bit_board (&self) {
        let first_space = " ".repeat(20);
        let second_space = " ".repeat(2);
        println!("\n{} {}+-----------------+", first_space, second_space);
        for i in (0..8).rev() {
            print!("{}{}{}| ", first_space, (i + 1).to_string(), second_space);
            for j in 0..8 {
                let shift = i * 8 + j;
                let bit = (self.0 & (1 << shift)) >> shift;
                print!("{} ", bit);
    
            }
            print!("|\n");
        }
        println!("{} {}+-----------------+\n", first_space, second_space);
        let files = ["A","B","C","D","E","F","G","H"];
        print!("{} {}  ", first_space, second_space);
        for i in 0..8 {
            print!("{} ", files[i]);
        }
        println!("\n");
    }
}

impl Iterator for Board {
    type Item = Square;
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
    pub fn get_file(&self) -> u8 {self.0 % 8}
    pub fn get_rank(&self) -> u8 {self.0 / 8}

    pub fn get_name(&self) -> String{
        let files = ["A","B","C","D","E","F","G","H"];
        files[self.get_file() as usize].to_string() + &(self.get_rank() + 1).to_string()
    }

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
