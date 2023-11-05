use crate::constants::directions::*;
use crate::constants::board_constants::*;
use crate::board_components::{BitBoard, Square, ChessBoard, MagicNum, Color, MagicNumGenerator};
use crate::constants::board_constants::{BISHOP_MAX_BLOCK_PERM, ROOK_MAX_BLOCK_PERM, create_bishop_move_counts, create_bishop_magics, create_rook_move_counts, create_rook_magics};
use std::cmp::{min, max};
use std::sync::Once;

static mut BISHOP_TABLE: ChessBoard<[BitBoard; BISHOP_MAX_BLOCK_PERM]> = ChessBoard::from([[BitBoard::new(); BISHOP_MAX_BLOCK_PERM]; 64]);
static mut BISHOP_ATTACKS: ChessBoard<BitBoard> = ChessBoard::from([BitBoard::new(); 64]);
static BISHOP_MOVES_COUNTS: ChessBoard<u64> = create_bishop_move_counts();
static BISHOP_MAGICS: ChessBoard<MagicNum> = create_bishop_magics();

static mut ROOK_TABLE: ChessBoard<[BitBoard; ROOK_MAX_BLOCK_PERM]> = ChessBoard::from([[BitBoard::new(); ROOK_MAX_BLOCK_PERM]; 64]);
static mut ROOK_ATTACKS: ChessBoard<BitBoard> = ChessBoard::from([BitBoard::new(); 64]);
static ROOK_MOVES_COUNTS: ChessBoard<u64> = create_rook_move_counts();
static ROOK_MAGICS: ChessBoard<MagicNum> = create_rook_magics();

static mut KING_TABLE: ChessBoard<BitBoard> = ChessBoard::from([BitBoard::new(); 64]);
static mut KNIGHT_TABLE: ChessBoard<BitBoard> = ChessBoard::from([BitBoard::new(); 64]);

static mut PAWN_TABLE: [ChessBoard<BitBoard>; 2] = [ChessBoard::from([BitBoard::new(); 64]), ChessBoard::from([BitBoard::new(); 64])];

static INIT: Once = Once::new();

pub fn init_statics() {
    INIT.call_once( || {
        initialize_bishop_components();
        initialize_rook_components();
        initialize_pawn_table();
        initialize_knight_table();
        initialize_king_table();
    });
}

fn get_possible_occupancy(bitboard: BitBoard, index: u64) -> BitBoard {
    let mut occupancy =  BitBoard::new();
    
    for (count, square) in bitboard.enumerate() {
        if (index & 1 << count) != 0 {
            occupancy.set_bit(square);
        }
    }
    return occupancy;
}

pub fn find_magic_number(mask_attacks: fn(Square) -> BitBoard, attack_on_fly: fn(Square, BitBoard) -> BitBoard, square: Square) -> MagicNum {
    let attack_mask = mask_attacks(square);
    let move_count = attack_mask.count_ones();
    let mut occupancies:  [BitBoard; 4096] = [BitBoard::new(); 4096];
    let mut used_attacks: [BitBoard; 4096] = [BitBoard::new(); 4096];
    let mut attacks:      [BitBoard; 4096] = [BitBoard::new(); 4096];
    
    let mut magic_num_generator = MagicNumGenerator::new();

    let total_mask_pos: usize = (2_usize).pow(move_count as u32);
    (0..total_mask_pos).for_each(|index| {
        occupancies[index] = get_possible_occupancy(attack_mask, index as u64);
        attacks[index] = attack_on_fly(square, occupancies[index]);
    });

    'start: for _ in 0..1000000000 {
        let magic_num = magic_num_generator.gen();
        if ((attack_mask * magic_num) & 0xFF00000000000000).count_ones() < 6 {continue;}
        for el in used_attacks.iter_mut() {*el = BitBoard::new();}
        for index in 0..total_mask_pos {
            let magic_index = ((occupancies[index] * magic_num) >> (64 - move_count)) as usize;
            
            if used_attacks[magic_index] == EMPTY_BITBOARD { used_attacks[magic_index] = attacks[index];}
            else if used_attacks[magic_index] != attacks[index] {continue 'start;}
        }
        return magic_num;
    }
    
  unreachable!()
}

