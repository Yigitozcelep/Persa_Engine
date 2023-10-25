use crate::board_components::{BitBoard, Square, Color, ChessBoard};
use crate::constants::board_constants::*;
use crate::constants::directions::*;

pub static mut PAWN_TABLE: [ChessBoard<BitBoard>; 2] = [ChessBoard::from([BitBoard::new(); 64]), ChessBoard::from([BitBoard::new(); 64])];

pub fn initialize_pawn_table() {
    for square in Square::create_squares(0, 64) {
        unsafe {
            PAWN_TABLE[Color::White as usize][square] = mask_pawn_attacks(Color::White, square);
            PAWN_TABLE[Color::Black as usize][square] = mask_pawn_attacks(Color::Black, square);
        }
    }
}

fn mask_pawn_attacks(side: Color, square: Square) -> BitBoard {
    let mut attack = BitBoard::new();
    if RANK1.is_square_set(square) || RANK8.is_square_set(square) {return attack;}

    match side {
        Color::White => {
            if !A_FILE.is_square_set(square) {attack.set_bit(square + NORTH_WEST);}
            if !H_FILE.is_square_set(square) {attack.set_bit(square + NORTH_EAST);}
        }
        Color::Black => {
            if !A_FILE.is_square_set(square) {attack.set_bit(square + SOUTH_WEST);}
            if !H_FILE.is_square_set(square) {attack.set_bit(square + SOUTH_EAST);}
        }
    }
    return attack;
}

#[inline(always)]
pub fn genereate_pawn_attacks(square: Square, side: Color) -> BitBoard {
    unsafe {PAWN_TABLE[side as usize][square]}
}