use crate::constants::board_constants::*;
use crate::constants::directions::*;
use crate::constants::squares::A8;
use crate::board_components::{Board, Square};
use std::cmp::{min, max};

pub fn mask_bishop_attacks(square: Square) -> Board {
    bishop_attacks_on_fly(square, Board::new()) & !EDGES
}

pub fn bishop_attacks_on_fly(square: Square, blocker: Board) -> Board {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = Board::new();

    for i in 1..8 - max(rank, file) {
        attacks.set_bit(square + NORTH_EAST * i);
        if blocker.is_square_set(square + NORTH_EAST * i) {break;}
    }
    for i in 1..min(rank, file) + 1 {
        attacks.set_bit(square + SOUTH_WEST * i);
        if blocker.is_square_set(square + SOUTH_WEST * i) {break;}
    }

    for i in 1..min(8 - rank, file + 1) {
        attacks.set_bit(square + NORTH_WEST * i);
        if blocker.is_square_set(square + NORTH_WEST * i) {break;}
    }
    
    for i in 1..min(rank + 1, 8 - file) {
        attacks.set_bit(square + SOUTH_EAST * i);
        if blocker.is_square_set(square + SOUTH_EAST * i) {break;}
    }

    attacks
}

pub fn create_bishop_table() {
    let square = A8;
    println!("------------------------------\nsquare: {}", square.get_name());
}