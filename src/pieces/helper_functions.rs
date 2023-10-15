use crate::board_components::{Board, Direction, Square};

// boundries included
pub fn mask_direction(square: Square, direction: Direction, boundry: Board) -> Board{
    let mut attack = Board::new();

    for i in 0..8 {
        let target = square + direction * i;
        attack.set_bit(target);
        if boundry.is_square_set(target) {break;}
    }
    attack.toggle_bit(square);
    attack
}

pub fn get_possible_occupancy(board: Board, index: Board) -> Board {
    let mut occupancy =  Board::new();
    
    for (count, square) in board.enumerate() {
        if index.is_square_set(Square(count as u8)) {
            occupancy.set_bit(square);
        }
    }
    return occupancy;
}


pub fn find_magic_number(square: u64, relevant_bits: u64, bishop: u64) {
    let mut occupancies:  [u64; 4096] = [0; 4096];
    let mut attacks:      [u64; 4096] = [0; 4096];
    let mut used_attacks: [u64; 4096] = [0; 4096];
}
