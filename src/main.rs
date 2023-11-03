pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;


use debug::FenString;
use crate::pieces::pieces_controller::{MoveList, CastleSlots};
use crate::pieces::tables::init_statics;

fn main() {
    init_statics();
    
    let data = FenString::new("r2q1rk1/1b1nbppp/p2ppn2/1pp5/3PPP2/2NBBN2/PPPQ2PP/R4RK1 w - - 0 15".to_string());
    let board_status = data.convert_to_board();
    let mut move_list = MoveList::new();
    println!("{}", board_status);
    move_list.generate_moves(&board_status);
    println!("{}", move_list);
    println!("Total_move: {}", move_list.count)
}