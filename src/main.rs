mod pass1;
mod pass2;
mod conversions;

use pass1::Pass1;
use pass2::Pass2;

fn main() {
    let mut pass1 = Pass1::new();
    let _ = pass1.process_file("inputs/in.txt");
    pass1.pass1_generator();

    let mut pass2 = Pass2::new();
    let _ = pass2.read_intermediate_file("outputs/intermediate.txt");
    let _ = pass2.read_symbol_table("outputs/symbTable.txt");
    let _ = pass2.read_literal_table("outputs/litTable.txt");
}
