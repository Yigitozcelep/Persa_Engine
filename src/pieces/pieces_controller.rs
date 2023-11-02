use crate::board_components::{BitBoard, Color, Square, Direction};
use crate::constants::directions::{NORTH, SOUTH};
use crate::constants::squares::NO_SQUARE;
use crate::debug::FenString;
use crate::pieces::bishop::bishop_table::generate_bishop_attacks;
use crate::pieces::king::king_table::generate_king_attacks;
use crate::pieces::knight::knight_table::generate_knight_attacks;
use crate::pieces::pawn::pawn_table::genereate_pawn_attacks;
use crate::pieces::queen::queen_table::generate_queen_attacks;
use crate::pieces::rook::rook_table::generate_rook_attakcs;
use crate::constants::board_constants::{EMPTY_BITBOARD, RANK1, RANK2, RANK7, RANK8};
use std::mem::{transmute, MaybeUninit};
use std::ops::Index;


#[repr(u8)]
pub enum CastleSlots {
    WhiteKingSide  = 0b1,     // 1
    WhiteQueenSide = 0b10,    // 2
    BlackKingSide  = 0b100,   // 4
    BlackQueenSide = 0b1000,  // 8
}

pub struct Castles(u8);

impl Castles {
    #[inline(always)]
    pub fn remove_castle(&mut self, castle: CastleSlots) {
        self.0 ^= castle as u8
    }
    #[inline(always)]
    pub const fn new() -> Self {
        Self(0b1111)
    }
}

pub struct BoardStatus {
    boards: [BitBoard; 15], 
    color: Color, 
    castles: Castles, 
    enpassant: Square,
    half_move: usize,
    full_move: usize,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl BoardSlots {
    #[inline(always)]
    pub fn iterate_pieces() -> impl Iterator<Item=BoardSlots> {
        unsafe {(BoardSlots::WhitePawn as usize..=BoardSlots::BlackKing as usize).map(|num| transmute(num))}
    }
}

impl Index<BoardSlots> for BoardStatus {
    type Output = BitBoard;
    fn index(&self, index: BoardSlots) -> &Self::Output {
        &self.boards[index as usize]
    }
}

impl BoardStatus {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            boards: [BitBoard::new(); 15],
            color: Color::White,
            enpassant: NO_SQUARE,
            castles: Castles::new(),
            half_move: 0,
            full_move: 0,
        }
    }

    #[inline(always)]
    pub fn get_half_move(&self) -> usize {self.half_move}
    
    #[inline(always)]
    pub fn get_full_move(&self) -> usize {self.full_move}
    
    #[inline(always)]
    pub fn get_enpassant(&self) -> Square { self.enpassant }

    #[inline(always)]
    pub fn get_color(&self) -> Color { self.color }

    pub fn can_castle(&self, castle: CastleSlots) -> bool {
        (self.castles.0 & (castle as u8)) != 0
    }
    pub fn from(boards: [BitBoard; 15], color: Color, enpassant: Square, castles: Castles, half_move: usize, full_move: usize) -> Self {
        Self {boards, color, enpassant, castles, half_move, full_move}
    }

    pub fn remove_castle(&mut self, castle: CastleSlots) {
        self.castles.remove_castle(castle)
    }

    #[inline(always)]
    pub fn change_color(&mut self) {
        self.color = unsafe {transmute((self.color as usize + 1) % 2)}
    }

    #[inline(always)]
    pub fn get_pieces_board(&mut self, piece: BoardSlots) -> &mut BitBoard {
        let i = (piece as usize) / 6;
        &mut self.boards[13 - i]
    }

    #[inline(always)]
    pub fn set_piece_bit(&mut self, piece: BoardSlots, square: Square) {
        self.boards[piece as usize].set_bit(square);
        self.get_pieces_board(piece).set_bit(square);
        self.boards[BoardSlots::AllPieces as usize].set_bit(square);
    }

    #[inline(always)]
    pub fn get_attacked_squares(&self) -> BitBoard {
        let mut attacks = BitBoard::new();
        let board = self.boards.clone();
        let all_pieces = self[BoardSlots::AllPieces];
        match self.color {
            Color::White => {
                for sqaure in board[BoardSlots::WhitePawn   as usize] {attacks = attacks | genereate_pawn_attacks(sqaure, self.color);}
                for sqaure in board[BoardSlots::WhiteKnight as usize] {attacks = attacks | generate_knight_attacks(sqaure);}
                for sqaure in board[BoardSlots::WhiteBishop as usize] {attacks = attacks | generate_bishop_attacks(sqaure, all_pieces);}
                for sqaure in board[BoardSlots::WhiteRook   as usize] {attacks = attacks | generate_rook_attakcs(sqaure, all_pieces);}
                for sqaure in board[BoardSlots::WhiteQueen  as usize] {attacks = attacks | generate_queen_attacks(sqaure, all_pieces);}
                for sqaure in board[BoardSlots::WhiteKing   as usize] {attacks = attacks | generate_king_attacks(sqaure);}
            }
            Color::Black => {
                for sqaure in board[BoardSlots::BlackPawn   as usize] {attacks = attacks | genereate_pawn_attacks(sqaure, self.color);}
                for sqaure in board[BoardSlots::BlackKnight as usize] {attacks = attacks | generate_knight_attacks(sqaure);}
                for sqaure in board[BoardSlots::BlackBishop as usize] {attacks = attacks | generate_bishop_attacks(sqaure, all_pieces);}
                for sqaure in board[BoardSlots::BlackRook   as usize] {attacks = attacks | generate_rook_attakcs(sqaure, all_pieces);}
                for sqaure in board[BoardSlots::BlackQueen  as usize] {attacks = attacks | generate_queen_attacks(sqaure, all_pieces);}
                for sqaure in board[BoardSlots::BlackKing   as usize] {attacks = attacks | generate_king_attacks(sqaure);}
            }
        }
        attacks
    }
    #[inline(always)]
    pub fn is_square_attacked_by_side(&self, square: Square) -> bool {
        let knight_attack  = generate_knight_attacks(square);
        let king_attack    = generate_king_attacks(square);
        let bishop_attacks = generate_bishop_attacks(square, self[BoardSlots::AllPieces]);
        let rook_attacks   = generate_rook_attakcs(square,   self[BoardSlots::AllPieces]);
        let queen_attacks  = bishop_attacks | rook_attacks;
        match self.color {
            Color::White => {
                (genereate_pawn_attacks(square, Color::Black) & self[BoardSlots::WhitePawn] |
                knight_attack  & self[BoardSlots::WhiteKnight] |
                bishop_attacks & self[BoardSlots::WhiteBishop] |
                rook_attacks   & self[BoardSlots::WhiteRook]   |
                queen_attacks  & self[BoardSlots::WhiteKing]   |
                king_attack    & self[BoardSlots::WhiteKing]) != EMPTY_BITBOARD
            }
            Color::Black => {
                (genereate_pawn_attacks(square, Color::White) & self[BoardSlots::BlackPawn] |
                knight_attack  & self[BoardSlots::BlackKnight] |
                bishop_attacks & self[BoardSlots::BlackBishop] |
                rook_attacks   & self[BoardSlots::BlackRook]   |
                queen_attacks  & self[BoardSlots::BlackKing]   |
                king_attack    & self[BoardSlots::BlackKing]) != EMPTY_BITBOARD
            }
        }
    }
    
   

}

