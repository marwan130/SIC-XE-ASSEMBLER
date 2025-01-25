use crate::conversions::*;
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::collections::HashMap;
use itertools::izip;

pub struct Pass1 {
    pub lines: Vec<String>,
    pub labels: Vec<String>,
    pub instr: Vec<String>,
    pub ref_data: Vec<String>,
    pub literals: Vec<String>,
    pub locctr: HashMap<usize, String>,
    pub default_locctr: HashMap<usize, String>,
    pub defaultb_locctr: HashMap<usize, String>,
    pub cdata_locctr: HashMap<usize, String>,
    pub cblks_locctr: HashMap<usize, String>,
}

impl Pass1 {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            labels: Vec::new(),
            instr: Vec::new(),
            ref_data: Vec::new(),
            literals: Vec::new(),
            locctr: HashMap::new(),
            default_locctr: HashMap::new(),
            defaultb_locctr: HashMap::new(),
            cdata_locctr: HashMap::new(),
            cblks_locctr: HashMap::new(),
        }
    }

    pub fn process_file(&mut self, file_path: &str) -> io::Result<()> {
        let input_file = File::open(file_path)?;
        let reader = io::BufReader::new(input_file);
        self.lines = reader.lines().filter_map(Result::ok).collect();

        for line in &self.lines {
            let parts: Vec<String> = line.split(';')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .map(|s| s.to_uppercase())
                .collect();

            match parts.len() {
                3 => {
                    self.labels.push(parts[0].trim_end_matches(',').to_string());
                    self.instr.push(parts[1].trim_end_matches(',').to_string());
                    self.ref_data.push(parts[2].trim_end_matches(',').to_string());
                }
                2 => {
                    self.labels.push("&".to_string());
                    self.instr.push(parts[0].trim_end_matches(',').to_string());
                    self.ref_data.push(parts[1].trim_end_matches(',').to_string());
                }
                1 => {
                    self.labels.push("&".to_string());
                    self.instr.push(parts[0].trim_end_matches(',').to_string());
                    self.ref_data.push("&".to_string());
                }
                _ => {}
            }
        }

        self.literals = self.ref_data.iter()
            .filter(|r| r.starts_with('='))
            .cloned()
            .collect();

        let mut intermediate_file = File::create("src/intermediate.txt")?;
        for (label, instr, ref_data) in izip!(&self.labels, &self.instr, &self.ref_data) {
            writeln!(intermediate_file, "{}\t{}\t{}", label, instr, ref_data)?;
        }

        Ok(())
    }

    pub fn blocks_generator(&mut self) {
        self.initialize_location_counters();

        let mut pass1_file = File::create("src/out_pass1.txt").unwrap();
        for i in 0..self.lines.len() {
            self.update_locctr(i, &mut pass1_file);
        }
    }

    fn initialize_location_counters(&mut self) {
        self.locctr.insert(0, "".to_string());
        self.default_locctr.insert(0, "".to_string());
        self.defaultb_locctr.insert(0, "".to_string());
        self.cdata_locctr.insert(0, "".to_string());
        self.cblks_locctr.insert(0, "".to_string());
    }    

    fn calculate_difference(&self, input: &str) -> usize {
        if input.contains('-') {
            let parts: Vec<&str> = input.split('-').collect();
            if parts.len() == 2 {
                let (part1, part2) = (parts[0].trim(), parts[1].trim());
    
                let mut value1 = 0;
                let mut value2 = 0;
    
                // check if the first part is a label and exists in `labels`
                if let Some(index1) = self.labels.iter().position(|label| label == part1) {
                    if let Some(locctr_value1) = self.locctr.get(&index1) {
                        if let Ok(locctr_numeric1) = usize::from_str_radix(locctr_value1, 16) {
                            value1 = locctr_numeric1;
                        }
                    }
                } else {
                    // if the first part is a number, just parse it
                    value1 = part1.parse::<usize>().unwrap_or(0);
                }
    
                // check if the second part is a label and exists in `labels`
                if let Some(index2) = self.labels.iter().position(|label| label == part2) {
                    if let Some(locctr_value2) = self.locctr.get(&index2) {
                        if let Ok(locctr_numeric2) = usize::from_str_radix(locctr_value2, 16) {
                            value2 = locctr_numeric2;
                        }
                    }
                } else {
                    // if the second part is a number, just parse it
                    value2 = part2.parse::<usize>().unwrap_or(0);
                }
    
                // subtract the two values
                return value1.saturating_sub(value2);
            }
        } else if input.contains('+') {
            let parts: Vec<&str> = input.split('+').collect();
            if parts.len() == 2 {
                let (part1, part2) = (parts[0].trim(), parts[1].trim());
    
                let mut value1 = 0;
                let mut value2 = 0;
    
                if let Some(index1) = self.labels.iter().position(|label| label == part1) {
                    if let Some(locctr_value1) = self.locctr.get(&index1) {
                        if let Ok(locctr_numeric1) = usize::from_str_radix(locctr_value1, 16) {
                            value1 = locctr_numeric1;
                        }
                    }
                } else {
                    value1 = part1.parse::<usize>().unwrap_or(0);
                }

                if let Some(index2) = self.labels.iter().position(|label| label == part2) {
                    if let Some(locctr_value2) = self.locctr.get(&index2) {
                        if let Ok(locctr_numeric2) = usize::from_str_radix(locctr_value2, 16) {
                            value2 = locctr_numeric2;
                        }
                    }
                } else {
                    value2 = part2.parse::<usize>().unwrap_or(0);
                }
                return value1.saturating_add(value2);
            }
        }
    
        // return 0 if no valid operation was performed
        0
    }
    
    fn update_locctr(&mut self, i: usize, pass1_file: &mut File) {
        let instr_type = &self.instr[i];
        let ref_type = &self.ref_data[i];
        let format1 = ["FIX", "FLOAT", "HIO", "SIO", "TIO", "NORM"];
        let format2=["ADDR", "CLEAR", "COMPR", "DIVR", "MULR", "RMO", "SHIFTR", "SHIFTL", "SUBR", "SVC", "TIXR"];
        let format3=["ADD", "ADDF", "AND", "COMP", "COMPF", "DIV", "J", "JEQ", "JGT", "JLT", "JSUB", "LDA", "LDB", "LDCH", "LDF", "LDL", "LDS", "LDT", "LDX", "LPS", "MUL", "MULF", "OR", "RD", "RSUB", "SSK", "STA", "STB", "STCH", "STF", "STI", "STL", "STS", "STSW", "STT", "STX", "SUB", "SUBF", "TD", "TIX", "WD"];
        let format4f=["CADD", "CSUB", "CLOAD", "CSTORE", "CJUMP"];

        if i == 0 {
            self.locctr.insert(1, format!("{:04X}", self.ref_data[0].parse::<usize>().unwrap_or(0)).to_string());
            self.default_locctr.insert(1, format!("{:04X}", self.ref_data[0].parse::<usize>().unwrap_or(0)).to_string());
        }

        else {
            let base_locctr_str: Option<&String> = self.locctr.get(&i);
            let base_locctr: usize = match base_locctr_str {Some(val) => usize::from_str_radix(&val, 16).unwrap_or(0), None => 0,};

            let new_locctr = match instr_type.as_str() {
                "RESW" => base_locctr + ref_type.parse::<usize>().unwrap_or(0) * 3,
                "RESB" => base_locctr + ref_type.parse::<usize>().unwrap_or(0),
                "WORD" | "RSUB" => base_locctr + 3,
                "BYTE" => base_locctr + self.calculate_byte_size(ref_type),
                "BASE" | "LTORG" => string_to_usize(""),
                _ if format1.contains(&instr_type.as_str()) => base_locctr + 1,
                _ if format2.contains(&instr_type.as_str()) => base_locctr + 2,
                _ if format3.contains(&instr_type.as_str()) => base_locctr + 3,
                _ if format4f.contains(&instr_type.as_str()) | instr_type.starts_with('+') => base_locctr + 4, 

                //handling literals
                _ if instr_type.starts_with('=') => {
                    base_locctr + self.calculate_literal_byte_size(instr_type)
                },
                "EQU" => if ref_type.starts_with('*') {base_locctr} 
                else { self.calculate_difference(ref_type)},
                _ => base_locctr + 3, // default size for other instructions
            };

            // update locctr
            let hexalocctr = usize_to_string(new_locctr);

            // handle empty strings
            if hexalocctr == "" {
                let nextlocctr = self.locctr.get(&(i)).unwrap().clone();
                self.locctr.insert(i, hexalocctr.clone());
                self.locctr.insert(i+1, nextlocctr.clone());
            } 
            // if the current instruction is EQU and the reference type is not a literal, then the next locctr should be the same as the current locctr
            else if instr_type.starts_with("EQU") && !ref_type.starts_with('*') {
                let nextlocctr = self.locctr.get(&(i)).unwrap().clone();
                self.locctr.insert(i, hexalocctr.clone());
                self.locctr.insert(i+1, nextlocctr.clone());
            }
            else {
            self.locctr.insert(i+1, hexalocctr.clone());
            }
        }

        let final_locctr_value = self.locctr.get(&i).unwrap();
        writeln!(pass1_file, "{}\t{}\t{}\t{}", final_locctr_value, self.labels[i], instr_type, ref_type).unwrap();
    }

    fn calculate_byte_size(&self, ref_type: &str) -> usize {
        if ref_type.starts_with('X') {
            (ref_type.len() - 3) / 2
        } 
        else if ref_type.starts_with('C') {
            ref_type.len() - 3
        } 
        else {
            0
        }
    }

    fn calculate_literal_byte_size(&self, instr_type: &str) -> usize {
        if instr_type.starts_with('X') {
            (instr_type.len() - 3) / 2
        } 
        else if instr_type.starts_with('C') {
            instr_type.len() - 3
        } 
        else {
            0
        }
    }
}
