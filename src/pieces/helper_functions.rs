use crate::board_components::{BitBoard, Square, MagicNumGenerator, MagicNum, ChessBoard};
use crate::constants::board_constants::EMPTY_BITBOARD;
use crate::pieces::rook::rook_table::initialize_rook_components;
use crate::pieces::bishop::bishop_table::initialize_bishop_components;
use crate::pieces::pawn::pawn_table::initialize_pawn_table;
use crate::pieces::knight::knight_table::initialize_knight_table;
use crate::pieces::king::king_table::initialize_king_table;
use std::sync::Once;

static INIT: Once = Once::new();
// Make sure that it called once
pub fn init_statics() {
    INIT.call_once( || {
        initialize_bishop_components();
        initialize_rook_components();
        initialize_pawn_table();
        initialize_knight_table();
        initialize_king_table();
    });
}

pub fn get_possible_occupancy(bitboard: BitBoard, index: u64) -> BitBoard {
    let mut occupancy =  BitBoard::new();
    
    for (count, square) in bitboard.enumerate() {
        if (index & 1 << count) != 0 {
            occupancy.set_bit(square);
        }
    }
    return occupancy;
}

pub fn find_magic_number(mask_attacks: fn(Square) -> BitBoard, attack_on_fly: fn(Square, BitBoard) -> BitBoard, square: Square) -> MagicNum {
    let attack_mask = mask_attacks(square);
    let move_count = attack_mask.count_ones();
    let mut occupancies:  [BitBoard; 4096] = [BitBoard::new(); 4096];
    let mut used_attacks: [BitBoard; 4096] = [BitBoard::new(); 4096];
    let mut attacks:      [BitBoard; 4096] = [BitBoard::new(); 4096];
    
    let mut magic_num_generator = MagicNumGenerator::new();

    let total_mask_pos: usize = (2_usize).pow(move_count as u32);
    (0..total_mask_pos).for_each(|index| {
        occupancies[index] = get_possible_occupancy(attack_mask, index as u64);
        attacks[index] = attack_on_fly(square, occupancies[index]);
    });

    'start: for _ in 0..1000000000 {
        let magic_num = magic_num_generator.gen();
        if ((attack_mask * magic_num) & 0xFF00000000000000).count_ones() < 6 {continue;}
        for el in used_attacks.iter_mut() {*el = BitBoard::new();}
        for index in 0..total_mask_pos {
            let magic_index = ((occupancies[index] * magic_num) >> (64 - move_count)) as usize;
            
            if used_attacks[magic_index] == EMPTY_BITBOARD { used_attacks[magic_index] = attacks[index];}
            else if used_attacks[magic_index] != attacks[index] {continue 'start;}
        }
        return magic_num;
    }
    
  unreachable!()
}

pub fn initialize_slider_table<const LEN: usize>(table: &mut ChessBoard<[BitBoard; LEN]>,  magics: &ChessBoard<MagicNum>, mask_attacks: fn(Square) -> BitBoard, attack_on_fly: fn(Square, BitBoard) -> BitBoard) {
    for square in Square::create_squares(0, 64) {
        let attack = mask_attacks(square);
        let move_count = attack.count_ones();
        let total_mask_pos = 2_u64.pow(move_count);
        for index in 0..total_mask_pos {
            let occupancy = get_possible_occupancy(attack, index);
            let magic_index = (occupancy * magics[square]) >> (64 - move_count);
            table[square][magic_index as usize] = attack_on_fly(square, occupancy);
        }
    }
}

pub fn initialize_slider_attacks(mask_attacks: fn(Square) -> BitBoard, attacks: &mut ChessBoard<BitBoard>) {
    for square in Square::create_squares(0, 64) {attacks[square] = mask_attacks(square);}
}

#[inline(always)]
pub fn generate_slider_moves<const LEN: usize> (square: Square, board: BitBoard, attacks: &ChessBoard<BitBoard>, 
    magics: &ChessBoard<MagicNum>, table: &ChessBoard<[BitBoard; LEN]>, move_counts: &ChessBoard<u64>) -> BitBoard {
    let occupancy = board & attacks[square];
    let magic_index = (occupancy * magics[square]) >> (64 - move_counts[square]);
    table[square][magic_index as usize]
}