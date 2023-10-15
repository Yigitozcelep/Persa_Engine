use crate::board_components::{Board, Square};
use std::ops::Index;
use crate::constants::board_constants::*;
use crate::constants::directions::*;

pub struct KingTable([Board; 64]);

impl KingTable {
    pub fn new() -> Self {
        let king_table: [Board; 64] = Square::create_squares(0, 64)
                                            .map(|square| mask_king_attacks(square))
                                            .collect::<Vec<_>>().try_into().unwrap();
        Self(king_table)
    }
}

impl Index<Square> for KingTable {
    type Output = Board;
    fn index(&self, index: Square) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

fn mask_king_attacks(square: Square) -> Board {
    let mut attack = Board::new();
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