use crate::board_components::{BitBoard, Color, Square};
use crate::pieces::bishop::bishop_table::generate_bishop_attacks;
use crate::pieces::king::king_table::generate_king_attacks;
use crate::pieces::knight::knight_table::generate_knight_attacks;
use crate::pieces::pawn::pawn_table::genereate_pawn_attacks;
use crate::pieces::queen::queen_table::generate_queen_attacks;
use crate::pieces::rook::rook_table::generate_rook_attakcs;
use crate::constants::board_constants::{UNICODE_PIECES, EMPTY_BITBOARD};
use std::ops::Index;



pub enum Castles {
    WhiteKingSide  = 0b1,     // 1
    WhiteQueenSide = 0b10,    // 2
    BlackKingSide  = 0b100,   // 4
    BlackQueenSide = 0b1000,  // 8
}

pub struct BoardStatus (pub [BitBoard; 15]);

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum BoardSlots {
    WhitePawn   = 0,
    WhiteKnight = 1,
    WhiteBishop = 2,
    WhiteRook   = 3,
    WhiteQueen  = 4,
    WhiteKing   = 5,
 
    BlackPawn   = 6,
    BlackKnight = 7,
    BlackBishop = 8,
    BlackRook   = 9,
    BlackQueen  = 10,
    BlackKing   = 11,
 
    BlackPieces = 12,
    WhitePieces = 13,
    AllPieces   = 14,
}

impl Index<BoardSlots> for BoardStatus {
    type Output = BitBoard;
    fn index(&self, index: BoardSlots) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl BoardStatus {
 
    #[inline(always)]
    pub const fn new() -> Self {
        Self([BitBoard::new(); 15])
    }

    pub fn set_piece_bit(&mut self, piece: BoardSlots, square: Square) {
        let i = (piece as usize) / 6;
        self.0[piece as usize].set_bit(square);
        self.0[13 - i].set_bit(square);
        self.0[BoardSlots::AllPieces as usize].set_bit(square);
    }

    #[inline(always)]
    pub fn get_attacked_squares(&self, side: Color) -> BitBoard {
        let start = (side as usize) * 6;
        let mut attacks = BitBoard::new();
        let board = self.0.clone();
        let all_pieces = self[BoardSlots::AllPieces];
        for sqaure in board[start]     {attacks = attacks | genereate_pawn_attacks(sqaure, side);}
        for sqaure in board[start + 1] {attacks = attacks | generate_knight_attacks(sqaure);}
        for sqaure in board[start + 2] {attacks = attacks | generate_bishop_attacks(sqaure, all_pieces);}
        for sqaure in board[start + 3] {attacks = attacks | generate_rook_attakcs(sqaure, all_pieces);}
        for sqaure in board[start + 4] {attacks = attacks | generate_queen_attacks(sqaure, all_pieces);}
        for sqaure in board[start + 5] {attacks = attacks | generate_king_attacks(sqaure);}
        
        attacks
    }
    #[inline(always)]
    pub fn is_square_attacked(&self, side: Color, square: Square) -> bool {
        let knight_attack  = generate_knight_attacks(square);
        let king_attack    = generate_king_attacks(square);
        let bishop_attacks = generate_bishop_attacks(square, self[BoardSlots::AllPieces]);
        let rook_attacks   = generate_rook_attakcs(square,   self[BoardSlots::AllPieces]);
        let queen_attacks  = bishop_attacks | rook_attacks;
        match side {
            Color::White => {
                (genereate_pawn_attacks(square, Color::Black) & self[BoardSlots::BlackPawn] |
                knight_attack  & self[BoardSlots::BlackKnight] |
                bishop_attacks & self[BoardSlots::BlackBishop] |
                rook_attacks   & self[BoardSlots::BlackRook]   |
                queen_attacks  & self[BoardSlots::BlackKing]   |
                king_attack    & self[BoardSlots::BlackKing]) != EMPTY_BITBOARD
            }
            Color::Black => {
                (genereate_pawn_attacks(square, Color::White) & self[BoardSlots::WhitePawn] |
                knight_attack  & self[BoardSlots::WhiteKnight] |
                bishop_attacks & self[BoardSlots::WhiteBishop] |
                rook_attacks   & self[BoardSlots::WhiteRook]   |
                queen_attacks  & self[BoardSlots::WhiteKing]   |
                king_attack    & self[BoardSlots::WhiteKing]) != EMPTY_BITBOARD
            }
        }
    }
    
    #[inline(always)]
    pub const fn get_other_side_pieces(&self, side: Color) -> BitBoard{
        self.0[side as usize + 12]
    }

}

impl std::fmt::Display for BoardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = ['.'; 64];
        let boards = self.0.clone();
        for (i, el) in UNICODE_PIECES.iter().enumerate() {
            for square in boards[i] {
                data[square.0 as usize] = *el;
            }
        }
        let result = BitBoard::get_bitboard_string(data);
        writeln!(f, "{}", result)
    }
}
