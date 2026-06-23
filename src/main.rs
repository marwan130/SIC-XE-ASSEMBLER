mod pass1;
mod pass2;
mod conversions;

use pass1::Pass1;
use pass2::Pass2;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 { &args[1] } else { "inputs/in.txt" };
    
    let mut pass1 = Pass1::new();
    let _ = pass1.process_file(input_file);
    pass1.pass1_generator();

    let mut pass2 = Pass2::new();
    let _ = pass2.pass2_generator("outputs/intermediate.txt", "outputs/symbTable.txt", "outputs/litTable.txt", "outputs/object_program.txt");
}
