use crate::board_components::{BitBoard, Square, MagicNumGenerator, MagicNum, ChessBoard};
use crate::constants::board_constants::EMPTY_BITBOARD;

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