mod pass1;
use crate::pass1::process_file;

fn main() {
    let file_path = "src/in.txt";
    if let Err(err) = process_file(file_path) {
        eprintln!("Error processing file: {}", err);
    }
}
