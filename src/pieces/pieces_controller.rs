use crate::board_components::{BitBoard, Color, Square, Direction};
use crate::constants::directions::{NORTH, SOUTH};
use crate::pieces::bishop::bishop_table::generate_bishop_attacks;
use crate::pieces::king::king_table::generate_king_attacks;
use crate::pieces::knight::knight_table::generate_knight_attacks;
use crate::pieces::pawn::pawn_table::genereate_pawn_attacks;
use crate::pieces::queen::queen_table::generate_queen_attacks;
use crate::pieces::rook::rook_table::generate_rook_attakcs;
use crate::constants::board_constants::{UNICODE_PIECES, EMPTY_BITBOARD};
use std::mem::transmute;
use std::ops::Index;
use std::mem::MaybeUninit;

pub enum Castles {
    WhiteKingSide  = 0b1,     // 1
    WhiteQueenSide = 0b10,    // 2
    BlackKingSide  = 0b100,   // 4
    BlackQueenSide = 0b1000,  // 8
}

pub struct BoardStatus (pub [BitBoard; 15]);


#[repr(usize)]
#[derive(Clone, Copy, Debug )]
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
    pub fn iterate_pieces() -> impl Iterator<Item = BoardSlots> {
        unsafe {(BoardSlots::WhitePawn as usize..=BoardSlots::BlackKing as usize).map(|num| transmute(num))}
    }
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

    #[inline(always)]
    pub fn get_pieces_board(&mut self, piece: BoardSlots) -> &mut BitBoard {
        let i = (piece as usize) / 6;
        &mut self.0[13 - i]
    }

    #[inline(always)]
    pub const fn get_other_side_pieces(&self, side: Color) -> BitBoard{
        self.0[side as usize + 12]
    }

    #[inline(always)]
    pub fn set_piece_bit(&mut self, piece: BoardSlots, square: Square) {
        self.0[piece as usize].set_bit(square);
        self.get_pieces_board(piece).set_bit(square);
        self.0[BoardSlots::AllPieces as usize].set_bit(square);
    }

    #[inline(always)]
    pub fn curr_side_start_idx(side: Color) -> usize {side as usize * 6}

    #[inline(always)]
    pub fn get_attacked_squares(&self, side: Color) -> BitBoard {
        let start = BoardStatus::curr_side_start_idx(side);
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
    pub fn is_square_attacked_by_side(&self, side: Color, square: Square) -> bool {
        let knight_attack  = generate_knight_attacks(square);
        let king_attack    = generate_king_attacks(square);
        let bishop_attacks = generate_bishop_attacks(square, self[BoardSlots::AllPieces]);
        let rook_attacks   = generate_rook_attakcs(square,   self[BoardSlots::AllPieces]);
        let queen_attacks  = bishop_attacks | rook_attacks;
        match side {
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


pub struct MoveBitField(u64);
impl MoveBitField {
    #[inline(always)]
    pub fn new() -> Self { Self(0) }

    #[inline(always)]
    pub fn set_source(&mut self, source: Square) { self.0 |= source.0 as u64; }

    #[inline(always)]
    pub fn set_target(&mut self, target: Square) { self.0 |= (target.0 as u64) << 6; }

    #[inline(always)]
    pub fn set_piece(&mut self, piece: BoardSlots) { self.0 |= (piece as u64) << 12; }

    #[inline(always)]
    pub fn set_promoted(&mut self, piece: BoardSlots) { self.0 |= (piece as u64) << 16; }

    #[inline(always)]
    pub fn set_capture(&mut self) { self.0 |= 1 << 20; }

    #[inline(always)]
    pub fn set_double(&mut self) { self.0 |= 1 << 21; }

    #[inline(always)]
    pub fn set_enpassant(&mut self) { self.0 |= 1 << 22; }

    #[inline(always)]
    pub fn set_castling(&mut self)  { self.0 |= 1 << 23; }

    #[inline(always)]
    pub fn get_source(&self) -> Square { Square((self.0 & 0x3f) as u8) }
    
    #[inline(always)]
    pub fn get_target(&self) -> Square { Square(((self.0 & 0xfc0) >> 6) as u8) }

    #[inline(always)]
    pub fn get_piece(&self) -> BoardSlots { unsafe { transmute( (self.0 & 0xf000) >> 12 ) } }
    
    #[inline(always)]
    pub fn get_promoted(&self) -> u64 { unsafe { transmute( (self.0 & 0xf0000) >> 16 ) } }

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
pub fn generate_pawn_moves(board: BitBoard, move_dir: Direction) {}

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


pub fn generate_moves(board_status: &mut BoardStatus, side: Color) {
    let mut source_square: Square = Square(0);
    let mut target_square: Square = Square(0);

    let attacks =  BitBoard::new();

    for piece in BoardSlots::iterate_pieces() {
        let board: BitBoard = board_status[piece];
        match side {
            Color::White => {
                generate_pawn_moves(board_status[BoardSlots::WhitePawn], NORTH)
            },
            Color::Black => {
                generate_pawn_moves(board_status[BoardSlots::BlackPawn], SOUTH)
            }
        }
        generate_knight_moves();
        generate_bishop_moves();
        generate_rook_moves();
        generate_queen_moves();
        generate_king_moves();
    }
}


pub struct MoveList {
    moves: [MaybeUninit<MoveBitField>; 256],
    count: usize,
}

impl std::fmt::Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "".to_string();
        for index in 0..self.count {
            result += &format!("Source: {} ", self[index].get_source()).to_string();
            result += &format!("Target: {} ", self[index].get_target()).to_string();
            result += &format!("Piece: {:?} ", self[index].get_piece()).to_string();
            result += &format!("Promoted: {:?} ", self[index].get_promoted()).to_string();
            result += &format!("Capture: {} ", self[index].is_move_capture()).to_string();
            result += &format!("Double: {} ", self[index].is_move_capture()).to_string();
            result += &format!("IsEnpassant: {} ", self[index].is_move_enpassant()).to_string();
            result += &format!("IsCastling: {} ", self[index].is_move_castling()).to_string();
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
    pub fn append_move(&mut self, mov: MoveBitField) {
        self.moves[self.count].write(mov);
        self.count += 1;
    }
}

impl Index<usize> for MoveList {
    type Output = MoveBitField;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.moves[index].assume_init_ref() }
    }
}

