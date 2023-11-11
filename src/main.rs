pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;
pub mod eveluation;

use debug::FenString;
use eveluation::minimax;

use crate::pieces::tables::init_statics;

fn main() {
    init_statics();
    let fen = FenString::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1 ".to_string());
    let board = fen.convert_to_board();
    let res = minimax(board, 4);
    println!("{} {}", res.0, res.1);
}
