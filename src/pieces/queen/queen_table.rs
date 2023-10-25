use crate::board_components::{Square, BitBoard};
use crate::pieces::bishop::bishop_table::generate_bishop_attacks;
use crate::pieces::rook::rook_table::generate_rook_attakcs;

pub fn generate_queen_attacks(square: Square, board: BitBoard) -> BitBoard {
    generate_rook_attakcs(square, board) | generate_bishop_attacks(square, board)
}