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
    
    score                                                                                                   
}

fn mini(board_status: BoardStatus, depth: isize, alpha: isize, mut beta: isize) -> isize {
    if depth == 0 {return eveluate(&board_status);}
    let move_list = MoveList::new(&board_status);
    let mut min_val = isize::MAX as isize;
    let mut move_count = 0;
    for mov in move_list.iterate_moves() {
        let mut board = board_status;
        if !board.make_move(mov) {continue;}
        move_count += 1;
        let val = maxi(board, depth - 1, alpha, beta);
        min_val = isize::min(min_val, val);
        beta = isize::min(beta, min_val);
        if beta <= alpha {break;}
    }
    
    if move_count == 0 {
        if is_square_attacked_black(&board_status, board_status[BoardSlots::BlackKing].get_lsb_index()) {return 5000000 + depth;}
        return 0;
    }
    min_val
}

fn maxi(board_status: BoardStatus, depth: isize, mut alpha: isize, beta: isize) -> isize {
    if depth == 0 {return eveluate(&board_status);}
    let move_list = MoveList::new(&board_status);
    let mut max_val = isize::MIN as isize;
    let mut move_count = 0;
    for mov in move_list.iterate_moves() {
        let mut board = board_status;
        if !board.make_move(mov) {continue;}
        move_count += 1;
        let val = mini(board, depth - 1, alpha, beta);
        max_val = isize::max(max_val, val);
        alpha = isize::max(alpha, max_val);
        if beta <= alpha {break;}
    }
    if move_count == 0 {
        if is_square_attacked_white(&board_status, board_status[BoardSlots::WhiteKing].get_lsb_index()) {return -5000000 - depth;}
        return 0;
    }
    max_val - depth
}

fn get_white_best(board_status: BoardStatus, depth: isize) -> (MoveBitField, isize) {
    debug_assert_ne!(0, depth);
    let move_list = MoveList::new(&board_status);
    let mut alpha = isize::MIN;
    let beta  = isize::MAX;
    let mut max_val = isize::MIN as isize;
    let mut best_move = MoveBitField::NO_MOVE;
    for mov in move_list.iterate_moves() {
        let mut board = board_status;
        if !board.make_move(mov) {continue;}
        let val = mini(board, depth -1, alpha, beta);
        if val > max_val {
            best_move = mov;
            max_val = val;
        }
        alpha = isize::max(alpha, max_val);
    }
    (best_move, max_val)
}

fn get_black_best(board_status: BoardStatus, depth: isize) -> (MoveBitField, isize){
    debug_assert_ne!(0, depth);

    let move_list = MoveList::new(&board_status);
    let alpha = isize::MIN;
    let mut beta  = isize::MAX;
    let mut min_val = isize::MAX as isize;
    let mut best_move = MoveBitField::NO_MOVE;
    for mov in move_list.iterate_moves() {
        let mut board = board_status;
        if !board.make_move(mov) {continue;}
        let val = maxi(board, depth -1, alpha, beta);
        if val < min_val {
            best_move = mov;
            min_val = val;
        }
        beta = isize::min(beta, min_val);
    }
    (best_move, min_val)
}

pub fn minimax(board_status: BoardStatus, depth: isize) -> (MoveBitField, isize) {
    match board_status.get_color() {
        Color::White => get_white_best(board_status, depth),
        Color::Black => get_black_best(board_status, depth),
    }
}                                                                                                                                                                                                                                                                                              
                                                                                                            