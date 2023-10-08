use crate::constants::board_constants::*;
use crate::constants::directions::*;
use crate::constants::squares::A8;
use crate::debug;
use crate::helper_functions::mask_direction;


fn mask_bishop_attacks(square: usize) -> usize {
    mask_direction(square, NORTH_EAST, H_FILE | RANK8) | 
    mask_direction(square, NORTH_WEST, A_FILE | RANK8) |
    mask_direction(square, SOUTH_EAST, H_FILE | RANK1) |
    mask_direction(square, SOUTH_WEST, A_FILE | RANK1)
}


pub fn create_bishop_table() {
    let square = A8;
    println!("------------------------------\nsquare: {}", debug::get_square_name(square));
    debug::print_bit_board(mask_bishop_attacks(square));
    
}