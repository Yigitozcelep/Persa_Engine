use crate::{is_square_set, clear_bit, set_bit, constants::board_constants::EDGES, debug, toggle_bit};

// boundries included
pub fn mask_direction(square: usize, direction: usize, boundry: usize) -> usize{
    let mut attack: usize = 0;

    for i in 0..8 {
        let target = square + direction.wrapping_mul(i);
        set_bit!(attack, target);
        if is_square_set!(boundry, target) {break;}
    }
    clear_bit!(attack, square);
    attack
}


pub fn get_possible_occupancy(mut board: usize, index: usize) -> usize {
    let mut occupancy: usize = 0;
    let bit_count = board.count_ones();
    for count in 0..bit_count {
        let bit_index = board.trailing_zeros();
        toggle_bit!(board, bit_index);
        if is_square_set!(index, count) {
            set_bit!(occupancy, bit_index);
        }
    }   
    return occupancy;
}