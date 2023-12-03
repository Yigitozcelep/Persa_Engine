use crate::uci::UciInformation;
use crate::board_components::{BitBoard, Color, Square, Direction, ChessBoard};
use crate::constants::board_constants::ASCII_PIECES;
use crate::constants::eveluation_constants::MMV_LVA;
use crate::constants::directions::{NORTH, SOUTH, WEST, EAST};
use crate::constants::squares::*;
use crate::debug::FenString;
use crate::pieces::tables::*;
use crate::constants::board_constants::{EMPTY_BITBOARD, RANK1, RANK2, RANK7, RANK8};
use std::mem::{transmute, MaybeUninit};
use std::ops::{Index, IndexMut};


static CASTLING_RIGHTS: ChessBoard<u8> = ChessBoard::from([
   13, 15, 15, 15, 12, 15, 15, 14,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
    7, 15, 15, 15,  3, 15,  15, 11
]);

#[repr(u8)]
pub enum CastleSlots {
    WhiteKingSide  = 0b1,     // 1
    WhiteQueenSide = 0b10,    // 2
    BlackKingSide  = 0b100,   // 4
    BlackQueenSide = 0b1000,  // 8
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
    pub fn iterate_board_slots(start: BoardSlots, end: BoardSlots) -> impl Iterator<Item=BoardSlots> {
        unsafe {(start as usize..=end as usize).map(|num| transmute(num))}
    }
    pub fn iterate_color_pieces(color: Color) -> impl Iterator<Item = BoardSlots> {
        match color {
            Color::White => Self::iterate_board_slots(Self::WhitePawn, Self::WhiteKing),
            Color::Black => Self::iterate_board_slots(Self::BlackPawn, Self::BlackKing),
        }
    }

    #[inline(always)]
    pub fn iterate_pieces() -> impl Iterator<Item=BoardSlots> {
        BoardSlots::iterate_board_slots(BoardSlots::WhitePawn, BoardSlots::BlackKing)
    }
    pub fn iterate_all_slots() -> impl Iterator<Item=BoardSlots> {
        BoardSlots::iterate_board_slots(BoardSlots::WhitePawn, BoardSlots::AllPieces)
    }
}

