use std::fs;
use persa_chess::{debug::{FenString, perft_driver}, pieces::{tables::init_statics, pieces_controller::{MoveBitField, BoardSlots}}, eveluation::find_best_move};
use persa_chess::pieces::pieces_controller::{is_square_attacked_black, is_square_attacked_white, MoveList};

#[test]
#[ignore]
pub fn test_pertfs() {
    let contents = fs::read_to_string("./perfts.txt")
        .expect("Should have been able to read the file");
    
    init_statics();
    for part in contents.split("\n") {
        let data: Vec<&str> = part.split(";").collect();
        let fen = data[0];
        for i in 1..5 {
            let perft = data[i].split(" ").nth(1).unwrap();
            let board = FenString::new(fen.to_string()).convert_to_board();
            let res = perft_driver(&board, i);
            assert_eq!(res, perft.parse().unwrap())
        }

    }
}

#[test]
pub fn find_mate() {
    init_statics();
    let fens = [
        "2k5/5Q2/K7/8/8/8/8/8 b - - 5 4", 
        "8/8/KQ2k3/2R5/8/8/8/8 b - - 0 0",
        "8/8/kq2K3/2r5/8/8/8/8 w - - 0 0",
        "rn1B1bnr/pp1B1ppp/k7/8/4N3/3p4/PPP1QPPP/2KR2NR w - - 0 13",
        "r4r2/1R4Rk/7p/8/8/5Ppq/P7/6K1 b - - 0 2",
        "3rr1k1/pp3ppp/8/2p5/2Q5/6bP/PPP1B1P1/R1B2K1R b - - 2 3",
        "7k/ppp2pp1/1q4b1/5rQp/8/1P6/PBP2PPP/6K1 w - - 0 3",
        "8/6k1/8/3b3Q/pP4P1/1P6/KP6/5r2 b - - 2 3",
        "3r1rk1/pp2bp1Q/2b1n1p1/q2pR3/8/1P1B4/PB3PPP/2K4R b - - 0 2",
        "k2r4/pp4bp/N7/8/1P2nQ2/P4pPq/5P2/6K1 w - - 2 3",
        "5Qrk/7p/p1b2p2/8/7N/8/PqP2PPP/6K1 w - - 2 3",
        "r1b2r1k/1ppq3p/p4p2/4B3/2B5/1P6/P4P1P/6RK w - - 0 3",
        "6k1/1pQ3p1/p6p/8/4b3/PP2PnPP/3rKP2/8 w - - 3 3",
        "8/Qkp3r1/p1N4p/1p1P4/6q1/8/P5P1/6K1 b - - 3 2",
        "8/5p2/pkQ1pb2/1B1p4/P7/P2P2P1/4rPK1/4q3 b - - 1 2",
        "1rr1q1k1/p4p1p/5Q2/3pPp2/3P4/6R1/P5PP/6K1 b - - 3 2",
        "6r1/5p1k/3p1p2/1p2qP2/4P3/1P2B2R/r5PP/6K1 b - - 1 2",
        "k3r2r/2p3pp/QB1pqp2/8/2P5/1P3P2/5NPP/6K1 b - - 3 2",
        "r3r2k/2R4R/1p5P/3p2p1/q7/3P1P2/1PPNB3/1K6 b - - 3 2",
        "R4n2/4rpkp/1p4pP/5bP1/4N3/1qP5/1P6/2K5 b - - 0 2",
        "7k/r5r1/2bpNp1Q/q1bN1P2/1p6/6P1/1PP5/1K6 b - - 1 2",
        "7r/1k2b3/6p1/4p3/1pB2n2/1P6/5PNn/R2R1K2 w - - 0 3",
        "1r6/5pRk/pqp1p2p/4Nn1N/7P/8/PPP2P2/2K5 b - - 0 2",
        "1r5r/k4ppp/2p2q2/Q3n3/8/1P1B2P1/P4P1P/5RK1 b - - 1 2",
    ];
    for fen in fens {
        let mut board = FenString::new(fen.to_string()).convert_to_board();
        for _ in 0..6 {
            let (mov, _) = find_best_move(board, 5);
            if mov == MoveBitField::NO_MOVE {break;}
            board.make_move(mov);
        }
        match board.get_color() {
            persa_chess::board_components::Color::White => {
                if !is_square_attacked_white(&board, board[BoardSlots::WhiteKing].get_lsb_index()) || !MoveList::new(&board).count == 0 {panic!("{} can not find mate", board)}
            },
            persa_chess::board_components::Color::Black => {
                if !is_square_attacked_black(&board, board[BoardSlots::BlackKing].get_lsb_index()) || !MoveList::new(&board).count == 0 {panic!("{} can not find mate", board)}
            },
        }
    }
}


#[test]
pub fn tempo_tests() {
    init_statics();
    //let fens = [
    //    "8/5k2/5n2/4K1Q1/8/8/8/8 w - - 28 15",
    //    "8/5K2/5N2/4k1q1/8/8/8/8 b - - 28 15",
    //];

}