use std::fs;
use persa_chess::{debug::{FenString, perft_driver}, pieces::tables::init_statics};


#[test]
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
