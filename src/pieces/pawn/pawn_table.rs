use crate::board_components::{Board, Square, Color};
use std::ops::Index;
use crate::constants::board_constants::*;
use crate::constants::directions::*;

pub struct PawnTable([[Board; 64]; 2]);

impl PawnTable {
    pub fn new() -> Self {
        let pawn_table: [[Board; 64]; 2] = [
            Square::create_squares(0, 64).map(|square| mask_pawn_attacks(Color::White, square)).collect::<Vec<_>>().try_into().unwrap(),
            Square::create_squares(0, 64).map(|square| mask_pawn_attacks(Color::Black, square)).collect::<Vec<_>>().try_into().unwrap(),
        ];
        Self(pawn_table)
    }
}

impl Index<(Color, Square)> for PawnTable {
    type Output = Board;
    fn index(&self, index: (Color, Square)) -> &Self::Output {
        &self.0[index.0 as usize][index.1.0 as usize]
    }
}

fn mask_pawn_attacks(side: Color, square: Square) -> Board {
    let mut attack = Board::new();
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
