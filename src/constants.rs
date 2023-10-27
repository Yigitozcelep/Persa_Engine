

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
    use crate::board_components::{BitBoard, ChessBoard, MagicNum};
    pub const ROOK_MAX_BLOCK_PERM:   usize = 4096;
    pub const BISHOP_MAX_BLOCK_PERM: usize = 512;

    pub const EMPTY_BITBOARD:     BitBoard =   BitBoard(0);
    pub const A_FILE:             BitBoard =   BitBoard(0x0101010101010101);
    pub const B_FILE:             BitBoard =   BitBoard(0x0202020202020202);
    pub const G_FILE:             BitBoard =   BitBoard(0x4040404040404040);
    pub const H_FILE:             BitBoard =   BitBoard(0x8080808080808080);
    pub const RANK1:              BitBoard =   BitBoard(0x00000000000000FF);
    pub const RANK2:              BitBoard =   BitBoard(0x000000000000FF00);
    pub const RANK7:              BitBoard =   BitBoard(0x00FF000000000000);
    pub const RANK8:              BitBoard =   BitBoard(0xFF00000000000000);
    pub const A1_H8_DIOGNAL:      BitBoard =   BitBoard(0x8040201008040201);
    pub const H1_A8_ANTI_DIOGNAL: BitBoard =   BitBoard(0x0102040810204080);
    pub const LIGHT_SQUARES:      BitBoard =   BitBoard(0x55AA55AA55AA55AA);
    pub const DARK_SQUARES:       BitBoard =   BitBoard(0xAA55AA55AA55AA55);
    pub const EDGES:              BitBoard =   BitBoard(0xFF818181818181FF);
    pub const CORNERS:            BitBoard =   BitBoard(0x8100000000000081);
    pub const TOP_2_RANK:         BitBoard =   BitBoard(0xFFFF000000000000);
    pub const RIGHT_2_FILE:       BitBoard =   BitBoard(0xC0C0C0C0C0C0C0C0);
    pub const BOTTOM_2_RANK:      BitBoard =   BitBoard(0x000000000000FFFF);
    pub const LEFT_2_FILE:        BitBoard =   BitBoard(0x0303030303030303);

    pub static ASCII_PIECES: [&'static str; 12] = ["p", "n", "b", "r", "q", "k", "P", "N", "B", "R", "Q", "K"];
    pub static UNICODE_PIECES: [char; 12] =       ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔'];



    pub const fn create_bishop_move_counts() -> ChessBoard<u64> {
        ChessBoard([
            6, 5, 5, 5, 5, 5, 5, 6, 
            5, 5, 5, 5, 5, 5, 5, 5, 
            5, 5, 7, 7, 7, 7, 5, 5, 
            5, 5, 7, 9, 9, 7, 5, 5, 
            5, 5, 7, 9, 9, 7, 5, 5, 
            5, 5, 7, 7, 7, 7, 5, 5, 
            5, 5, 5, 5, 5, 5, 5, 5, 
            6, 5, 5, 5, 5, 5, 5, 6
        ])
    }

    pub const fn create_rook_move_counts() -> ChessBoard<u64> {
        ChessBoard([
            12, 11, 11, 11, 11, 11, 11, 12, 
            11, 10, 10, 10, 10, 10, 10, 11, 
            11, 10, 10, 10, 10, 10, 10, 11, 
            11, 10, 10, 10, 10, 10, 10, 11, 
            11, 10, 10, 10, 10, 10, 10, 11, 
            11, 10, 10, 10, 10, 10, 10, 11, 
            11, 10, 10, 10, 10, 10, 10, 11, 
            12, 11, 11, 11, 11, 11, 11, 12
            ])
        }
    // these numbers generated by pieces::helper_functions::find_magic_number
    pub const fn create_rook_magics() -> ChessBoard<MagicNum> {
        ChessBoard([
            MagicNum(0x8A80104000800020), MagicNum(0xC40100040082000),  MagicNum(0x100102001000840),  MagicNum(0x1080041000080080), 
            MagicNum(0x4280240080020800), MagicNum(0x4800A00211C0080),  MagicNum(0x1080008001000200), MagicNum(0x42000082C9020424), 
            MagicNum(0x2002081004200),    MagicNum(0x2002081004200),    MagicNum(0x801000802000),     MagicNum(0x201001000082100), 
            MagicNum(0xE41001005000800),  MagicNum(0x1022001008854200), MagicNum(0x211000100020084),  MagicNum(0x18801041000080), 
            MagicNum(0x80084000200040),   MagicNum(0x20A0024000500020), MagicNum(0x80410010200901),   MagicNum(0x2083090010002300), 
            MagicNum(0x808004000800),     MagicNum(0x804008080040200),  MagicNum(0x8800040002100108), MagicNum(0x20001208044), 
            MagicNum(0x4020800080204000), MagicNum(0x40008280200042),   MagicNum(0x820200204010),     MagicNum(0x200100480080080), 
            MagicNum(0x300040080080080),  MagicNum(0x804008080040200),  MagicNum(0x8000020400881001), MagicNum(0x88808200204401), 
            MagicNum(0x6480042006400041), MagicNum(0x4080804000802000), MagicNum(0x801000802000),     MagicNum(0x1518001000800882), 
            MagicNum(0xE41001005000800),  MagicNum(0x1012001002000408), MagicNum(0x140108804006201),  MagicNum(0x2050882000054), 
            MagicNum(0x90080C000618011),  MagicNum(0xA0004000208080),   MagicNum(0x22001080220043),   MagicNum(0x1012010050008), 
            MagicNum(0x40008008080),      MagicNum(0x1100040002008080), MagicNum(0x40100182040008),   MagicNum(0x800000648102000C), 
            MagicNum(0x481248002C90100),  MagicNum(0x2002081004200),    MagicNum(0x400C802211420200), MagicNum(0x280200C10010100), 
            MagicNum(0x300040080080080),  MagicNum(0x1100040002008080), MagicNum(0x4000018802100400), MagicNum(0x4310800100004080), 
            MagicNum(0x4024800508102041), MagicNum(0x88801100204001),   MagicNum(0x401080104200200A), MagicNum(0x8010210408100101), 
            MagicNum(0x9202002005881002), MagicNum(0x8012004824011022), MagicNum(0x2000011002080084), MagicNum(0x1010549228402), 
        ])
    }
    
    // these numbers generated by pieces::helper_functions::find_magic_number
    pub const fn create_bishop_magics() -> ChessBoard<MagicNum> {
        ChessBoard([
            MagicNum(0x40040822862081),   MagicNum(0x10201A0200411402), MagicNum(0x81024288020C000),  MagicNum(0x1404640080008810), 
            MagicNum(0x9004242000012008), MagicNum(0x10A412020A04008),  MagicNum(0x1000989008208000), MagicNum(0x22010108410402), 
            MagicNum(0x2104840810014200), MagicNum(0x2150210810A080),   MagicNum(0x81080089061040),   MagicNum(0x2400A82040408008), 
            MagicNum(0x240420005810),     MagicNum(0x4200022860180000), MagicNum(0x4000090082504000), MagicNum(0x410402422220), 
            MagicNum(0x8140420C80200),    MagicNum(0x2038A4832080200),  MagicNum(0xC108005006404048), MagicNum(0x2208001041404049), 
            MagicNum(0xC014021880A01000), MagicNum(0x704200110101006),  MagicNum(0x2808C12013100),    MagicNum(0x6400411200444402), 
            MagicNum(0x124240A1041400),   MagicNum(0x8802088010100088), MagicNum(0x4020040408508),    MagicNum(0x604080004006128), 
            MagicNum(0x8000848004002004), MagicNum(0x4008020008405210), MagicNum(0x806000AA22902),    MagicNum(0x2200888682020080), 
            MagicNum(0x10C2105000400320), MagicNum(0x8018010800102208), MagicNum(0x4000841102100044), MagicNum(0x200800050104), 
            MagicNum(0x160C030400280408), MagicNum(0x820080320094405),  MagicNum(0x201E40100540101),  MagicNum(0x8408248080002227), 
            MagicNum(0x8021004001040),    MagicNum(0x400455030034803),  MagicNum(0x912020322180400),  MagicNum(0x8026013002800), 
            MagicNum(0xE000040810130200), MagicNum(0x2168500092000020), MagicNum(0x2002304119002200), MagicNum(0x1901020400400510), 
            MagicNum(0x1000989008208000), MagicNum(0x100840108020006),  MagicNum(0x3010020842088080), MagicNum(0x8000001B84044881), 
            MagicNum(0x8004404010510072), MagicNum(0xC10801010000),     MagicNum(0x4090808148101),    MagicNum(0x10201A0200411402), 
            MagicNum(0x22010108410402),   MagicNum(0x410402422220),     MagicNum(0x41000114204D004),  MagicNum(0x4082000100840440), 
            MagicNum(0x400C802211420200), MagicNum(0x81800140828C8100), MagicNum(0x2104840810014200), MagicNum(0x40040822862081), 
        ])
    }
}
