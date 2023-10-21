use std::ops::Index;

use crate::board_components::{ChessBoard, BitBoard, MagicNum, Square};
use crate::pieces::helper_functions::get_possible_occupancy;

pub struct SliderPieceTable<const SIZE: usize> {
    table:       ChessBoard<[BitBoard ; SIZE]>,
    attacks:     ChessBoard<BitBoard>,
    magics:      ChessBoard<MagicNum>,
    move_counts: ChessBoard<u64>,
}

impl <const SIZE: usize> Index<(Square, BitBoard)> for SliderPieceTable<SIZE> {
    type Output = BitBoard;
    fn index(&self, index: (Square, BitBoard)) -> &Self::Output {
        let occupancy = index.1 & self.attacks[index.0];
        let magic_index = (occupancy * self.magics[index.0]) >> (64 - self.move_counts[index.0]);
        &self.table[index.0][magic_index as usize]
    }
}


impl <const SIZE: usize> SliderPieceTable<SIZE> {
    
    pub fn new(move_counts: ChessBoard<u64>, mask_attacks: fn(Square) -> BitBoard, attack_on_fly: fn(Square, BitBoard) -> BitBoard, magics: ChessBoard<MagicNum>) -> Self {
        let mut table   = ChessBoard::from([[BitBoard::new(); SIZE]; 64]);
        let mut attacks = ChessBoard::from([BitBoard::new(); 64]);
        
        for square in Square::create_squares(0, 64) {
            let attack = mask_attacks(square);
            attacks[square] = attack;
            let move_count = attack.count_ones();
            let total_mask_pos = 2_u64.pow(move_count);

            for index in 0..total_mask_pos {
                let occupancy = get_possible_occupancy(attack, index);
                let magic_index = (occupancy * magics[square]) >> (64 - move_count);
                table[square][magic_index as usize] = attack_on_fly(square, occupancy);
            }
        }
        Self {table, attacks, magics, move_counts}
    }
}