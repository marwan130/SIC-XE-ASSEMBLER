pub mod conversions;
pub mod pass1;
use crate::pass1::Pass1;
pub mod pass2;

fn main() {
    let file_path = "src/in.txt";
    let mut processor = Pass1::new();
    if let Err(err) = processor.process_file(file_path) {
        eprintln!("Error processing file: {}", err);
    }
    
    processor.pass1_generator();

}
