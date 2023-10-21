use crate::constants::board_constants::EDGES;
use crate::constants::directions::*;
use crate::board_components::{BitBoard, Square};
use crate::constants::board_constants::{BISHOP_MAX_BLOCK_PERM, create_bishop_move_counts, create_bishop_magics};
use crate::pieces::pieces_structs::SliderPieceTable;
use std::cmp::{min, max};

pub fn mask_bishop_attacks(square: Square) -> BitBoard {
    bishop_attacks_on_fly(square, BitBoard::new()) & !EDGES
}

pub fn bishop_attacks_on_fly(square: Square, blocker: BitBoard) -> BitBoard {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = BitBoard::new();

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

#[inline(always)]
pub fn create_bishop_table() -> SliderPieceTable<BISHOP_MAX_BLOCK_PERM> {
    SliderPieceTable::new(create_bishop_move_counts(), mask_bishop_attacks, bishop_attacks_on_fly, create_bishop_magics())
}
