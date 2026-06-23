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
    pub literals: HashMap<usize, String>,
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
            literals: HashMap::new(),
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

        for i in 0..self.ref_data.len() {
            if self.ref_data[i].starts_with("=") {
                self.literals.insert(i, self.ref_data[i].clone());
            }
        }

        let mut intermediate_file = File::create("outputs/intermediate.txt")?;
        for (label, instr, ref_data) in izip!(&self.labels, &self.instr, &self.ref_data) {
            writeln!(intermediate_file, "{:<10} {:<8} {}", label, instr, ref_data)?;
        }

        Ok(())
    }

    pub fn pass1_generator(&mut self) {
        let mut intermediate_file = File::create("outputs/intermediate.txt").unwrap();
        let mut symbol_table = File::create("outputs/symbTable.txt").unwrap();
        let mut literals_table = File::create("outputs/litTable.txt").unwrap();

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

    fn get_symbol_block(&self, _label: &str, _block_locctrs: &HashMap<String, usize>) -> String {
        "DEFAULTB".to_string()
    }

    fn get_literal_block(&self, _literal: &str, _block_locctrs: &HashMap<String, usize>) -> String {
        "DEFAULTB".to_string()
    }  

    fn initialize_location_counters(&mut self) {
        self.locctr.insert(1, format!("{:04X}", usize::from_str_radix(&self.ref_data[0], 16).unwrap()).to_string());
        self.default_locctr.insert(1, format!("{:04X}", usize::from_str_radix(&self.ref_data[0], 16).unwrap()).to_string());
        self.defaultb_locctr.insert(1, format!("{:04X}", usize::from_str_radix(&self.ref_data[0], 16).unwrap()).to_string());
        self.cdata_locctr.insert(1, format!("{:04X}", usize::from_str_radix(&self.ref_data[0], 16).unwrap()).to_string());
        self.cblks_locctr.insert(1, format!("{:04X}", usize::from_str_radix(&self.ref_data[0], 16).unwrap()).to_string());
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
    
    fn update_locctr(&mut self, i: usize, pass1_file: &mut File, symbol_table: &mut File, literals_table: &mut File) {
        let instr_type = &self.instr[i];
        let ref_type = &self.ref_data[i];
        let labels_type = &self.labels[i];

        let format1 = ["FIX", "FLOAT", "HIO", "SIO", "TIO", "NORM"];
        let format2=["ADDR", "CLEAR", "COMPR", "DIVR", "MULR", "RMO", "SHIFTR", "SHIFTL", "SUBR", "SVC", "TIXR"];
        /*let format3 = ["ADD", "ADDF", "AND", "COMP", "COMPF", "DIV", "J", "JEQ", "JGT", "JLT", "JSUB", "LDA", "LDB", "LDCH", "LDF", "LDL", "LDS", "LDT", "LDX", "LPS", "MUL", "MULF", "OR", "RD", "RSUB", "SSK", "STA", "STB", "STCH", "STF", "STI", "STL", "STS", "STSW", "STT", "STX", "SUB", "SUBF", "TD", "TIX", "WD"];*/
        let format4f=["CADD", "CSUB", "CLOAD", "CSTORE", "CJUMP"];
        let defaultblock = ["DEFAULT"];
        let defaultb = ["DEFAULTB"];
        let cdata = ["CDATA"];
        let cblks = ["CBLKS"];

        if i == 0 {
            self.locctr.write_to_file(pass1_file, i, instr_type, ref_type, labels_type).unwrap();
        }

        else {
            let base_locctr_str: Option<&String> = self.locctr.get(&i);
            let base_locctr: usize = match base_locctr_str {Some(val) => usize::from_str_radix(&val, 16).unwrap_or(0), None => 0,};

            let default_locctr_str: Option<&String> = self.default_locctr.get(&i);
            let default_locctr: usize = match default_locctr_str {Some(val) => usize::from_str_radix(&val, 16).unwrap_or(0), None => 0,};

            let defaultb_locctr_str: Option<&String> = self.defaultb_locctr.get(&i);
            let defaultb_locctr: usize = match defaultb_locctr_str {Some(val) => usize::from_str_radix(&val, 16).unwrap_or(0), None => 0,};

            let cdata_locctr_str: Option<&String> = self.cdata_locctr.get(&i);
            let cdata_locctr: usize = match cdata_locctr_str {Some(val) => usize::from_str_radix(&val, 16).unwrap_or(0), None => 0,};

            let cblks_locctr_str: Option<&String> = self.cblks_locctr.get(&i);
            let cblks_locctr: usize = match cblks_locctr_str {Some(val) => usize::from_str_radix(&val, 16).unwrap_or(0), None => 0,};

            let (new_locctr, new_default_locctr, new_defaultb_locctr, new_cdata_locctr, new_cblks_locctr, default_boolean, defaultb_boolean, cdata_boolean, cblks_boolean) = match instr_type.as_str() {
                _ if format1.contains(&instr_type.as_str()) => (base_locctr + 1, default_locctr + 1, defaultb_locctr, cdata_locctr, cblks_locctr, true, false, false, false),
                _ if format2.contains(&instr_type.as_str()) => (base_locctr + 2, default_locctr + 2, defaultb_locctr, cdata_locctr, cblks_locctr, true, false, false, false),
                _ if format4f.contains(&instr_type.as_str()) | instr_type.starts_with('+') => (base_locctr + 4, default_locctr, defaultb_locctr + 4, cdata_locctr, cblks_locctr, false, true, false, false),
                "WORD" => (base_locctr + 3, default_locctr, defaultb_locctr, cdata_locctr + 3, cblks_locctr, false, false, true, false),
                "BYTE" => (base_locctr + self.calculate_byte_size(ref_type), default_locctr, defaultb_locctr, cdata_locctr + self.calculate_byte_size(ref_type), cblks_locctr, false, false, true, false),
                "RESW" => {
                    (base_locctr + ref_type.parse::<usize>().unwrap_or(0) * 3, default_locctr, defaultb_locctr, cdata_locctr, cblks_locctr + ref_type.parse::<usize>().unwrap_or(0) * 3, false, false, false, true)
                },
                "RESB" => {
                    (base_locctr + ref_type.parse::<usize>().unwrap_or(0) * 1, default_locctr, defaultb_locctr, cdata_locctr, cblks_locctr + ref_type.parse::<usize>().unwrap_or(0) * 1, false, false, false, true)
                },
                //handling literals 
                _ if instr_type.starts_with("=") => (base_locctr + self.calculate_literal_byte_size(instr_type), default_locctr, defaultb_locctr, cdata_locctr + self.calculate_literal_byte_size(instr_type), cblks_locctr, false, false, true, false),
                "EQU" => {
                    if ref_type.starts_with('*') {
                        (base_locctr, default_locctr, defaultb_locctr, cdata_locctr, cblks_locctr, false, false, false, true)
                    } else {
                        let diff = self.calculate_difference(ref_type);
                        (diff, default_locctr, defaultb_locctr, cdata_locctr, diff, false, false, false, true)
                    }
                },
                "BASE" | "LTORG" | "END" => (string_to_usize(""), string_to_usize(""), string_to_usize(""), string_to_usize(""), string_to_usize(""), false, true, false, false),
                _ if defaultblock.contains(&ref_type.as_str()) => (base_locctr, default_locctr, defaultb_locctr, cdata_locctr, cblks_locctr, true, false, false, false),
                _  if defaultb.contains(&ref_type.as_str()) => (base_locctr, default_locctr, defaultb_locctr, cdata_locctr, cblks_locctr, false, true, false, false),
                _  if cdata.contains(&ref_type.as_str()) => (base_locctr, default_locctr, defaultb_locctr, cdata_locctr, cblks_locctr, false, false, true, false),
                _ if cblks.contains(&ref_type.as_str()) => (base_locctr, default_locctr, defaultb_locctr, cdata_locctr, cblks_locctr, false, false, false, true),
                _ => (base_locctr + 3, default_locctr, defaultb_locctr + 3, cdata_locctr, cblks_locctr, false, true, false, false), //default to format 3
            };

            self.locctr.turn_to_hexa(i, new_locctr, instr_type, ref_type);
            self.default_locctr.turn_to_hexa(i, new_default_locctr, instr_type, ref_type);
            self.defaultb_locctr.turn_to_hexa(i, new_defaultb_locctr, instr_type, ref_type);
            self.cdata_locctr.turn_to_hexa(i, new_cdata_locctr, instr_type, ref_type);
            self.cblks_locctr.turn_to_hexa(i, new_cblks_locctr, instr_type, ref_type);

            // checks if sic/xe
            let mut sic: bool = true;
            for i in 0..self.instr.len() {
                if format1.contains(&self.instr[i].as_str()) || format2.contains(&self.instr[i].as_str()) || format4f.contains(&self.instr[i].as_str()) || self.instr[i].starts_with("+") {
                    sic = false;
                    break;
                } 
            }

            if sic == false {
                // logic to check block used then write to file
                if default_boolean == true{
                    self.default_locctr.write_to_file(pass1_file, i, instr_type, ref_type, labels_type).unwrap();
                }
                else if defaultb_boolean == true {
                    self.defaultb_locctr.write_to_file(pass1_file, i, instr_type, ref_type, labels_type).unwrap();
                }
                else if cdata_boolean == true {
                    self.cdata_locctr.write_to_file(pass1_file, i, instr_type, ref_type, labels_type).unwrap();
                }
                else if cblks_boolean == true {
                    self.cblks_locctr.write_to_file(pass1_file, i, instr_type, ref_type, labels_type).unwrap();

                } //else cblks
            }
            else {
                self.locctr.write_to_file(pass1_file, i, instr_type, ref_type, labels_type).unwrap();
            }

            //generate symbol table and literal table
            self.locctr.generate_symbol_table(symbol_table, labels_type, i).unwrap();
            if let Some(literals_type) = self.literals.get(&i) {
                self.locctr.generate_literal_table(literals_table, literals_type, i).unwrap();
            }            
            
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

    fn calculate_literal_byte_size(&self, instr_type: &str) -> usize {
        if instr_type.contains('X') {
            (instr_type.len() - 4) / 2
        } 
        else if instr_type.contains('C') {
            instr_type.len() - 4
        } 
        else {
            0
        }
    }

}
