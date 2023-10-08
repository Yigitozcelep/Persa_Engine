use crate::constants::board_constants::*;
use crate::constants::directions::*;
use crate::{set_bit, is_square_set};
type PawnTable =  [[usize; 64]; 2];

fn mask_pawn_attacks(side: usize, square: usize) -> usize{
    let mut attack: usize = 0;
    
    if side == WHITE {
        if !is_square_set!(A_FILE, square) {set_bit!(attack, square + NORTH_WEST);}
        if !is_square_set!(H_FILE, square) {set_bit!(attack, square + NORTH_EAST);}
    }

    else if side == BLACK {
        if !is_square_set!(A_FILE, square) {set_bit!(attack, square + SOUTH_WEST);}
        if !is_square_set!(H_FILE, square) {set_bit!(attack, square + SOUTH_EAST);}
    }
    
    return attack;
}

pub fn create_pawn_table() -> PawnTable {
    let mut pawn_table: PawnTable = [[0; 64]; 2];
    for square in 0..64 {
        if is_square_set!(RANK1, square) || is_square_set!(RANK8, square) {continue;}
        pawn_table[WHITE][square] = mask_pawn_attacks(WHITE, square);
        pawn_table[BLACK][square] = mask_pawn_attacks(BLACK, square);
    }
    pawn_table
}
