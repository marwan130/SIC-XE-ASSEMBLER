mod pass1;
mod pass2;
mod conversions;

use pass1::Pass1;
use pass2::Pass2;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <job_id>");
        std::process::exit(1);
    }
    
    let job_id = &args[1];
    let input_dir = format!("jobs/{}/input", job_id);
    let output_dir = format!("jobs/{}/output", job_id);
    let input_file = format!("{}/in.txt", input_dir);
    
    // Create directory structure
    fs::create_dir_all(&input_dir).expect("Failed to create input directory");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    let intermediate_path = format!("{}/intermediate.txt", output_dir);
    let symbol_path = format!("{}/symbTable.txt", output_dir);
    let literal_path = format!("{}/litTable.txt", output_dir);
    let object_path = format!("{}/object_program.txt", output_dir);
    
    let mut pass1 = Pass1::new();
    let _ = pass1.process_file(&input_file);
    pass1.pass1_generator(&output_dir);

    let mut pass2 = Pass2::new();
    let _ = pass2.pass2_generator(&intermediate_path, &symbol_path, &literal_path, &object_path);
}
