pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;


use crate::{board_components::{Color, Square}, pieces::{helper_functions::init_statics, pawn::pawn_table::PAWN_TABLE}};

fn main() {
    init_statics();
    
}