fn initialize_slider_table<const LEN: usize>(table: &mut ChessBoard<[BitBoard; LEN]>,  magics: &ChessBoard<MagicNum>, mask_attacks: fn(Square) -> BitBoard, attack_on_fly: fn(Square, BitBoard) -> BitBoard) {
    for square in Square::create_squares(0, 64) {
        let attack = mask_attacks(square);
        let move_count = attack.count_ones();
        let total_mask_pos = 2_u64.pow(move_count);
        for index in 0..total_mask_pos {
            let occupancy = get_possible_occupancy(attack, index);
            let magic_index = (occupancy * magics[square]) >> (64 - move_count);
            table[square][magic_index as usize] = attack_on_fly(square, occupancy);
        }
    }
}

fn initialize_slider_attacks(mask_attacks: fn(Square) -> BitBoard, attacks: &mut ChessBoard<BitBoard>) {
    for square in Square::create_squares(0, 64) {attacks[square] = mask_attacks(square);}
}

#[inline(always)]
fn generate_slider_moves<const LEN: usize> (square: Square, board: BitBoard, attacks: &ChessBoard<BitBoard>, 
    magics: &ChessBoard<MagicNum>, table: &ChessBoard<[BitBoard; LEN]>, move_counts: &ChessBoard<u64>) -> BitBoard {
    let occupancy = board & attacks[square];
    let magic_index = (occupancy * magics[square]) >> (64 - move_counts[square]);
    table[square][magic_index as usize]
}

fn mask_bishop_attacks(square: Square) -> BitBoard {
    bishop_attacks_on_fly(square, BitBoard::new()) & !EDGES
}

fn bishop_attacks_on_fly(square: Square, blocker: BitBoard) -> BitBoard {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = BitBoard::new();

    for i in 1..8 - max(rank, file) {
        attacks.set_bit(square + NORTH_EAST * i);
        if blocker.is_square_set(square + NORTH_EAST * i) {break;}
    }
    for i in 1..min(rank, file) + 1 {
        attacks.set_bit(square + SOUTH_WEST * i);
        if blocker.is_square_set(square + SOUTH_WEST * i) {break;}
    }

    for i in 1..min(8 - rank, file + 1) {
        attacks.set_bit(square + NORTH_WEST * i);
        if blocker.is_square_set(square + NORTH_WEST * i) {break;}
    }
    
    for i in 1..min(rank + 1, 8 - file) {
        attacks.set_bit(square + SOUTH_EAST * i);
        if blocker.is_square_set(square + SOUTH_EAST * i) {break;}
    }

    attacks
}

#[inline(always)]
pub fn generate_bishop_attacks(square: Square, board: BitBoard) -> BitBoard {
    unsafe {generate_slider_moves(square, board, &BISHOP_ATTACKS, &BISHOP_MAGICS, &BISHOP_TABLE, &BISHOP_MOVES_COUNTS)}
}

fn initialize_bishop_components() {
    unsafe {initialize_slider_table(&mut BISHOP_TABLE, &BISHOP_MAGICS, mask_bishop_attacks, bishop_attacks_on_fly);}
    unsafe {initialize_slider_attacks(mask_bishop_attacks, &mut BISHOP_ATTACKS)}
}

fn initialize_king_table() {
    for square in Square::create_squares(0, 64) {
        unsafe {KING_TABLE[square] = mask_king_attacks(square);}
    }
}

fn mask_king_attacks(square: Square) -> BitBoard {
    let mut attack = BitBoard::new();
    if !RANK8.is_square_set(square)            {attack.set_bit(square + NORTH);}
    if !(RANK8 | H_FILE).is_square_set(square) {attack.set_bit(square + NORTH_EAST);}
    if !H_FILE.is_square_set(square)           {attack.set_bit(square + EAST);}
    if !(RANK1 | H_FILE).is_square_set(square) {attack.set_bit(square + SOUTH_EAST);}
    if !RANK1.is_square_set(square)            {attack.set_bit(square + SOUTH);}
    if !(RANK1 | A_FILE).is_square_set(square) {attack.set_bit(square + SOUTH_WEST);}
    if !A_FILE.is_square_set(square)           {attack.set_bit(square + WEST);}
    if !(RANK8 | A_FILE).is_square_set(square) {attack.set_bit(square + NORTH_WEST);}
    
    attack
}

#[inline(always)]
pub fn generate_king_attacks(square: Square) -> BitBoard{
    unsafe {KING_TABLE[square]}
}

fn initialize_knight_table() {
    for square in Square::create_squares(0, 64) {
        unsafe {KNIGHT_TABLE[square] = mask_knight_attacks(square);}
    }
}

