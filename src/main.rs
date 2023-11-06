pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;


use crate::pieces::tables::init_statics;

fn main() {
    init_statics();

}
