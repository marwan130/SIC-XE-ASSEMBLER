use std::fs::File;
use std::io::{self, BufRead};

pub fn process_file(file_path: &str) -> io::Result<()> {
    // open the file and read its contents
    let input_file = File::open(file_path)?;
    let reader = io::BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    // define the variables to store the data
    let mut labels: Vec<String> = Vec::new();
    let mut instr: Vec<String> = Vec::new();
    let mut ref_data: Vec<String> = Vec::new();

    for line in &lines {
        let line_split: Vec<String> = line.split(':').next().unwrap_or("").split_whitespace().map(|s| s.to_string()).collect();

        match line_split.len() {
            3 => {
                labels.push(line_split[0].trim().trim_end_matches(',').to_string());
                instr.push(line_split[1].trim().trim_end_matches(',').to_string());
                ref_data.push(line_split[2].trim().trim_end_matches(',').to_string());
            }

            2 => {
                labels.push("&".to_string());
                instr.push(line_split[0].trim().trim_end_matches(',').to_string());
                ref_data.push(line_split[1].trim().trim_end_matches(',').to_string());
            }

            1 => {
                labels.push("&".to_string());
                instr.push(line_split[0].trim().trim_end_matches(',').to_string());
                ref_data.push("&".to_string());
            }
            _ => {}
        }
    }
    let literals: Vec<String> = ref_data.iter().filter(|r| r.starts_with('=')).cloned().collect();

    println!("Labels: {:?}", labels);
    println!("Instructions: {:?}", instr);
    println!("References: {:?}", ref_data);
    println!("Literals: {:?}", literals);

    Ok(())
}

