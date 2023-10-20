use crate::constants::board_constants::*;
use crate::constants::directions::*;
use crate::pieces::helper_functions::{get_possible_occupancy};
use crate::board_components::{Board, Square};


pub fn mask_rook_attacks(square: Square) -> Board {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = Board::new();

    for i in 1..7 - file {attacks.set_bit(square + EAST * i);}

    for i in 1..file {attacks.set_bit(square + WEST * i);}
    
    for i in 1..7 - rank {attacks.set_bit(square + NORTH * i);}

    for i in 1..rank {attacks.set_bit(square + SOUTH * i);}
    attacks
}

pub fn rook_attacks_on_fly(square: Square, blockers: Board) -> Board {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = Board::new();
    for i in 1..8 - file {
        attacks.set_bit(square + EAST * i);
        if blockers.is_square_set(square + EAST * i) {break;}
    }
    for i in 1..file + 1{
        attacks.set_bit(square + WEST * i);
        if blockers.is_square_set(square + WEST * i) {break;}
    }
    
    for i in 1..8 - rank {
        attacks.set_bit(square + NORTH * i);
        if blockers.is_square_set(square + NORTH * i) {break;}
    }

    for i in 1..rank + 1 {
        attacks.set_bit(square + SOUTH * i);
        if blockers.is_square_set(square + SOUTH * i) {break;}
    }
    attacks
}


pub fn create_rook_table() {
    for square in 0..1 {
        let attack = rook_attacks_on_fly(Square(square), Board::new()) & !CORNERS;
        for index in 0..4096 {
            println!("-----------------------------\nindex: {}", index);
            get_possible_occupancy(attack, Board(index as u64));
        }
    }
}