impl std::fmt::Display for BoardStatus {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", FenString::from_board(self).adjust_board_display())
    }
}


pub struct MoveBitField(u64);
impl MoveBitField {
    #[inline(always)]
    pub fn new(piece: BoardSlots, source: Square, target: Square) -> Self { 
        Self((source.0 as u64) | (target.0 as u64) << 6 | (piece as u64) << 12) 
    }

    #[inline(always)]
    pub fn set_promoted(mut self, piece: BoardSlots) -> Self { 
        self.0 |= (piece as u64) << 16; 
        self
    }

    #[inline(always)]
    pub fn set_capture(mut self) -> Self { 
        self.0 |= 1 << 20; 
        self
    }

    #[inline(always)]
    pub fn set_double(mut self) -> Self { 
        self.0 |= 1 << 21; 
        self
    }

    #[inline(always)]
    pub fn set_enpassant(mut self) -> Self { 
        self.0 |= 1 << 22; 
        self
    }

    #[inline(always)]
    pub fn set_castling(mut self) -> Self { 
        self.0 |= 1 << 23; 
        self
    }

    #[inline(always)]
    pub fn get_source(&self) -> Square { Square((self.0 & 0x3f) as u8) }
    
    #[inline(always)]
    pub fn get_target(&self) -> Square { Square(((self.0 & 0xfc0) >> 6) as u8) }

    #[inline(always)]
    pub fn get_piece(&self) -> BoardSlots { unsafe { transmute( (self.0 & 0xf000) >> 12 ) } }
    
    #[inline(always)]
    pub fn get_promoted(&self) -> BoardSlots { unsafe { transmute( (self.0 & 0xf0000) >> 16 ) } }

    #[inline(always)]
    pub fn is_move_capture(&self) -> bool { (self.0 & 0x100000) != 0  }
    
    #[inline(always)]
    pub fn is_move_double(&self) -> bool { (self.0 & 0x200000) != 0 }

    #[inline(always)]
    pub fn is_move_enpassant(&self) -> bool { (self.0 & 0x400000) != 0 }

    #[inline(always)]
    pub fn is_move_castling(&self) -> bool { (self.0 & 0x800000) != 0 }
}

#[inline(always)]
pub fn generate_knight_moves() {}

#[inline(always)]
pub fn generate_bishop_moves() {}

#[inline(always)]
pub fn generate_rook_moves() {}

