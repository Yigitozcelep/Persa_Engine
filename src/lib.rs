pub mod constants;
pub mod board_components;
pub mod pieces;
pub mod helper_macros;
pub mod debug;
pub mod eveluation;
pub mod uci;

use debug::FenString;
use pieces::tables::init_statics;
use crate::pieces::pieces_controller::{MoveList, is_square_attacked_black, is_square_attacked_white, BoardSlots};
use board_components::Color;
use uci::UciInformation;

pub fn init_all_statics() { init_statics(); }

pub fn get_moves(fen: String) -> Vec<String> {
    let board = FenString::new(fen.clone()).convert_to_board();
    let moves = MoveList::new(&UciInformation::new().set_board(board));
    moves.iterate_moves().filter(|mov| {
        let mut dummy = FenString::new(fen.clone()).convert_to_board();
        dummy.make_move(*mov)
    }).map(|mov| mov.get_move_name()).collect()
}

pub fn make_move(fen: String, move_name: String) -> String {
    let mut board = FenString::new(fen).convert_to_board();
    let moves = MoveList::new(&UciInformation::new().set_board(board));
    let mov = moves.iterate_moves().find(|mov| mov.get_move_name() == move_name).unwrap();
    board.make_move(mov);
    FenString::from_board(&board).get_fen_string()
}

pub fn is_king_attacked(fen: String) -> bool {
    let board = FenString::new(fen).convert_to_board();
    match board.get_color() {
        Color::White => {
            let square = board[BoardSlots::WhiteKing].get_lsb_index();
            is_square_attacked_white(&board, square)
        }
        Color::Black => {
            let square = board[BoardSlots::BlackKing].get_lsb_index();
            is_square_attacked_black(&board, square)
        }
    }
}