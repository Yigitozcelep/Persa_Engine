pub mod pieces;
pub mod board_components;
pub mod constants;
pub mod debug;
pub mod eveluation;
pub mod helper_macros;

use pieces::tables::init_statics;

fn uci_loop() {
    println!("id name Persa");
    println!("")
}


fn main() {
    init_statics();
    let fen = debug::FenString::new("r1bqkbnr/pppp2Pp/8/8/1n2P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 5".to_string());
    let board = fen.convert_to_board();
    let move_list = pieces::pieces_controller::MoveList::new(&board);
    println!("{}", move_list);

}   