fn mask_knight_attacks(square: Square) -> BitBoard {
    let mut attack = BitBoard::new();

    if !(TOP_2_RANK    | A_FILE).is_square_set(square) {attack.set_bit(square + NORTH * 2 + WEST);}
    if !(TOP_2_RANK    | H_FILE).is_square_set(square) {attack.set_bit(square + NORTH * 2 + EAST);}
    if !(RIGHT_2_FILE  | RANK8 ).is_square_set(square) {attack.set_bit(square + EAST * 2 + NORTH);}
    if !(RIGHT_2_FILE  | RANK1 ).is_square_set(square) {attack.set_bit(square + EAST * 2 + SOUTH);}
    if !(BOTTOM_2_RANK | H_FILE).is_square_set(square) {attack.set_bit(square + SOUTH * 2 + EAST);}
    if !(BOTTOM_2_RANK | A_FILE).is_square_set(square) {attack.set_bit(square + SOUTH * 2 + WEST);}
    if !(LEFT_2_FILE   | RANK1 ).is_square_set(square) {attack.set_bit(square + WEST * 2 + SOUTH);}
    if !(LEFT_2_FILE   | RANK8 ).is_square_set(square) {attack.set_bit(square + WEST * 2 + NORTH);}

    attack
}

pub fn generate_knight_attacks(square: Square) -> BitBoard {
    unsafe {KNIGHT_TABLE[square]}
}

fn initialize_pawn_table() {
    for square in Square::create_squares(0, 64) {
        unsafe {
            PAWN_TABLE[Color::White as usize][square] = mask_pawn_attacks(Color::White, square);
            PAWN_TABLE[Color::Black as usize][square] = mask_pawn_attacks(Color::Black, square);
        }
    }
}

fn mask_pawn_attacks(side: Color, square: Square) -> BitBoard {
    let mut attack = BitBoard::new();
    
    match side {
        Color::White => {
            if RANK8.is_square_set(square)   {return attack;}
            if !A_FILE.is_square_set(square) {attack.set_bit(square + NORTH_WEST);}
            if !H_FILE.is_square_set(square) {attack.set_bit(square + NORTH_EAST);}
        }
        Color::Black => {
            if RANK1.is_square_set(square)   {return attack;}
            if !A_FILE.is_square_set(square) {attack.set_bit(square + SOUTH_WEST);}
            if !H_FILE.is_square_set(square) {attack.set_bit(square + SOUTH_EAST);}
        }
    }
    return attack;
}

fn mask_rook_attacks(square: Square) -> BitBoard {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = BitBoard::new();

    for i in 1..7 - file {attacks.set_bit(square + EAST * i);}

    for i in 1..file {attacks.set_bit(square + WEST * i);}
    
    for i in 1..7 - rank {attacks.set_bit(square + NORTH * i);}

    for i in 1..rank {attacks.set_bit(square + SOUTH * i);}
    attacks
}

fn rook_attacks_on_fly(square: Square, blockers: BitBoard) -> BitBoard {
    let rank = square.get_rank();
    let file = square.get_file();
    let mut attacks = BitBoard::new();
    for i in 1..8 - file {
        attacks.set_bit(square + EAST * i);
        if blockers.is_square_set(square + EAST * i) {break;}
    }
    for i in 1..file + 1{
        attacks.set_bit(square + WEST * i);
        if blockers.is_square_set(square + WEST * i) {break;}
    }
    
    for i in 1..8 - rank {
        attacks.set_bit(square + NORTH * i);
        if blockers.is_square_set(square + NORTH * i) {break;}
    }

    for i in 1..rank + 1 {
        attacks.set_bit(square + SOUTH * i);
        if blockers.is_square_set(square + SOUTH * i) {break;}
    }
    attacks
}



#[inline(always)]
pub fn generate_rook_attakcs(square: Square, board: BitBoard) -> BitBoard{
    unsafe {generate_slider_moves(square, board, &ROOK_ATTACKS, &ROOK_MAGICS, &ROOK_TABLE, &ROOK_MOVES_COUNTS)}
}

fn initialize_rook_components() {
    unsafe {initialize_slider_table(&mut ROOK_TABLE, &ROOK_MAGICS, mask_rook_attacks, rook_attacks_on_fly);}
    unsafe {initialize_slider_attacks(mask_rook_attacks, &mut ROOK_ATTACKS)}
}



#[inline(always)]
pub fn genereate_pawn_attacks(square: Square, side: Color) -> BitBoard {
    unsafe {PAWN_TABLE[side as usize][square]}
}

pub fn generate_queen_attacks(square: Square, board: BitBoard) -> BitBoard {
    generate_rook_attakcs(square, board) | generate_bishop_attacks(square, board)
}