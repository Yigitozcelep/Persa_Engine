pub mod constants;
pub mod debug;
pub mod helper_functions;
pub mod helper_macros;
pub mod tables;

use tables::rook_table::magic_mask_rook_attakcs;

use crate::{constants::board_constants::{EDGES, CORNERS}, debug::get_square_name};

fn main() {
    for rank in 0..8 {
        for file in 0..8 {
            print!("{}, ", magic_mask_rook_attakcs(rank * 8 + file).count_ones());
        }
        println!("");
    }
}
