use crate::board_components::{BitBoard, Color, Square, Direction};
use crate::constants::directions::{NORTH, SOUTH, WEST, EAST, NORTH_EAST};
use crate::constants::squares::*;
use crate::debug::FenString;
use crate::pieces::tables::*;
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

pub struct MoveList {
    moves: [MaybeUninit<MoveBitField>; 256],
    pub count: usize,
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
            if self[index].is_move_castling() {result += &format!("Castling ")};
            result += "\n";
        }
        writeln!(f, "{}", result)
    }
}

#[inline(always)]
pub fn is_square_attacked_black(board_status: &BoardStatus, square: Square) -> bool {
    let knight_attack  = generate_knight_attacks(square);
    let king_attack    = generate_king_attacks(square);
    let bishop_attacks = generate_bishop_attacks(square, board_status[BoardSlots::AllPieces]);
    let rook_attacks   = generate_rook_attakcs(square,   board_status[BoardSlots::AllPieces]);
    let queen_attacks  = bishop_attacks | rook_attacks;
    (genereate_pawn_attacks(square, Color::Black) & board_status[BoardSlots::WhitePawn] |
    knight_attack  & board_status[BoardSlots::WhiteKnight] |
    bishop_attacks & board_status[BoardSlots::WhiteBishop] |
    rook_attacks   & board_status[BoardSlots::WhiteRook]   |
    queen_attacks  & board_status[BoardSlots::WhiteKing]   |
    king_attack    & board_status[BoardSlots::WhiteKing]) != EMPTY_BITBOARD

}
#[inline(always)]
pub fn is_square_attacked_white(board_status: &BoardStatus, square: Square) -> bool {
    let knight_attack  = generate_knight_attacks(square);
    let king_attack    = generate_king_attacks(square);
    let bishop_attacks = generate_bishop_attacks(square, board_status[BoardSlots::AllPieces]);
    let rook_attacks   = generate_rook_attakcs(square,   board_status[BoardSlots::AllPieces]);
    let queen_attacks  = bishop_attacks | rook_attacks;
    (genereate_pawn_attacks(square, Color::White) & board_status[BoardSlots::BlackPawn] |
    knight_attack  & board_status[BoardSlots::BlackKnight] |
    bishop_attacks & board_status[BoardSlots::BlackBishop] |
    rook_attacks   & board_status[BoardSlots::BlackRook]   |
    queen_attacks  & board_status[BoardSlots::BlackKing]   |
    king_attack    & board_status[BoardSlots::BlackKing]) != EMPTY_BITBOARD
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
    fn generate_pawn_moves(&mut self, board_status: &BoardStatus, mov_dir: Direction, 
        double_move_line: BitBoard, fnish_line: BitBoard, pawn: BoardSlots, enemy_color: Color, queen: BoardSlots, 
        rook: BoardSlots, bishop: BoardSlots, knight: BoardSlots, enemy_pieces: BoardSlots) {
        
        for square in board_status[pawn].clone() {
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
    #[inline(always)]
    fn generate_slider_moves(&mut self, gen_moves: fn(Square, BitBoard) -> BitBoard, board_status: &BoardStatus, 
        piece: BoardSlots, my_pieces: BoardSlots, enemy_pieces: BoardSlots) {
        for square in board_status[piece].clone() {
            let attacks = gen_moves(square, board_status[BoardSlots::AllPieces]) & !board_status[my_pieces];
            for attack in attacks {
                if board_status[enemy_pieces].is_square_set(attack) {
                    self.append_move(MoveBitField::new(piece, square, attack).set_capture())
                }
                else {
                    self.append_move(MoveBitField::new(piece, square, attack))
                }
            }
        }   
    }
    #[inline(always)]
    fn generate_non_sliding_moves(&mut self, gen_moves: fn(Square) -> BitBoard, board_status: &BoardStatus, piece: BoardSlots, my_pieces: BoardSlots, enemy_pieces: BoardSlots) {
        for square in board_status[piece].clone() {
            let attacks = gen_moves(square) & !board_status[my_pieces];
            for attack in attacks {
                if board_status[enemy_pieces].is_square_set(attack) {
                    self.append_move(MoveBitField::new(piece, square, attack).set_capture())
                }
                else {
                    self.append_move(MoveBitField::new(piece, square, attack))
                }
            }
        }   
    }
    #[inline(always)]
    fn generate_king_moves(&mut self, board_status: &BoardStatus, piece: BoardSlots, my_pieces: BoardSlots, enemy_pieces: BoardSlots,
    king_pos: Square, king_side_castle: CastleSlots, queen_side_castle: CastleSlots, is_square_attacked: fn(&BoardStatus, Square) -> bool) {
        
        self.generate_non_sliding_moves(generate_king_attacks, board_status, piece, my_pieces, enemy_pieces);
        let board = board_status[my_pieces];
        if !is_square_attacked(board_status, king_pos) {
            if board_status.can_castle(king_side_castle) && !is_square_attacked(board_status, king_pos + EAST) && !board.is_square_set(king_pos + EAST) && !board.is_square_set(king_pos + EAST * 2) {
                self.append_move(MoveBitField::new(piece, king_pos, king_pos + EAST * 2).set_castling());
            }
            if board_status.can_castle(queen_side_castle) && !is_square_attacked(board_status, king_pos + WEST) && !is_square_attacked(board_status, king_pos + WEST * 2) && 
            !board.is_square_set(king_pos + WEST) && !board.is_square_set(king_pos + WEST * 2) && !board.is_square_set(king_pos + WEST * 3) {
                self.append_move(MoveBitField::new(piece, king_pos, king_pos + WEST * 3).set_castling())
            }
        }

    }

    pub fn generate_moves(&mut self, board_status: &BoardStatus) {
        match board_status.color {
            Color::White => {
                self.generate_pawn_moves(&board_status, NORTH, RANK2, RANK8, BoardSlots::WhitePawn, Color::Black, BoardSlots::WhiteQueen, BoardSlots::WhiteRook, BoardSlots::WhiteBishop, BoardSlots::WhiteKnight, BoardSlots::BlackPieces);
                self.generate_non_sliding_moves(generate_knight_attacks, &board_status, BoardSlots::WhiteKnight, BoardSlots::WhitePieces, BoardSlots::BlackPieces);
                self.generate_slider_moves(generate_bishop_attacks, board_status, BoardSlots::WhiteBishop, BoardSlots::WhitePieces, BoardSlots::BlackPieces);
                self.generate_slider_moves(generate_rook_attakcs,   board_status, BoardSlots::WhiteRook,   BoardSlots::WhitePieces, BoardSlots::BlackPieces);
                self.generate_slider_moves(generate_queen_attacks,  board_status, BoardSlots::WhiteQueen,  BoardSlots::WhitePieces, BoardSlots::BlackPieces);
                self.generate_king_moves(board_status, BoardSlots::WhiteKing, BoardSlots::WhitePieces, BoardSlots::BlackPieces, E1, CastleSlots::WhiteKingSide, CastleSlots::WhiteQueenSide, is_square_attacked_white);
            }
            Color::Black => {
                self.generate_pawn_moves(&board_status, SOUTH, RANK7, RANK1, BoardSlots::BlackPawn, Color::White, BoardSlots::BlackQueen, BoardSlots::BlackRook, BoardSlots::BlackBishop, BoardSlots::BlackKnight, BoardSlots::WhitePieces);
                self.generate_non_sliding_moves(generate_knight_attacks, &board_status, BoardSlots::BlackKnight, BoardSlots::BlackPieces, BoardSlots::WhitePieces);
                self.generate_slider_moves(generate_bishop_attacks, board_status, BoardSlots::BlackBishop, BoardSlots::BlackPieces, BoardSlots::WhitePieces);
                self.generate_slider_moves(generate_rook_attakcs,   board_status, BoardSlots::BlackRook,   BoardSlots::BlackPieces, BoardSlots::WhitePieces);
                self.generate_slider_moves(generate_queen_attacks,  board_status, BoardSlots::BlackQueen,  BoardSlots::BlackPieces, BoardSlots::WhitePieces);
                self.generate_king_moves(board_status, BoardSlots::BlackKing, BoardSlots::BlackPieces, BoardSlots::WhitePieces, E8, CastleSlots::BlackKingSide, CastleSlots::BlackQueenSide, is_square_attacked_black);
            }
        }
        
    }
    
}

impl Index<usize> for MoveList {
    type Output = MoveBitField;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.moves[index].assume_init_ref() }
    }
    
}

