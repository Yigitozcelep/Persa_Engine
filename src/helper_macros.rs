#[macro_export]
macro_rules! set_bit {
    ($board:expr, $shift: expr) => {
        $board = $board | 1 << $shift;
    };
}

#[macro_export]
macro_rules! clear_bit {
    ($board:expr, $shift: expr) => {
        $board = $board & !(1 << $shift);
    };
}

#[macro_export]
macro_rules! toggle_bit {
    ($board:expr, $shift: expr) => {
        $board = $board ^ (1 << $shift);
    };
}

#[macro_export]
macro_rules! isolate_bit {
    ($board:expr, $bit: expr) => {
        $board & (1 << $bit);
    };
}

#[macro_export]
macro_rules! is_square_set {
    ($board:expr, $square: expr) => {
        ($board & (1 << $square)) != 0
    };
}


#[macro_export]
macro_rules! shift_board_left {
    ($board:expr, $num: expr) => {
        $board >> $num
    };
}

#[macro_export]
macro_rules! shift_board_right {
    ($board:expr, $num: expr) => {
        $board << $num
    };
}

#[macro_export]
macro_rules! shift_board_up {
    ($board:expr, $num: expr) => {
        shift_board_right!($board, $num * 8)
    };
}

#[macro_export]
macro_rules! shift_board_down {
    ($board:expr, $num: expr) => {
        shift_board_left!($board, $num * 8)
    };
}


#[macro_export]
macro_rules! get_rank {
    ($square:expr) => {
        $square / 8
    };
}

#[macro_export]
macro_rules! get_file {
    ($square:expr) => {
        $square % 8
    };
}
