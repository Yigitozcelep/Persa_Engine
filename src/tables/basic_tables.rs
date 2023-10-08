use crate::constants::board_constants::*;
use crate::constants::types::{PawnTable, KnightTable, KingTable};
use crate::constants::directions::*;
use crate::{set_bit, is_square_set};



fn mask_pawn_attacks(side: usize, square: usize) -> usize{
    let mut attack: usize = 0;
    if is_square_set!(RANK1, square) || is_square_set!(RANK8, square) {return attack;}

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
        pawn_table[WHITE][square] = mask_pawn_attacks(WHITE, square);
        pawn_table[BLACK][square] = mask_pawn_attacks(BLACK, square);
    }
    pawn_table
}


fn mask_knight_attacks(square: usize) -> usize {
    let mut attack = 0;

    const TOP_2_RANK:    usize =   RANK8  | RANK7;
    const RIGHT_2_FILE:  usize =   G_FILE | H_FILE;
    const BOTTOM_2_RANK: usize =   RANK1  | RANK2;
    const LEFT_2_FILE:   usize =   A_FILE | B_FILE;

    if !is_square_set!(TOP_2_RANK    | A_FILE, square)  {set_bit!(attack, square + NORTH.wrapping_mul(2) + WEST);}
    if !is_square_set!(TOP_2_RANK    | H_FILE, square)  {set_bit!(attack, square + NORTH.wrapping_mul(2) + EAST);}
    if !is_square_set!(RIGHT_2_FILE  | RANK8,  square)  {set_bit!(attack, square + EAST.wrapping_mul(2)  + NORTH);}
    if !is_square_set!(RIGHT_2_FILE  | RANK1,  square)  {set_bit!(attack, square + EAST.wrapping_mul(2)  + SOUTH);}
    if !is_square_set!(BOTTOM_2_RANK | H_FILE, square)  {set_bit!(attack, square + SOUTH.wrapping_mul(2) + EAST);}
    if !is_square_set!(BOTTOM_2_RANK | A_FILE, square)  {set_bit!(attack, square + SOUTH.wrapping_mul(2) + WEST);}
    if !is_square_set!(LEFT_2_FILE   | RANK1,  square)  {set_bit!(attack, square + WEST.wrapping_mul(2)  + SOUTH);}
    if !is_square_set!(LEFT_2_FILE   | RANK8,  square)  {set_bit!(attack, square + WEST.wrapping_mul(2)  + NORTH);}
    
    attack
}

pub fn create_knight_table() -> KnightTable {
    let mut knight_table: KnightTable = [0; 64];
    for square in 0..64 {
        knight_table[square] = mask_knight_attacks(square);
    }
    knight_table
}

fn mask_king_attacks(square: usize) -> usize {
    let mut attack: usize = 0;
    if !is_square_set!(RANK8, square)           {set_bit!(attack, square + NORTH);}
    if !is_square_set!(RANK8 | H_FILE, square)  {set_bit!(attack, square + NORTH + EAST);}
    if !is_square_set!(H_FILE, square)          {set_bit!(attack, square + EAST);}
    if !is_square_set!(RANK1 | H_FILE, square)  {set_bit!(attack, square + SOUTH + EAST);}
    if !is_square_set!(RANK1, square)           {set_bit!(attack, square + SOUTH);}
    if !is_square_set!(RANK1 | A_FILE, square)  {set_bit!(attack, square + SOUTH + WEST);}
    if !is_square_set!(A_FILE, square)          {set_bit!(attack, square + WEST);}
    if !is_square_set!(RANK8 | A_FILE, square)  {set_bit!(attack, square + NORTH + WEST);}
    attack
}

pub fn create_king_table() -> KingTable {
    let mut king_table: KingTable = [0;64];
    for square in 0..64 {
        king_table[square] = mask_king_attacks(square);
    }
    king_table
}