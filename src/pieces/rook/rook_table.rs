use crate::constants::directions::*;
use crate::constants::board_constants::{ROOK_MAX_BLOCK_PERM, create_rook_move_counts, create_rook_magics};
use crate::board_components::{BitBoard, Square};
use crate::pieces::pieces_structs::SliderPieceTable;

pub fn mask_rook_attacks(square: Square) -> BitBoard {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = BitBoard::new();

    for i in 1..7 - file {attacks.set_bit(square + EAST * i);}

    for i in 1..file {attacks.set_bit(square + WEST * i);}
    
    for i in 1..7 - rank {attacks.set_bit(square + NORTH * i);}

    for i in 1..rank {attacks.set_bit(square + SOUTH * i);}
    attacks
}

pub fn rook_attacks_on_fly(square: Square, blockers: BitBoard) -> BitBoard {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = BitBoard::new();
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

#[inline(always)]
pub fn create_rook_table() -> SliderPieceTable<ROOK_MAX_BLOCK_PERM> {
    SliderPieceTable::new(create_rook_move_counts(), mask_rook_attacks, rook_attacks_on_fly, create_rook_magics())
}