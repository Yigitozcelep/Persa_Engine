use crate::pieces::pieces_controller::{BoardSlots, BoardStatus, MoveList, MoveBitField};
use crate::board_components::Color;
use crate::constants::eveluation_constants::MATERIAL_SCORES;
use crate::pieces::pieces_controller::{is_square_attacked_black, is_square_attacked_white};

pub fn eveluate(board_status: &BoardStatus) -> isize {
    let mut score: isize = 0;
    
    for square in board_status[BoardSlots::WhitePawn]   {score += MATERIAL_SCORES.white_pawn_square_score[square];   score += MATERIAL_SCORES.pawn_score }   
    for square in board_status[BoardSlots::WhiteBishop] {score += MATERIAL_SCORES.white_bishop_square_score[square]; score += MATERIAL_SCORES.bishop_score;}   
    for square in board_status[BoardSlots::WhiteKing]   {score += MATERIAL_SCORES.white_king_square_score[square];}   
    for square in board_status[BoardSlots::WhiteKnight] {score += MATERIAL_SCORES.white_knight_square_score[square]; score += MATERIAL_SCORES.knight_score;}   
    for square in board_status[BoardSlots::WhiteRook]   {score += MATERIAL_SCORES.white_rook_square_score[square];   score += MATERIAL_SCORES.rook_score;}
    for _ in board_status[BoardSlots::WhiteQueen]       {score += MATERIAL_SCORES.queen_score;}

    for square in board_status[BoardSlots::BlackPawn]   {score += MATERIAL_SCORES.black_pawn_square_score[square];   score -= MATERIAL_SCORES.pawn_score;}   
    for square in board_status[BoardSlots::BlackBishop] {score += MATERIAL_SCORES.black_bishop_square_score[square]; score -= MATERIAL_SCORES.bishop_score;}   
    for square in board_status[BoardSlots::BlackKing]   {score += MATERIAL_SCORES.black_king_square_score[square];}   
    for square in board_status[BoardSlots::BlackKnight] {score += MATERIAL_SCORES.black_knight_square_score[square]; score -= MATERIAL_SCORES.knight_score;}   
    for square in board_status[BoardSlots::BlackRook]   {score += MATERIAL_SCORES.black_rook_square_score[square];   score -= MATERIAL_SCORES.rook_score;}
    for _ in board_status[BoardSlots::BlackQueen]       {score -= MATERIAL_SCORES.queen_score;}
    
    match board_status.get_color() {
        Color::White => score,
        Color::Black => -score,
    }
}

pub fn find_best_move(board_status: BoardStatus, depth: isize) -> (MoveBitField, isize) {
    let move_list = MoveList::new(&board_status);
    let mut best_move = MoveBitField::NO_MOVE;
    let beta = 1000000;
    let mut alpha = -1000000;
    for mov in move_list.iterate_moves() {
        let mut board = board_status;
        if !board.make_move(mov) {continue;}
        let score = -negamax(board, -alpha, -beta, depth -1);
        if score > alpha {
            alpha = score;
            best_move = mov;
        }
    }
    (best_move, alpha)
}


fn quiescence(board_status: BoardStatus, beta: isize, mut alpha: isize) -> isize {
    let stdpt = eveluate(&board_status);
    if stdpt >= beta {return beta}
    alpha = isize::max(alpha, stdpt);
    for mov in MoveList::new(&board_status).iterate_moves().filter(MoveBitField::is_move_capture) {
        let mut board = board_status;
        if !board.make_move(mov) {continue;}
        let score = -quiescence(board, -alpha, -beta);
        if score >= beta {return beta;}
        alpha = isize::max(alpha, score);
    }   

    alpha
}

fn negamax(board_status: BoardStatus, beta: isize, mut alpha: isize, depth: isize) -> isize {
    if depth == 0 {return quiescence(board_status, beta, alpha);}
    let move_list = MoveList::new(&board_status);
    let mut move_count = 0;
    for mov in move_list.iterate_moves() {
        let mut board = board_status;
        if !board.make_move(mov) {continue;}
        move_count += 1;
        let score = -negamax(board, -alpha, -beta, depth - 1);
        if score >= beta {return beta;}
        alpha = isize::max(score, alpha);
    }
    if move_count == 0 {
        if board_status.get_color() == Color::White {
            if is_square_attacked_white(&board_status, board_status[BoardSlots::WhiteKing].get_lsb_index()) {return -50000 - depth;}
        }
        else {
            if is_square_attacked_black(&board_status, board_status[BoardSlots::BlackKing].get_lsb_index()) {return -50000 - depth;}
        }
        return 0;
    }
    alpha
}

