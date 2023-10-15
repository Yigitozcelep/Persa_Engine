use crate::constants::board_constants::*;
use crate::constants::directions::*;
use crate::pieces::helper_functions::{mask_direction, get_possible_occupancy};
use crate::board_components::{Board, Square};


pub fn mask_rook_attacks(square: Square) -> Board {
    mask_direction(square, NORTH, RANK8)  | 
    mask_direction(square, EAST,  H_FILE) |
    mask_direction(square, SOUTH, RANK1)  |
    mask_direction(square, WEST,  A_FILE)
}

// THE EDGES included because other wise it leds to bug, 
// for example if square is A8 then nothing can stop him from go to one square up (NORTH)
pub fn magic_mask_rook_attakcs(square: Square) -> Board {
    mask_direction(square, NORTH, RANK7  | RANK8)  |
    mask_direction(square, EAST,  G_FILE | H_FILE) |
    mask_direction(square, SOUTH, RANK1  | RANK2)  |
    mask_direction(square, WEST,  A_FILE |B_FILE)
}

pub fn create_rook_table() {
    for square in 0..1 {
        let attack = mask_rook_attacks(Square(square)) & !CORNERS;
        for index in 0..4096 {
            println!("-----------------------------\nindex: {}", index);
            get_possible_occupancy(attack, Board(index)).print_bit_board();
        }
    }
}