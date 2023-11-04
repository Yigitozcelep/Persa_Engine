pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;


use debug::FenString;

use crate::pieces::tables::init_statics;
use crate::pieces::pieces_controller::{BoardSlots, BoardStatus, MoveBitField, MoveList};

fn main() {
    init_statics();
    let fen = FenString::new("r3k2r/pb2bppp/1p21n2/3p1B2/3P4/2N2N2/PPPQPPPP/R3K2R w KQkq - 0 1".to_string());
    let mut board = fen.convert_to_board();
    board.change_color();
    let mut move_list = MoveList::new();
    move_list.generate_moves(&board);
    
    println!("{}", board);
    println!("{}", move_list);
    let mut board = board.make_move(move_list[30]);
    println!("{}", board);
}
