use crate::constants::board_constants::WHITE;

pub mod constants;
pub mod debug;
pub mod helper_functions;
pub mod tables;
pub mod helper_macros;


fn main() {
    let pawn_table = tables::pawn_table::create_pawn_table();
    for square in 0..64 {
        println!("------------------------------------------\nsqaure: {}", debug::get_square_name(square));
        debug::print_bit_board(pawn_table[WHITE][square]);
    }
    
}
