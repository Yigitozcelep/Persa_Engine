pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;


use debug::FenString;

use crate::pieces::pieces_controller::MoveList;
use crate::pieces::tables::init_statics;

fn main() {
    init_statics();
    
    let data = FenString::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string());
    let mut res = data.convert_to_board();
    res.change_color();
    let mut move_list = MoveList::new();
    println!("{}", res);
    move_list.generate_moves(&res);
    println!("{}", move_list);
    
}