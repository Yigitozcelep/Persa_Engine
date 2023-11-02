use crate::board_components::{BitBoard, Square, Color};
use crate::constants::directions::SOUTH;
use crate::constants::squares::NO_SQUARE;
use crate::constants::{squares::{A8, H1}, directions::*};
use crate::pieces::pieces_controller::{BoardStatus, BoardSlots, CastleSlots, Castles};
use crate::constants::board_constants::{UNICODE_PIECES, ASCII_PIECES, H_FILE};


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

pub struct FenString {
    board:     String,
    color:     String,
    castles:   String,
    enpassant: String,
    half_move: String,
    full_move: String,
}

impl FenString {
    pub fn new(fen: String) -> Self { 
        let data: Vec<&str> = fen.split(" ").collect();
        Self {
            board: data[0].to_string(),
            color: data[1].to_string(),
            castles: data[2].to_string(),
            enpassant: data[3].to_string(),
            half_move: data[4].to_string(),
            full_move: data[5].to_string()
        }
    }
    
    pub fn from_board(board_status: &BoardStatus) -> Self {
        let mut board_data = ["."; 64];
        for piece in BoardSlots::iterate_pieces() {
            for square in board_status[piece] {
                board_data[square.0 as usize] = ASCII_PIECES[piece as usize];
            }
        }
        let mut counter = 0;
        let mut board: String = "".to_string();
        let mut square = A8;
        loop {
            if board_data[square.0 as usize].to_string() == "." {counter += 1;}
            else {
                if counter != 0 {board += &counter.to_string()}
                counter = 0;
                board += &board_data[square.0 as usize].to_string();
            }

            if square == H1 {
                if counter != 0 {board += &counter.to_string();}
                break;
            }
            square = square + EAST;
            if square + WEST != H1 && H_FILE.is_square_set(square + WEST) {
                if counter != 0 {board += &counter.to_string();}
                counter = 0;
                board += "/";
                square = square + WEST * 8 + SOUTH;
            }
            
        }
        
        let color = match board_status.get_color() {
            Color::White => "w".to_string(),
            Color::Black => "b".to_string(),
        };
        
        let mut castles = "".to_string();
        let mut flag = true;
        if board_status.can_castle(CastleSlots::WhiteKingSide)  {castles += "K"; flag = false;}
        if board_status.can_castle(CastleSlots::WhiteQueenSide) {castles += "Q"; flag = false;}
        if board_status.can_castle(CastleSlots::BlackKingSide)  {castles += "k"; flag = false;}
        if board_status.can_castle(CastleSlots::BlackQueenSide) {castles += "q"; flag = false;}
        if flag {castles += "-";}
        
        let mut enpassant = "-".to_string();
        let enpassant_square = board_status.get_enpassant();
        if enpassant_square != NO_SQUARE {
            enpassant = enpassant_square.get_name().to_lowercase().to_string();
        }
        let half_move = board_status.get_half_move().to_string();
        let full_move = board_status.get_full_move().to_string();
        Self { board, color, castles, enpassant, half_move, full_move}
    }

    pub fn convert_to_board(&self) -> BoardStatus {
        let mut enpassant_square = NO_SQUARE;
        if self.enpassant != "-" {
            enpassant_square = Square::create_squares(0, 64).find(|num| num.get_name().to_lowercase() == self.enpassant).unwrap();
        }
        let color = if self.color == "w" {Color::White} else {Color::Black};
        let half_move: usize = self.half_move.parse().unwrap();
        let full_move: usize = self.full_move.parse().unwrap();
        let mut board = BoardStatus::from([BitBoard::new(); 15], color, enpassant_square, Castles::new(), half_move, full_move);

        let mut square = A8 + WEST; // undefined square but it helps to create array
        for c in self.board.chars() {
            if c.is_numeric() {square = square + (EAST * c.to_digit(10).unwrap() as u8)}
            else if c == '/' {square = (square + WEST * 8) + SOUTH; print!("\n");}
            else {
                let piece = str_to_piece(c.to_string().as_str());
                square = square + EAST;
                board.set_piece_bit(piece, square);
            }
        }
        
        if !self.castles.contains("K") {board.remove_castle(CastleSlots::WhiteKingSide);}
        if !self.castles.contains("k") {board.remove_castle(CastleSlots::BlackKingSide);}
        if !self.castles.contains("Q") {board.remove_castle(CastleSlots::WhiteQueenSide);}
        if !self.castles.contains("q") {board.remove_castle(CastleSlots::BlackQueenSide);}
        board
    }

    pub fn adjust_board_display(&self) -> String {
        let mut data = ['.'; 64];
        let mut square = 56; // A8
        for c in self.board.chars() {
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
        res = res.chars().enumerate().map(|data| if data.0 != 459 {data.1} else {'B'}).collect();
        res = format!("\n----------------------------------------------------------------\nColor: {}, Castles: {}, Enpassant: {}, Half_move: {} Full_move: {}\n{}", self.color, self.castles, self.enpassant, self.half_move, self.full_move, res);
        res
    }
}

impl std::fmt::Display for FenString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.adjust_board_display())
    }
}
