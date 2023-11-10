use crate::pieces::pieces_controller::{BoardSlots, BoardStatus, MoveList};
use crate::board_components::Color;
use crate::constants::eveluation_constants::MATERIAL_SCORES;

pub fn eveluate(board_status: &BoardStatus) -> isize {
    let mut score: isize = 0;
    
    for square in board_status[BoardSlots::WhitePawn]   {score += MATERIAL_SCORES.white_pawn_score[square] }   
    for square in board_status[BoardSlots::WhiteBishop] {score += MATERIAL_SCORES.white_bishop_score[square]}   
    for square in board_status[BoardSlots::WhiteKing]   {score += MATERIAL_SCORES.white_king_score[square]}   
    for square in board_status[BoardSlots::WhiteKnight] {score += MATERIAL_SCORES.white_knight_score[square]}   

    for square in board_status[BoardSlots::BlackPawn]   {score -= MATERIAL_SCORES.black_pawn_score[square]}   
    for square in board_status[BoardSlots::BlackBishop] {score -= MATERIAL_SCORES.black_bishop_score[square]}   
    for square in board_status[BoardSlots::BlackKing]   {score -= MATERIAL_SCORES.black_king_score[square]}   
    for square in board_status[BoardSlots::BlackKnight] {score -= MATERIAL_SCORES.black_knight_score[square]}   
    match  board_status.get_color() {                                                                       
        Color::White => score,                                                                              
        Color::Black => -score,                                                                             
    }                                                                                                       
}

pub fn negamax(mut alpha: isize, mut beta: isize, depth: usize , board_status: &BoardStatus) -> isize {
    if depth == 0 {return eveluate(board_status)}

    let moves = MoveList::new(board_status);
    for mov in moves.iterate_moves() {
        let board = board_status.clone().make_move(mov);
        
    }

    return 2;
}


pub fn minimax(board_status: BoardStatus, depth: usize, mut alpha: isize, mut beta: isize) -> isize {
    match board_status.get_color() {
        Color::White => {
            let mut max_eval = isize::MIN;
            let moves = MoveList::new(&board_status);
            for mov in moves.iterate_moves() {
                let mut board = board_status;
                if !board.make_move(mov) {continue;}
                
                let eval = minimax(board, depth -1, alpha, beta);
                max_eval = isize::max(eval, max_eval);
                alpha = isize::max(alpha, eval);
                if beta <= alpha {break;}
            }
            return max_eval;
        },
        Color::Black => (),
    }

    5
}
                                                                                                            
                                                                                                            
                                                                                                            
                                                                                                            