impl Index<BoardSlots> for BoardStatus {
    type Output = BitBoard;
    fn index(&self, index: BoardSlots) -> &Self::Output {
        &self.boards[index as usize]
    }
}
impl IndexMut<BoardSlots> for BoardStatus {
    fn index_mut(&mut self, index: BoardSlots) -> &mut Self::Output {
        &mut self.boards[index as usize]
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
        self.color = unsafe {transmute(self.color as usize ^ 1)}
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
    pub fn remove_piece(&mut self, piece: BoardSlots, square: Square) {
        self[piece].toggle_bit(square);
        self.get_pieces_board(piece).toggle_bit(square);
        self[BoardSlots::AllPieces].toggle_bit(square)
    }

    #[inline(always)]
    pub fn get_other_color(&self) -> Color {
        match self.color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline(always)]
    pub fn make_move(&mut self, mov: MoveBitField) -> bool {
        let copy_data     = *self;
        let source_square = mov.get_source();
        let target_square = mov.get_target();
        let piece         = mov.get_piece();
        let promoted      = mov.get_promoted();
        
        self.enpassant = NO_SQUARE;
        self.half_move += 1;

        self.remove_piece(piece, source_square);
        if MoveBitField::is_move_promoted(promoted) { self.set_piece_bit(promoted, target_square);}
        else { self.set_piece_bit(piece, target_square); }
        
        if mov.is_move_enpassant() {
            match self.color {
                Color::White => self.remove_piece(BoardSlots::BlackPawn, target_square + SOUTH),
                Color::Black => self.remove_piece(BoardSlots::WhitePawn, target_square + NORTH),
            }
            
        }
        else if mov.is_move_capture() {
            let enemey_pieces = match self.color {
                Color::White => BoardSlots::iterate_board_slots(BoardSlots::BlackPawn, BoardSlots::BlackKing),
                Color::Black => BoardSlots::iterate_board_slots(BoardSlots::WhitePawn, BoardSlots::WhiteKing),
            };
            for enemy_piece in enemey_pieces {
                if !self[enemy_piece].is_square_set(target_square) {continue;}
                self[enemy_piece].toggle_bit(target_square);
                self.get_pieces_board(enemy_piece).toggle_bit(target_square);
                break;
            }
        }

        else if mov.is_move_double() {
            match self.color {
                Color::White => self.enpassant = target_square + SOUTH,
                Color::Black => self.enpassant = target_square + NORTH,
            }
        }
        else if mov.is_move_castling() {
            match target_square {
                G1 => { 
                    self.remove_piece(BoardSlots::WhiteRook, H1);
                    self.set_piece_bit(BoardSlots::WhiteRook, F1);
                },
                C1 => {
                    self.remove_piece(BoardSlots::WhiteRook, A1);
                    self.set_piece_bit(BoardSlots::WhiteRook, D1);
                },
                G8 => {
                    self.remove_piece(BoardSlots::BlackRook, H8);
                    self.set_piece_bit(BoardSlots::BlackRook, F8);
                },
                C8 => {
                    self.remove_piece(BoardSlots::BlackRook, A8);
                    self.set_piece_bit(BoardSlots::BlackRook, D8);
                },
                _  => unreachable!(),
            }
        }
        self.castles.0 &= CASTLING_RIGHTS[source_square];
        self.castles.0 &= CASTLING_RIGHTS[target_square];
        self.change_color();
        
        match self.color {
            Color::Black => {
                let square = self[BoardSlots::WhiteKing].get_lsb_index();
                
                if is_square_attacked_white(&self, square) {
                    *self = copy_data;
                    return false;
                }
                true
            }
            Color::White => {
                let square = self[BoardSlots::BlackKing].get_lsb_index();
                if is_square_attacked_black(&self, square) {
                    *self = copy_data;
                    return false;
                }
                true
            }
        }
    }
}

impl std::fmt::Display for BoardStatus {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", FenString::from_board(self).adjust_board_display())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct MoveBitField(u64);
impl MoveBitField {
    pub const NO_MOVE: MoveBitField = MoveBitField(0);

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

    pub fn set_score(&mut self, uci_info: &UciInformation, other_color: Color) {
        self.0 |= score_move(uci_info, *self, other_color) << 24;
    }
    #[inline(always)]
    pub fn get_score(&self) -> u64 { (self.0 >> 24) & 0xffff}

    #[inline(always)]
    pub fn get_source(&self) -> Square { Square((self.0 & 0x3f) as u8) }
    
    #[inline(always)]
    pub fn get_target(&self) -> Square { Square(((self.0 & 0xfc0) >> 6) as u8) }

    #[inline(always)]
    pub fn get_piece(&self) -> BoardSlots { unsafe { transmute( (self.0 & 0xf000) >> 12 ) } }
    
    #[inline(always)]
    pub fn get_promoted(&self) -> BoardSlots { unsafe { transmute( (self.0 & 0xf0000) >> 16 ) } }

    #[inline(always)]
    pub fn is_move_promoted(mov: BoardSlots) -> bool {mov != BoardSlots::WhitePawn}

    #[inline(always)]
    pub fn is_move_capture(&self) -> bool { (self.0 & 0x100000) != 0  }
    
    #[inline(always)]
    pub fn is_move_double(&self) -> bool { (self.0 & 0x200000) != 0 }

    #[inline(always)]
    pub fn is_move_enpassant(&self) -> bool { (self.0 & 0x400000) != 0 }

    #[inline(always)]
    pub fn is_move_castling(&self) -> bool { (self.0 & 0x800000) != 0 }

