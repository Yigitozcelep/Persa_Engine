use crate::constants::board_constants::*;
use crate::constants::directions::*;
use crate::constants::squares::A8;
use crate::pieces::helper_functions::mask_direction;
use crate::board_components::{Board, Square};


pub fn mask_bishop_attacks(square: Square) -> Board {
    mask_direction(square, NORTH_EAST, H_FILE | RANK8) | 
    mask_direction(square, NORTH_WEST, A_FILE | RANK8) |
    mask_direction(square, SOUTH_EAST, H_FILE | RANK1) |
    mask_direction(square, SOUTH_WEST, A_FILE | RANK1)
}


pub fn create_bishop_table() {
    let square = A8;
    println!("------------------------------\nsquare: {}", square.get_name());
    mask_bishop_attacks(square).print_bit_board(); 
}