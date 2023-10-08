pub mod constants;
pub mod debug;
pub mod helper_functions;
pub mod helper_macros;
pub mod tables;

fn main() {
    tables::bishop_table::create_bishop_table();
}
