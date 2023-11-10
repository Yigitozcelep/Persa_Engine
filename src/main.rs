pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;
pub mod eveluation;

use debug::FenString;

use crate::pieces::tables::init_statics;
use crate::debug::perft_diff_terminal;
use crate::eveluation::eveluate;
use crate::pieces::pieces_controller::BoardSlots;
fn main() {
    init_statics();
    let fen = FenString::new("rnbqkbnr/pppppppp/8/8/1P6/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1 ".to_string());
    let board = fen.convert_to_board();
    let res = eveluate(&board);
    println!("{}", board);
    println!("{}", res);
}
