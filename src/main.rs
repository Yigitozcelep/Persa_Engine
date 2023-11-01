pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;


use constants::squares::{A3, B2};

use crate::pieces::helper_functions::init_statics;
use crate::pieces::pieces_controller::{MoveBitField, MoveList, BoardSlots};

fn main() {
    init_statics();
    
}