    pub fn get_move_name(&self) -> String {
        let mut key = format!("{}{}", self.get_source(), self.get_target());
        let promoted = self.get_promoted();
        if MoveBitField::is_move_promoted(promoted) { key += ASCII_PIECES[promoted as usize]; }
        key.to_lowercase()
    }

    pub fn convert_to_string(&self) -> String {
        let mut result = "".to_string();
        result += &self.get_move_name();
        result += &format!(" {:?} ", self.get_piece()).to_string();
        if self.get_promoted() != BoardSlots::WhitePawn {result += &format!("Promoted: {:?} ", self.get_promoted()).to_string()};
        if self.is_move_capture() {result += &format!("Capture ");}
        if self.is_move_double() {result += &format!("Double ");}
        if self.is_move_enpassant() {result += &format!("Enpassant ");}
        if self.is_move_castling() {result += &format!("Castling ")};
        result
    }

    
}

impl std::fmt::Display for MoveBitField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.convert_to_string())
    }
}

#[derive(Clone, Copy)]
pub struct MoveList {
    moves: [MaybeUninit<MoveBitField>; 256],
    pub count: usize,
}

impl std::fmt::Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut result = "".to_string();
        for index in 0..self.count {
            result += &self[index].convert_to_string();
            result += "\n";
        }
        result += &format!("\nTotal Move: {}", self.count);
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
    queen_attacks  & board_status[BoardSlots::WhiteQueen]  |
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
    queen_attacks  & board_status[BoardSlots::BlackQueen]  |
    king_attack    & board_status[BoardSlots::BlackKing]) != EMPTY_BITBOARD
}

