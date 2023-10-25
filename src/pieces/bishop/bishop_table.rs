use crate::constants::directions::*;
use crate::constants::board_constants::EDGES;
use crate::board_components::{BitBoard, Square, ChessBoard, MagicNum};
use crate::constants::board_constants::{BISHOP_MAX_BLOCK_PERM, create_bishop_move_counts, create_bishop_magics};
use crate::pieces::helper_functions::{initialize_slider_attacks, initialize_slider_table, generate_slider_moves};
use std::cmp::{min, max};

static mut BISHOP_TABLE: ChessBoard<[BitBoard; BISHOP_MAX_BLOCK_PERM]> = ChessBoard::from([[BitBoard::new(); BISHOP_MAX_BLOCK_PERM]; 64]);
static mut BISHOP_ATTACKS: ChessBoard<BitBoard> = ChessBoard::from([BitBoard::new(); 64]);
static BISHOP_MOVES_COUNTS: ChessBoard<u64> = create_bishop_move_counts();
static BISHOP_MAGICS: ChessBoard<MagicNum> = create_bishop_magics();

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
pub fn generate_bishop_attacks(square: Square, board: BitBoard) -> BitBoard {
    unsafe {generate_slider_moves(square, board, &BISHOP_ATTACKS, &BISHOP_MAGICS, &BISHOP_TABLE, &BISHOP_MOVES_COUNTS)}
}

pub fn initialize_bishop_components() {
    unsafe {initialize_slider_table(&mut BISHOP_TABLE, &BISHOP_MAGICS, mask_bishop_attacks, bishop_attacks_on_fly);}
    unsafe {initialize_slider_attacks(mask_bishop_attacks, &mut BISHOP_ATTACKS)}
}
