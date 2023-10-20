pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
use crate::pieces::helper_functions::initialize_magic_nums;

fn main() {
    initialize_magic_nums();
}
