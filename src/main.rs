pub mod pieces;
pub mod board_components;
pub mod constants;
pub mod debug;
pub mod eveluation;
pub mod helper_macros;
pub mod uci;

use pieces::tables::init_statics;
use uci::uci_loop;

fn main() {
    init_statics();
    uci_loop();
}   