#[inline(always)]
pub fn generate_queen_moves() {}

#[inline(always)]
pub fn generate_king_moves() {}


pub struct MoveList {
    moves: [MaybeUninit<MoveBitField>; 256],
    count: usize,
}

impl std::fmt::Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut result = "".to_string();
        for index in 0..self.count {
            result += &format!("Piece: {:?} ", self[index].get_piece()).to_string();
            result += &format!("Source: {} ", self[index].get_source()).to_string();
            result += &format!("Target: {} ", self[index].get_target()).to_string();
            if self[index].get_promoted() != BoardSlots::WhitePawn {result += &format!("Promoted: {:?} ", self[index].get_promoted()).to_string()};
            if self[index].is_move_capture() {result += &format!("Capture ");}
            if self[index].is_move_double() {result += &format!("Double ");}
            if self[index].is_move_enpassant() {result += &format!("Enpassant ");}
            if self[index].is_move_castling() {result += &format!("Castling: ")};
            result += "\n";
        }
        writeln!(f, "{}", result)
    }
}

impl MoveList {
    #[inline(always)]
    pub fn new() -> Self {
        unsafe {
            Self {
                moves: MaybeUninit::uninit().assume_init(), 
                count: 0,
             }
        }
    }
    #[inline(always)]
    pub fn append_move(&mut self, mov: MoveBitField) {
        self.moves[self.count].write(mov);
        self.count += 1;
    }

    #[inline(always)]
    pub fn generate_pawn_moves(&mut self, board: BitBoard, board_status: &BoardStatus, mov_dir: Direction, 
        double_move_line: BitBoard, fnish_line: BitBoard, pawn: BoardSlots, enemy_color: Color, queen: BoardSlots, 
        rook: BoardSlots, bishop: BoardSlots, knight: BoardSlots, enemy_pieces: BoardSlots) {
        
        for square in board {
            let target = square + mov_dir;
            if board_status[BoardSlots::AllPieces].is_square_set(target) { continue; }
            if fnish_line.is_square_set(target) {
                self.append_move(MoveBitField::new(pawn, square, target).set_promoted(queen));
                self.append_move(MoveBitField::new(pawn, square, target).set_promoted(rook));
                self.append_move(MoveBitField::new(pawn, square, target).set_promoted(bishop));
                self.append_move(MoveBitField::new(pawn, square, target).set_promoted(knight));
            }
            else { self.append_move(MoveBitField::new(pawn, square, target) ); }
            let double_move = target + mov_dir;
            if double_move_line.is_square_set(square) && !board_status[BoardSlots::AllPieces].is_square_set(double_move) {
                self.append_move(MoveBitField::new(pawn, square, double_move).set_double())
            }
            
            let attacks = genereate_pawn_attacks(square, board_status.color);
            for attack in attacks {
                if !board_status[enemy_pieces].is_square_set(attack) {continue;}
                if fnish_line.is_square_set(target) {
                    self.append_move(MoveBitField::new(pawn, square, attack).set_promoted(queen).set_capture());
                    self.append_move(MoveBitField::new(pawn, square, attack).set_promoted(rook).set_capture());
                    self.append_move(MoveBitField::new(pawn, square, attack).set_promoted(bishop).set_capture());
                    self.append_move(MoveBitField::new(pawn, square, attack).set_promoted(knight).set_capture());
                }
                else { self.append_move(MoveBitField::new(pawn, square, attack).set_capture() ); }
            }
        }
        if board_status.enpassant != NO_SQUARE {
            for square in genereate_pawn_attacks(board_status.enpassant, enemy_color) {
                if board_status[pawn].is_square_set(square) {
                    self.append_move(MoveBitField::new(pawn, square, board_status.enpassant).set_enpassant().set_capture())
                }
            }
        }
    }

    pub fn generate_moves(&mut self, board_status: BoardStatus) {
        match board_status.color {
            Color::White => self.generate_pawn_moves(board_status[BoardSlots::WhitePawn], &board_status, NORTH, RANK2, RANK8, BoardSlots::WhitePawn, Color::Black, BoardSlots::WhiteQueen, BoardSlots::WhiteRook, BoardSlots::WhiteBishop, BoardSlots::WhiteKnight, BoardSlots::BlackPieces),
            Color::Black => self.generate_pawn_moves(board_status[BoardSlots::BlackPawn], &board_status, SOUTH, RANK7, RANK1, BoardSlots::BlackPawn, Color::White, BoardSlots::BlackQueen, BoardSlots::BlackRook, BoardSlots::BlackBishop, BoardSlots::BlackKnight, BoardSlots::WhitePieces)
        }
        generate_knight_moves();
        generate_bishop_moves();
        generate_rook_moves();
        generate_queen_moves();
        generate_king_moves();
    }
    
}

impl Index<usize> for MoveList {
    type Output = MoveBitField;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.moves[index].assume_init_ref() }
    }
    
}

