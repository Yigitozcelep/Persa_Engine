pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;


use debug::{FenString, debug_perft};
use crate::pieces::tables::init_statics;



fn main() {
    init_statics();
    debug_perft();
}