impl MoveList {
    #[inline(always)]
    pub fn new(uci_info: &UciInformation) -> Self {
        let mut res =  unsafe { Self { moves: MaybeUninit::uninit().assume_init(), count: 0, }};
        let board_status = &uci_info.board;
        match board_status.get_color() {
            Color::White => {
                res.generate_pawn_moves(&board_status, NORTH, RANK2, RANK8, BoardSlots::WhitePawn, Color::Black, BoardSlots::WhiteQueen, BoardSlots::WhiteRook, BoardSlots::WhiteBishop, BoardSlots::WhiteKnight, BoardSlots::BlackPieces);
                res.generate_non_sliding_moves(generate_knight_attacks, &board_status, BoardSlots::WhiteKnight, BoardSlots::WhitePieces, BoardSlots::BlackPieces);
                res.generate_slider_moves(generate_bishop_attacks, board_status, BoardSlots::WhiteBishop, BoardSlots::WhitePieces, BoardSlots::BlackPieces);
                res.generate_slider_moves(generate_rook_attakcs,   board_status, BoardSlots::WhiteRook,   BoardSlots::WhitePieces, BoardSlots::BlackPieces);
                res.generate_slider_moves(generate_queen_attacks,  board_status, BoardSlots::WhiteQueen,  BoardSlots::WhitePieces, BoardSlots::BlackPieces);
                res.generate_king_moves(board_status, BoardSlots::WhiteKing, BoardSlots::WhitePieces, BoardSlots::BlackPieces, E1, CastleSlots::WhiteKingSide, CastleSlots::WhiteQueenSide, is_square_attacked_white);
            }
            Color::Black => {
                res.generate_pawn_moves(&board_status, SOUTH, RANK7, RANK1, BoardSlots::BlackPawn, Color::White, BoardSlots::BlackQueen, BoardSlots::BlackRook, BoardSlots::BlackBishop, BoardSlots::BlackKnight, BoardSlots::WhitePieces);
                res.generate_non_sliding_moves(generate_knight_attacks, &board_status, BoardSlots::BlackKnight, BoardSlots::BlackPieces, BoardSlots::WhitePieces);
                res.generate_slider_moves(generate_bishop_attacks, board_status, BoardSlots::BlackBishop, BoardSlots::BlackPieces, BoardSlots::WhitePieces);
                res.generate_slider_moves(generate_rook_attakcs,   board_status, BoardSlots::BlackRook,   BoardSlots::BlackPieces, BoardSlots::WhitePieces);
                res.generate_slider_moves(generate_queen_attacks,  board_status, BoardSlots::BlackQueen,  BoardSlots::BlackPieces, BoardSlots::WhitePieces);
                res.generate_king_moves(board_status, BoardSlots::BlackKing, BoardSlots::BlackPieces, BoardSlots::WhitePieces, E8, CastleSlots::BlackKingSide, CastleSlots::BlackQueenSide, is_square_attacked_black);
            }
        }
        let other_color = board_status.get_other_color();
        unsafe {
            res.moves[0..res.count].iter_mut().for_each(|mov| {
                mov.assume_init_mut().set_score(uci_info, other_color);
            });
            res.moves[0..res.count].sort_by(|mov1, mov2| mov2.assume_init().get_score().cmp(&mov1.assume_init().get_score()));
        }
        res
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
        
        for square in board_status[pawn] {
            let target = square + mov_dir;
            if !board_status[BoardSlots::AllPieces].is_square_set(target) {
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
            if board_status[pawn].is_square_set(board_status.enpassant + mov_dir) {return;}
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
        let board = board_status[BoardSlots::AllPieces];
        if !is_square_attacked(board_status, king_pos) {
            if board_status.can_castle(king_side_castle) && !is_square_attacked(board_status, king_pos + EAST) && !board.is_square_set(king_pos + EAST) && !board.is_square_set(king_pos + EAST * 2) {
                self.append_move(MoveBitField::new(piece, king_pos, king_pos + EAST * 2).set_castling());
            }
            if board_status.can_castle(queen_side_castle) && !is_square_attacked(board_status, king_pos + WEST) && 
            !board.is_square_set(king_pos + WEST) && !board.is_square_set(king_pos + WEST * 2) && !board.is_square_set(king_pos + WEST * 3) {
                self.append_move(MoveBitField::new(piece, king_pos, king_pos + WEST * 2).set_castling())
            }
        }

    }

    pub fn iterate_moves<'a>(&'a self) -> impl Iterator<Item = MoveBitField> + 'a {
        (0..self.count).map(|i| self[i])
    }
}

impl Index<usize> for MoveList {
    type Output = MoveBitField;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.moves[index].assume_init_ref() }
    }
    
}


#[inline(always)]
fn score_move(uci_info: &UciInformation, mov: MoveBitField, enemy_color: Color) -> u64 {
    let current_piece = mov.get_piece();
    let target_square = mov.get_target();
    if mov.is_move_capture() { 
        for enemy_piece in BoardSlots::iterate_color_pieces(enemy_color) {
            if uci_info.board[enemy_piece].is_square_set(target_square) {
                return MMV_LVA[(current_piece, enemy_piece)];
            }
        }
    }
    if mov == uci_info.board_history.killer_moves[uci_info.board.half_move][0] { return 9000; }
    if mov == uci_info.board_history.killer_moves[uci_info.board.half_move][1] { return 8000; }
    return 0;
}

pub struct BoardHistory {
    pub killer_moves    :   [[MoveBitField; 2]; 64],
    pub found_best_move :   MoveBitField,
}

impl BoardHistory {
    pub fn new () -> Self {
        Self {killer_moves: [[MoveBitField::NO_MOVE; 2]; 64], found_best_move: MoveBitField::NO_MOVE}
    }
    
    pub fn append_killer_move(&mut self, mov: MoveBitField, half_move: usize) {
        self.killer_moves[half_move][1] = self.killer_moves[half_move][0];
        self.killer_moves[half_move][0] = mov;
    }

    pub fn add_new_best_move(&mut self, mov: MoveBitField) { self.found_best_move = mov; }
}