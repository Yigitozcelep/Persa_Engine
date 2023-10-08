use crate::{get_file, get_rank};

pub fn print_bit_board (board: usize) {
    let first_space = " ".repeat(20);
    let second_space = " ".repeat(2);
    println!("\n{} {}+-----------------+", first_space, second_space);
    for i in (0..8).rev() {
        print!("{}{}{}| ", first_space, (i + 1).to_string(), second_space);
        for j in 0..8 {
            let shift = i * 8 + j;
            let bit = (board & (1 << shift)) >> shift;
            print!("{} ", bit);

        }
        print!("|\n");
    }
    println!("{} {}+-----------------+\n", first_space, second_space);
    let files = ["A","B","C","D","E","F","G","H"];
    print!("{} {}  ", first_space, second_space);
    for i in 0..8 {
        print!("{} ", files[i]);
    }
    println!("\n");
}

pub fn get_square_name (square: usize) -> String{
    let files = ["A","B","C","D","E","F","G","H"];
    files[get_file!(square)].to_string() + &(get_rank!(square) + 1).to_string()
}

