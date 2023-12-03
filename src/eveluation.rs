use crate::pieces::pieces_controller::{BoardSlots, BoardStatus, MoveList, MoveBitField};
use crate::board_components::Color;
use crate::constants::eveluation_constants::MATERIAL_SCORES;
use crate::pieces::pieces_controller::{is_square_attacked_black, is_square_attacked_white};
use crate::uci::UciInformation;


#[inline(always)]
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


pub fn find_best_move(uci_info: &mut UciInformation) {
    if uci_info.depth_limit == 0 {return ;}
    for depth in 1..uci_info.depth_limit + 1 {
        uci_info.node_count = 1;
        let move_list = MoveList::new(uci_info);
        let mut best_move = MoveBitField::NO_MOVE;
        let beta = 1000000;
        let mut alpha = -1000000;
        let old_board = uci_info.board;
        for mov in move_list.iterate_moves() {
            if uci_info.board.make_move(mov) {
                let score = -negamax(uci_info, -alpha, -beta, depth -1);
                if score > alpha {
                    alpha = score;
                    best_move = mov;
                }
            }
            uci_info.board = old_board;
        }
        uci_info.board_history.add_new_best_move(best_move);
    }
}

#[inline(always)]
fn quiescence(uci_info: &mut UciInformation, beta: isize, mut alpha: isize) -> isize {
    uci_info.node_count += 1;
    let stdpt = eveluate(&uci_info.board);
    if stdpt >= beta {return beta}
    alpha = isize::max(alpha, stdpt);
    let old_board = uci_info.board;
    for mov in MoveList::new(&uci_info).iterate_moves().filter(MoveBitField::is_move_capture) {
        if uci_info.board.make_move(mov) {
            let score = -quiescence(uci_info, -alpha, -beta);
            if score >= beta {return beta;}
            alpha = isize::max(alpha, score);
        }
        uci_info.board = old_board;
    }   
    alpha
}

#[inline(always)]
fn negamax(uci_info: &mut UciInformation, beta: isize, mut alpha: isize, depth: isize) -> isize {
    if depth == 0 { return quiescence(uci_info, beta, alpha); }
    uci_info.node_count += 1;
    let move_list = MoveList::new(&uci_info);
    let mut move_count = 0;
    let old_board = uci_info.board;
    for mov in move_list.iterate_moves() {
        if uci_info.board.make_move(mov) {
            let score = -negamax(uci_info, -alpha, -beta, depth - 1);
            if score >= beta {
                if !mov.is_move_capture() {uci_info.board_history.append_killer_move(mov, old_board.get_half_move());}
                return beta;
            }
            alpha = isize::max(score, alpha);
            move_count += 1;
        }
        uci_info.board = old_board;
    }
    if move_count == 0 {
        if uci_info.board.get_color() == Color::White {
            if is_square_attacked_white(&uci_info.board, uci_info.board[BoardSlots::WhiteKing].get_lsb_index()) {return -50000 - depth;}
        }
        else {
            if is_square_attacked_black(&uci_info.board, uci_info.board[BoardSlots::BlackKing].get_lsb_index()) {return -50000 - depth;}
        }
        return 0;
    }
    alpha
}