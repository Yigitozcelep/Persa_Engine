use crate::{is_square_set, clear_bit, set_bit};
// the edges of the board is not setted due to implementation of magic bitboard
// boundries included
pub fn mask_direction(square: usize, direction: usize, boundry: usize) -> usize{
    let mut attack: usize = 0;

    for i in 0..8 {
        let target = square + direction.wrapping_mul(i);
        set_bit!(attack, target);
        if is_square_set!(boundry, target) {break;}
    }
    clear_bit!(attack, square);
    attack
}