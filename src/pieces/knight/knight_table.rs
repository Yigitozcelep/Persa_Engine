use crate::board_components::{BitBoard, Square, ChessBoard};
use crate::constants::board_constants::*;
use crate::constants::directions::*;

static mut KNIGHT_TABLE: ChessBoard<BitBoard> = ChessBoard::from([BitBoard::new(); 64]);

pub fn initialize_knight_table() {
    for square in Square::create_squares(0, 64) {
        unsafe {KNIGHT_TABLE[square] = mask_knight_attacks(square);}
    }
}

fn mask_knight_attacks(square: Square) -> BitBoard {
    let mut attack = BitBoard::new();

    if !(TOP_2_RANK    | A_FILE).is_square_set(square) {attack.set_bit(square + NORTH * 2 + WEST);}
    if !(TOP_2_RANK    | H_FILE).is_square_set(square) {attack.set_bit(square + NORTH * 2 + EAST);}
    if !(RIGHT_2_FILE  | RANK8 ).is_square_set(square) {attack.set_bit(square + EAST * 2 + NORTH);}
    if !(RIGHT_2_FILE  | RANK1 ).is_square_set(square) {attack.set_bit(square + EAST * 2 + SOUTH);}
    if !(BOTTOM_2_RANK | H_FILE).is_square_set(square) {attack.set_bit(square + SOUTH * 2 + EAST);}
    if !(BOTTOM_2_RANK | A_FILE).is_square_set(square) {attack.set_bit(square + SOUTH * 2 + WEST);}
    if !(LEFT_2_FILE   | RANK1 ).is_square_set(square) {attack.set_bit(square + WEST * 2 + SOUTH);}
    if !(LEFT_2_FILE   | RANK8 ).is_square_set(square) {attack.set_bit(square + WEST * 2 + NORTH);}

    attack
}

pub fn generate_knight_attacks(square: Square) -> BitBoard {
    unsafe {KNIGHT_TABLE[square]}
}