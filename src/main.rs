pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;


use board_components::Square;
use constants::squares::*;
use debug::FenString;

use crate::pieces::helper_functions::init_statics;

fn main() {
    init_statics();
    let fen = FenString::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R".to_string());
    let data = fen.convert_to_board();
    let res = data.is_square_attacked(board_components::Color::White, A5);
    println!("{}", fen);
    println!("{}", res);
}
