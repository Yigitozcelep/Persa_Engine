

pub mod squares {
    use crate::board_components::Square;

    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);

    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);

    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);

    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);

    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);

    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);

    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);

    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

}



pub mod directions {
    use crate::board_components::Direction;
    pub const NORTH_WEST: Direction = Direction(7);
    pub const NORTH:      Direction = Direction(8);
    pub const NORTH_EAST: Direction = Direction(9);
    pub const EAST:       Direction = Direction(1);
    pub const SOUTH_EAST: Direction = Direction(u8::MAX - 6);
    pub const SOUTH:      Direction = Direction(u8::MAX - 7);
    pub const SOUTH_WEST: Direction = Direction(u8::MAX - 8);
    pub const WEST:       Direction = Direction(u8::MAX);
}

pub mod board_constants {
    use crate::board_components::Board;
    pub const EMPTY_BOARD:        Board =   Board(0);
    pub const A_FILE:             Board =   Board(0x0101010101010101);
    pub const B_FILE:             Board =   Board(0x0202020202020202);
    pub const G_FILE:             Board =   Board(0x4040404040404040);
    pub const H_FILE:             Board =   Board(0x8080808080808080);
    pub const RANK1:              Board =   Board(0x00000000000000FF);
    pub const RANK2:              Board =   Board(0x000000000000FF00);
    pub const RANK7:              Board =   Board(0x00FF000000000000);
    pub const RANK8:              Board =   Board(0xFF00000000000000);
    pub const A1_H8_DIOGNAL:      Board =   Board(0x8040201008040201);
    pub const H1_A8_ANTI_DIOGNAL: Board =   Board(0x0102040810204080);
    pub const LIGHT_SQUARES:      Board =   Board(0x55AA55AA55AA55AA);
    pub const DARK_SQUARES:       Board =   Board(0xAA55AA55AA55AA55);
    pub const EDGES:              Board =   Board(0xFF818181818181FF);
    pub const CORNERS:            Board =   Board(0x8100000000000081);
    pub const TOP_2_RANK:         Board =   Board(0xFFFF000000000000);
    pub const RIGHT_2_FILE:       Board =   Board(0xC0C0C0C0C0C0C0C0);
    pub const BOTTOM_2_RANK:      Board =   Board(0x000000000000FFFF);
    pub const LEFT_2_FILE:        Board =   Board(0x0303030303030303);
}
