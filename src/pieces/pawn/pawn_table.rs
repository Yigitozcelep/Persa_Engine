use crate::board_components::{BitBoard, Square, Color};
use std::ops::Index;
use crate::constants::board_constants::*;
use crate::constants::directions::*;

pub struct PawnTable([[BitBoard; 64]; 2]);

impl PawnTable {
    pub fn new() -> Self {
        let pawn_table: [[BitBoard; 64]; 2] = [
            Square::create_squares(0, 64).map(|square| mask_pawn_attacks(Color::White, square)).collect::<Vec<_>>().try_into().unwrap(),
            Square::create_squares(0, 64).map(|square| mask_pawn_attacks(Color::Black, square)).collect::<Vec<_>>().try_into().unwrap(),
        ];
        Self(pawn_table)
    }
}

impl Index<(Square, Color)> for PawnTable {
    type Output = BitBoard;
    fn index(&self, index: (Square, Color)) -> &Self::Output {
        &self.0[index.0.0 as usize][index.1 as usize]
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
