use crate::board_components::BitBoard;
use crate::constants::directions::SOUTH;
use crate::constants::{squares::A8, directions::*};
use crate::pieces::pieces_controller::{BoardStatus, BoardSlots};
use crate::constants::board_constants::{UNICODE_PIECES, ASCII_PIECES};


pub fn str_to_piece(asci_piece: &str) -> BoardSlots {
    match asci_piece {
      "P" => BoardSlots::WhitePawn,
      "N" => BoardSlots::WhiteKnight,
      "B" => BoardSlots::WhiteBishop,
      "R" => BoardSlots::WhiteRook,
      "Q" => BoardSlots::WhiteQueen,
      "K" => BoardSlots::WhiteKing,
      
      "p" => BoardSlots::BlackPawn,
      "n" => BoardSlots::BlackKnight,
      "b" => BoardSlots::BlackBishop,
      "r" => BoardSlots::BlackRook,
      "q" => BoardSlots::BlackQueen,
      "k" => BoardSlots::BlackKing,
        _ => panic!("invalid string piece")
    }
}

pub struct FenString(String);

impl FenString {
    pub fn new(fen: String) -> Self { Self(fen) }
    
    pub fn convert_to_board(&self) -> BoardStatus {
        let mut board = BoardStatus::new();
        let mut square = A8 + WEST; // undefined square but it helps to create array
        for c in self.0.chars() {
            if c.is_numeric() {square = square + (EAST * c.to_digit(10).unwrap() as u8)}
            else if c == '/' {square = (square + WEST * 8) + SOUTH; print!("\n");}
            else {
                let piece = str_to_piece(c.to_string().as_str());
                square = square + EAST;
                board.set_piece_bit(piece, square);
            }
        }
        board
    }
}

impl std::fmt::Display for FenString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = ['.'; 64];
        let mut square = 56; // A8
        for c in self.0.chars() {
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
        res = res.chars().enumerate().map(|data| if data.0 != 459 {data.1} else {'b'}).collect();
        write!(f, "{}", res)
    }
}
