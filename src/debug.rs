use crate::constants::board_constants::{A_FILE, RANK1};
use crate::{shift_board_right, is_square_set, shift_board_up};

pub fn print_bit_board (board: usize) {
    let space = " ".repeat(20);
    println!("\n{}+--------+", space);
    for i in (0..8).rev() {
        print!("{}|", space);
        for j in 0..8 {
            let shift = i * 8 + j;
            let bit = (board & (1 << shift)) >> shift;
            print!("{}", bit);

        }
        print!("|\n");
    }
    println!("{}+--------+", space);
}




pub fn get_square_name (square: usize) -> String{
    let mut name = "".to_string();
    let files = ["A","B","C","D","E","F","G","H"];
    name += files[(0..8).position(|i| is_square_set!(shift_board_right!(A_FILE, i), square)).unwrap()];

    let index = (0..8).position(|i| is_square_set!(shift_board_up!(RANK1, i), square)).unwrap();
    name += &(index + 1).to_string(); // there is no A0 square, Ranks stars from 1 not 0;
    name
}

