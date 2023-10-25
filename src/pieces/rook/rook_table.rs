use crate::constants::directions::*;
use crate::constants::board_constants::{ROOK_MAX_BLOCK_PERM, create_rook_move_counts, create_rook_magics};
use crate::board_components::{BitBoard, Square, ChessBoard, MagicNum};
use crate::pieces::helper_functions::{initialize_slider_attacks, initialize_slider_table, generate_slider_moves};


static mut ROOK_TABLE: ChessBoard<[BitBoard; ROOK_MAX_BLOCK_PERM]> = ChessBoard::from([[BitBoard::new(); ROOK_MAX_BLOCK_PERM]; 64]);
static mut ROOK_ATTACKS: ChessBoard<BitBoard> = ChessBoard::from([BitBoard::new(); 64]);
static ROOK_MOVES_COUNTS: ChessBoard<u64> = create_rook_move_counts();
static ROOK_MAGICS: ChessBoard<MagicNum> = create_rook_magics();

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
pub fn generate_rook_attakcs(square: Square, board: BitBoard) -> BitBoard{
    unsafe {generate_slider_moves(square, board, &ROOK_ATTACKS, &ROOK_MAGICS, &ROOK_TABLE, &ROOK_MOVES_COUNTS)}
}

pub fn initialize_rook_components() {
    unsafe {initialize_slider_table(&mut ROOK_TABLE, &ROOK_MAGICS, mask_rook_attacks, rook_attacks_on_fly);}
    unsafe {initialize_slider_attacks(mask_rook_attacks, &mut ROOK_ATTACKS)}
}

