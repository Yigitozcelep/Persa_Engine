use crate::board_components::{BitBoard, Color, Square};
use crate::pieces::bishop::bishop_table::generate_bishop_attacks;
use crate::pieces::king::king_table::generate_king_attacks;
use crate::pieces::knight::knight_table::generate_knight_attacks;
use crate::pieces::pawn::pawn_table::genereate_pawn_attacks;
use crate::pieces::queen::queen_table::generate_queen_attacks;
use crate::pieces::rook::rook_table::generate_rook_attakcs;

use std::ops::Index;
pub static ASCII_PIECES: [&'static str; 12] = ["p", "n", "b", "r", "q", "k", "P", "N", "B", "R", "Q", "K"];
pub static UNICODE_PIECES: [char; 12] =       ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔'];



pub enum Castles {
    WhiteKingSide  = 0b1,     // 1
    WhiteQueenSide = 0b10,    // 2
    BlackKingSide  = 0b100,   // 4
    BlackQueenSide = 0b1000,  // 8
}

pub struct BoardStatus (pub [BitBoard; 15]);

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum BoardStatusIndex {
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

impl Index<BoardStatusIndex> for BoardStatus {
    type Output = BitBoard;
    fn index(&self, index: BoardStatusIndex) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl BoardStatus {

    #[inline(always)]
    pub const fn new() -> Self {
        Self([BitBoard::new(); 15])
    }

    pub fn set_piece_bit(&mut self, piece: BoardStatusIndex, square: Square) {
        let i = (piece as usize) / 6;
        self.0[piece as usize].set_bit(square);
        self.0[13 - i].set_bit(square);
        self.0[BoardStatusIndex::AllPieces as usize].set_bit(square);
    }

    #[inline(always)]
    pub fn get_attacked_squares(&self, side: Color) -> BitBoard {
        let start = (side as usize) * 6;
        let mut attacks = BitBoard::new();
        let board = self.0.clone();
        let all_pieces = self[BoardStatusIndex::AllPieces];
        for sqaure in board[start]     {attacks = attacks | genereate_pawn_attacks(sqaure, side);}
        for sqaure in board[start + 1] {attacks = attacks | generate_knight_attacks(sqaure);}
        for sqaure in board[start + 2] {attacks = attacks | generate_bishop_attacks(sqaure, all_pieces);}
        for sqaure in board[start + 3] {attacks = attacks | generate_rook_attakcs(sqaure, all_pieces);}
        for sqaure in board[start + 4] {attacks = attacks | generate_queen_attacks(sqaure, all_pieces);}
        for sqaure in board[start + 5] {attacks = attacks | generate_king_attacks(sqaure);}
        
        attacks
    }
    #[inline(always)]
    pub const fn get_other_side_pieces(&self, side: Color) -> BitBoard{
        self.0[side as usize + 12]
    }

    #[inline(always)]
    pub fn is_square_attacked(&self, square: Square, side: Color) -> bool {
        self.get_other_side_pieces(side).is_square_set(square)
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

pub fn str_to_piece(asci_piece: &str) -> BoardStatusIndex {
    match asci_piece {
      "P" => BoardStatusIndex::WhitePawn,
      "N" => BoardStatusIndex::WhiteKnight,
      "B" => BoardStatusIndex::WhiteBishop,
      "R" => BoardStatusIndex::WhiteRook,
      "Q" => BoardStatusIndex::WhiteQueen,
      "K" => BoardStatusIndex::WhiteKing,
      
      "p" => BoardStatusIndex::BlackPawn,
      "n" => BoardStatusIndex::BlackKnight,
      "b" => BoardStatusIndex::BlackBishop,
      "r" => BoardStatusIndex::BlackRook,
      "q" => BoardStatusIndex::BlackQueen,
      "k" => BoardStatusIndex::BlackKing,
        _ => panic!("invalid string piece")
    }
}


pub fn print_fen_board(fen: String) {
    let mut data = ['.'; 64];
    let mut square = 56; // A8
    for c in fen.chars() {
        if c.is_numeric() {
            for _ in 0..c.to_digit(10).unwrap() {
                data[square] = '.';
                square += 1;
            }
        }
        else if c == '/' {square -= 16;} // below rank first square A1, A2 .. A7 
        else {
            data[square] = c;
            square += 1;
        }
    }
    let mut res = BitBoard::get_bitboard_string(data);
    for &s in &ASCII_PIECES {
        let piece = str_to_piece(s);
        let unicode = UNICODE_PIECES[piece as usize];
        res = res.replace(s, &unicode.to_string());
    }
    println!("{}", res);
}