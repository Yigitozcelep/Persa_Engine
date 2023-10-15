use crate::board_components::{Board, Square};
use std::ops::Index;
use crate::constants::board_constants::*;
use crate::constants::directions::*;

pub struct KnightTable([Board; 64]);

impl KnightTable {
    pub fn new() -> Self {
        let knight_table: [Board; 64] = Square::create_squares(0, 64)
                                        .map(|sqaure| mask_knight_attacks(sqaure))
                                        .collect::<Vec<_>>().try_into().unwrap();
        Self(knight_table)
    }
}

impl Index<Square> for KnightTable {
    type Output = Board;
    fn index(&self, index: Square) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

fn mask_knight_attacks(square: Square) -> Board {
    let mut attack = Board::new();

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
