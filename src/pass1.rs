use std::fs::File;
use std::io::{self, Write, BufRead};
use std::collections::HashMap;
pub struct Pass1 {
    pub lines: Vec<String>,
    pub labels: Vec<String>,
    pub instr: Vec<String>,
    pub ref_data: Vec<String>,
}

impl Pass1 {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            labels: Vec::new(),
            instr: Vec::new(),
            ref_data: Vec::new(),
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
                    if parts[0].trim_end_matches(',').to_string() == "*"
                    {
                        self.labels.push(parts[0].trim_end_matches(',').to_string());
                        self.instr.push(parts[1].trim_end_matches(',').to_string());
                        self.ref_data.push("&".to_string());
                    }
                    else
                    {
                        self.labels.push("&".to_string());
                        self.instr.push(parts[0].trim_end_matches(',').to_string());
                        self.ref_data.push(parts[1].trim_end_matches(',').to_string());
                    }
                }
                1 => {
                    self.labels.push("&".to_string());
                    self.instr.push(parts[0].trim_end_matches(',').to_string());
                    self.ref_data.push("&".to_string());
                }
                _ => {}
            }

        }

        Ok(())
    }

    pub fn pass1_generator(&mut self, output_dir: &str) {
        let intermediate_path = format!("{}/intermediate.txt", output_dir);
        let symbol_path = format!("{}/symbTable.txt", output_dir);
        let literal_path = format!("{}/litTable.txt", output_dir);
        
        let mut intermediate_file = File::create(&intermediate_path).unwrap();
        let mut symbol_table = File::create(&symbol_path).unwrap();
        let mut literals_table = File::create(&literal_path).unwrap();

        let mut block_locctrs: HashMap<String, usize> = HashMap::new();
        block_locctrs.insert("DEFAULT".to_string(), 0);
        block_locctrs.insert("DEFAULTB".to_string(), 0);
        block_locctrs.insert("CDATA".to_string(), 0);
        block_locctrs.insert("CBLKS".to_string(), 0);

        let mut current_block = "DEFAULTB".to_string();

        let mut symbols: HashMap<String, (usize, String)> = HashMap::new();
        let mut literals: HashMap<String, (usize, String)> = HashMap::new();
        let mut pending_literals: Vec<String> = Vec::new();

        for i in 0..self.lines.len() {
            let instr_type = self.instr[i].clone();
            let ref_type = self.ref_data[i].clone();
            let labels_type = self.labels[i].clone();

            if instr_type == "START" {
                writeln!(intermediate_file, "{:<10} {:<9} {}", labels_type, instr_type, ref_type).unwrap();
                continue;
            }

            if instr_type == "USE" {
                let valid_blocks = ["DEFAULT", "DEFAULTB", "CDATA", "CBLKS"];
                if !valid_blocks.contains(&ref_type.as_str()) {
                    eprintln!("Error: Unidentified block name '{}' at line {}", ref_type, i + 1);
                    std::process::exit(1);
                }
                current_block = ref_type.clone();
                let locctr = *block_locctrs.get(&current_block).unwrap_or(&0);
                writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", locctr), labels_type, instr_type, ref_type).unwrap();
                continue;
            }

            let locctr = *block_locctrs.get(&current_block).unwrap_or(&0);
            let display_label = if labels_type == "&" { String::new() } else { labels_type.clone() };
            
            if labels_type != "&" && labels_type != "*" {
                symbols.insert(labels_type.clone(), (locctr, current_block.clone()));
            }

            if instr_type == "LTORG" {
                writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", locctr), display_label, instr_type, ref_type).unwrap();
                for lit in &pending_literals {
                    let lit_locctr = *block_locctrs.get(&current_block).unwrap_or(&0);
                    if lit.starts_with("=C'") {
                        let _c = lit.chars().nth(3).unwrap_or('A');
                        writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", lit_locctr), "*", lit, "").unwrap();
                        literals.insert(lit.clone(), (lit_locctr, current_block.clone()));
                        *block_locctrs.get_mut(&current_block).unwrap() += 1;
                    } else if lit.starts_with("=X'") {
                        writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", lit_locctr), "*", lit, "").unwrap();
                        literals.insert(lit.clone(), (lit_locctr, current_block.clone()));
                        *block_locctrs.get_mut(&current_block).unwrap() += 1;
                    }
                }
                pending_literals.clear();
                continue;
            }

            if instr_type == "END" {
                writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", locctr), display_label, instr_type, ref_type).unwrap();
                if !pending_literals.is_empty() {
                    current_block = "CDATA".to_string();
                    let mut lit_locctr = *block_locctrs.get(&current_block).unwrap_or(&0);
                    for lit in &pending_literals {
                        if lit.starts_with("=C'") {
                            let _c = lit.chars().nth(3).unwrap_or('A');
                            writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", lit_locctr), "*", lit, "").unwrap();
                            literals.insert(lit.clone(), (lit_locctr, current_block.clone()));
                            lit_locctr += 1;
                            *block_locctrs.get_mut(&current_block).unwrap() += 1;
                        } else if lit.starts_with("=X'") {
                            writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", lit_locctr), "*", lit, "").unwrap();
                            literals.insert(lit.clone(), (lit_locctr, current_block.clone()));
                            lit_locctr += 1;
                            *block_locctrs.get_mut(&current_block).unwrap() += 1;
                        }
                    }
                    pending_literals.clear();
                }
                continue;
            }

            if labels_type == "*" {
                if ref_type.starts_with("=C'") {
                    let _obj = format!("{:02X}", ref_type.chars().nth(3).unwrap_or('A') as usize);
                    writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", locctr), "*", instr_type, ref_type).unwrap();
                    literals.insert(ref_type.clone(), (locctr, current_block.clone()));
                    *block_locctrs.get_mut(&current_block).unwrap() += 1;
                } else if ref_type.starts_with("=X'") {
                    let _obj = format!("{:02X}", usize::from_str_radix(&ref_type[3..ref_type.len()-1], 16).unwrap_or(0));
                    writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", locctr), "*", instr_type, ref_type).unwrap();
                    literals.insert(ref_type.clone(), (locctr, current_block.clone()));
                    *block_locctrs.get_mut(&current_block).unwrap() += 1;
                }
                continue;
            }

            if ref_type.starts_with("=") {
                pending_literals.push(ref_type.clone());
            }

            writeln!(intermediate_file, "{:<6} {:<10} {:<9} {}", format!("{:04X}", locctr), display_label, instr_type, ref_type).unwrap();

            let format1 = ["FIX", "FLOAT", "HIO", "SIO", "TIO", "NORM"];
            let format2 = ["ADDR", "CLEAR", "COMPR", "DIVR", "MULR", "RMO", "SHIFTR", "SHIFTL", "SUBR", "SVC", "TIXR"];
            let format4f = ["CADD", "CSUB", "CLOAD", "CSTORE", "CJUMP"];

            let increment = match instr_type.as_str() {
                _ if format1.contains(&instr_type.as_str()) => 1,
                _ if format2.contains(&instr_type.as_str()) => 2,
                _ if format4f.contains(&instr_type.as_str()) || instr_type.starts_with('+') => 4,
                "WORD" => 3,
                "BYTE" => self.calculate_byte_size(&ref_type),
                "RESW" => ref_type.parse::<usize>().unwrap_or(0) * 3,
                "RESB" => ref_type.parse::<usize>().unwrap_or(0),
                "BASE" | "LTORG" | "END" => 0,
                _ => 3,
            };

            *block_locctrs.get_mut(&current_block).unwrap() += increment;
        }

        let mut block_bases: HashMap<String, usize> = HashMap::new();
        let mut base = 0;
        let blocks = ["DEFAULTB", "CDATA", "CBLKS", "DEFAULT"];
        for block in blocks {
            block_bases.insert(block.to_string(), base);
            base += *block_locctrs.get(block).unwrap_or(&0);
        }

        let mut absolute_symbols: HashMap<String, usize> = HashMap::new();
        for (label, (rel_addr, block)) in symbols {
            let abs_addr = rel_addr + *block_bases.get(&block).unwrap_or(&0);
            absolute_symbols.insert(label, abs_addr);
        }

        let mut absolute_literals: HashMap<String, usize> = HashMap::new();
        for (literal, (rel_addr, block)) in literals {
            let abs_addr = rel_addr + *block_bases.get(&block).unwrap_or(&0);
            absolute_literals.insert(literal, abs_addr);
        }

        for i in 0..self.lines.len() {
            let ref_type = self.ref_data[i].clone();
            let instr_type = self.instr[i].clone();
            
            if instr_type != "START" && instr_type != "USE" && instr_type != "END" && instr_type != "LTORG" && instr_type != "BASE" && instr_type != "WORD" && instr_type != "BYTE" && instr_type != "RESW" && instr_type != "RESB" {
                if !ref_type.starts_with("=") && !ref_type.starts_with("#") && !ref_type.starts_with("@") && ref_type != "&" && !ref_type.starts_with("X'") && !ref_type.starts_with("C'") {
                    let parts: Vec<&str> = ref_type.split(',').collect();
                    for part in parts {
                        let operand = part.trim();
                        if !operand.is_empty() && !operand.chars().all(|c| c.is_digit(10) || c.is_uppercase()) && !operand.contains("'") {
                            if !absolute_symbols.contains_key(operand) && !["A", "X", "L", "B", "S", "T", "F", "Z", "N", "C", "V"].contains(&operand) {
                                eprintln!("Error: Unidentified symbol '{}' at line {}", operand, i + 1);
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
        }

        for (label, addr) in absolute_symbols {
            writeln!(symbol_table, "{:<10} {:04X}", label, addr).unwrap();
        }

        for (literal, addr) in absolute_literals {
            writeln!(literals_table, "{:<10} {:04X}", literal, addr).unwrap();
        }
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
}
