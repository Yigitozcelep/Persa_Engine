use crate::board_components::{Board, Square, MagicNumGenerator, MagicNum, create_rook_move_counts, create_bishop_move_counts};
use crate::constants::board_constants::EMPTY_BOARD;
use crate::pieces::rook::rook_table::{rook_attacks_on_fly, mask_rook_attacks};
use super::bishop::bishop_table::{mask_bishop_attacks, bishop_attacks_on_fly};

pub fn get_possible_occupancy(board: Board, index: Board) -> Board {
    let mut occupancy =  Board::new();
    
    for (count, square) in board.enumerate() {
        if index.is_square_set(Square(count as u8)) {
            occupancy.set_bit(square);
        }
    }
    return occupancy;
}

pub fn find_magic_number(mask_attacks: fn(Square) -> Board, attack_on_fly: fn(Square, Board) -> Board, square: Square, move_count: u8) -> MagicNum {

    let attack_mask = mask_attacks(square);

    let mut occupancies:  [Board; 4096] = [Board::new(); 4096];
    let mut used_attacks: [Board; 4096] = [Board::new(); 4096];
    let mut attacks:      [Board; 4096] = [Board::new(); 4096];
    
    let mut magic_num_generator = MagicNumGenerator::new();

    let total_mask_pos: usize = (2_usize).pow(move_count as u32);
    (0..total_mask_pos).for_each(|index| {
        occupancies[index] = get_possible_occupancy(attack_mask, Board(index as u64));
        attacks[index] = attack_on_fly(square, occupancies[index]);
    });



    'start: for _ in 0..1000000000 {
        let magic_num = magic_num_generator.gen();
        if ((attack_mask * magic_num) & 0xFF00000000000000).count_ones() < 6 {continue;}
        for el in used_attacks.iter_mut() {*el = Board::new();}
        for index in 0..total_mask_pos {
            let magic_index = ((occupancies[index] * magic_num) >> (64 - move_count)) as usize;
            
            if used_attacks[magic_index] == EMPTY_BOARD {used_attacks[magic_index] = attacks[index]}
            else if used_attacks[magic_index] != attacks[index] {continue 'start;}
        }
        return magic_num;
    }
    
  unreachable!()
}

pub fn initialize_magic_nums() {
    let mut rook_magics: [MagicNum; 64] = [MagicNum(0); 64];
    let rook_counts = create_rook_move_counts();
    for square in Square::create_squares(0, 64) {
        let res = find_magic_number(mask_rook_attacks, rook_attacks_on_fly, square, rook_counts[square]);
        
    }
}