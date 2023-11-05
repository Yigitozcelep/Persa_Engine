pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;


use debug::{FenString, perft_diff_manuel, perft_driver, perft_diff_terminal};
use crate::pieces::tables::init_statics;
use crate::pieces::tables::{genereate_pawn_attacks};
use crate::pieces::pieces_controller::{MoveList, is_square_attacked_white, BoardSlots};
use constants::squares::*;

fn main() {
    init_statics();
    perft_diff_terminal();
}
