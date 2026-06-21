use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Pass2 {
    pub labels: Vec<String>,
    pub instr: Vec<String>,
    pub operands: Vec<String>,
    pub symbol_table: HashMap<String, String>,
    pub literal_table: HashMap<String, String>,
    pub object_code: HashMap<usize, String>,
}

impl Pass2 {
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            instr: Vec::new(),
            operands: Vec::new(),
            symbol_table: HashMap::new(),
            literal_table: HashMap::new(),
            object_code: HashMap::new(),
        }
    }

    pub fn read_intermediate_file(&mut self, file_path: &str) -> io::Result<()> {
        let input_file = File::open(file_path)?;
        let reader = io::BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        for line in lines {
            let parts: Vec<String> = line.split_whitespace()
                .map(|s| s.to_string())
                .collect();

            if parts.len() >= 3 {
                self.labels.push(parts[0].clone());
                self.instr.push(parts[1].clone());
                self.operands.push(parts[2].clone());
            } else if parts.len() == 2 {
                self.labels.push("&".to_string());
                self.instr.push(parts[0].clone());
                self.operands.push(parts[1].clone());
            } else if parts.len() == 1 {
                self.labels.push("&".to_string());
                self.instr.push(parts[0].clone());
                self.operands.push("&".to_string());
            }
        }

        Ok(())
    }

    pub fn read_symbol_table(&mut self, file_path: &str) -> io::Result<()> {
        let input_file = File::open(file_path)?;
        let reader = io::BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        for line in lines {
            let parts: Vec<String> = line.split_whitespace()
                .map(|s| s.to_string())
                .collect();

            if parts.len() >= 2 {
                self.symbol_table.insert(parts[0].clone(), parts[1].clone());
            }
        }

        Ok(())
    }

    pub fn read_literal_table(&mut self, file_path: &str) -> io::Result<()> {
        let input_file = File::open(file_path)?;
        let reader = io::BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        for line in lines {
            let parts: Vec<String> = line.split_whitespace()
                .map(|s| s.to_string())
                .collect();

            if parts.len() >= 2 {
                self.literal_table.insert(parts[0].clone(), parts[1].clone());
            }
        }

        Ok(())
    }
}