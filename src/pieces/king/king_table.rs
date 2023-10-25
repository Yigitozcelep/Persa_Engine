use crate::board_components::{BitBoard, Square, ChessBoard};
use crate::constants::board_constants::*;
use crate::constants::directions::*;

static mut KING_TABLE: ChessBoard<BitBoard> = ChessBoard::from([BitBoard::new(); 64]);

pub fn initialize_king_table() {
    for square in Square::create_squares(0, 64) {
        unsafe {KING_TABLE[square] = mask_king_attacks(square);}
    }
}

fn mask_king_attacks(square: Square) -> BitBoard {
    let mut attack = BitBoard::new();
    if !RANK8.is_square_set(square)            {attack.set_bit(square + NORTH);}
    if !(RANK8 | H_FILE).is_square_set(square) {attack.set_bit(square + NORTH_EAST);}
    if !H_FILE.is_square_set(square)           {attack.set_bit(square + EAST);}
    if !(RANK1 | H_FILE).is_square_set(square) {attack.set_bit(square + SOUTH_EAST);}
    if !RANK1.is_square_set(square)            {attack.set_bit(square + SOUTH);}
    if !(RANK1 | A_FILE).is_square_set(square) {attack.set_bit(square + SOUTH_WEST);}
    if !A_FILE.is_square_set(square)           {attack.set_bit(square + WEST);}
    if !(RANK8 | A_FILE).is_square_set(square) {attack.set_bit(square + NORTH_WEST);}
    
    attack
}

#[inline(always)]
pub fn generate_king_attacks(square: Square) -> BitBoard{
    unsafe {KING_TABLE[square]}
}