use crate::constants::board_constants::*;
use crate::constants::directions::*;
use crate::debug;
use crate::helper_functions::mask_direction;


fn mask_rook_attacks(square: usize) -> usize {
    mask_direction(square, NORTH, RANK8)  | 
    mask_direction(square, EAST,  H_FILE) |
    mask_direction(square, SOUTH, RANK1)  |
    mask_direction(square, WEST,  A_FILE)
}


pub fn create_rook_table() {
    for square in 0..64 {
        println!("------------------------------\nsquare: {}", debug::get_square_name(square));
        debug::print_bit_board(mask_rook_attacks(square));
    }
}