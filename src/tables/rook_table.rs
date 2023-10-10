use std::f32::consts::E;

use crate::constants::board_constants::*;
use crate::constants::directions::*;
use crate::debug;
use crate::helper_functions::{mask_direction, get_possible_occupancy};
use crate::is_square_set;


pub fn mask_rook_attacks(square: usize) -> usize {
    mask_direction(square, NORTH, RANK8)  | 
    mask_direction(square, EAST,  H_FILE) |
    mask_direction(square, SOUTH, RANK1)  |
    mask_direction(square, WEST,  A_FILE)
}

// THE EDGES included because other wise it leds to bug, 
// for example if square is A8 then nothing can stop him from go to one square up (NORTH)
pub fn magic_mask_rook_attakcs(square: usize) -> usize {
    mask_direction(square, NORTH, RANK7  | RANK8)  |
    mask_direction(square, EAST,  G_FILE | H_FILE) |
    mask_direction(square, SOUTH, RANK1  | RANK2)  |
    mask_direction(square, WEST,  A_FILE |B_FILE)
}

pub fn create_rook_table() {
    for square in 0..1 {
        let attack = mask_rook_attacks(square) & !CORNERS;
        for index in 0..4096 {
            println!("-----------------------------\nindex: {}", index);
            debug::print_bit_board(get_possible_occupancy(attack, index));
        }
    }
}