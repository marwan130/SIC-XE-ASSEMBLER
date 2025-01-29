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

        let mut intermediate_file = File::create("src/intermediate.txt")?;
        for (label, instr, ref_data) in izip!(&self.labels, &self.instr, &self.ref_data) {
            writeln!(intermediate_file, "{}\t{}\t{}", label, instr, ref_data)?;
        }

        Ok(())
    }

    pub fn pass1_generator(&mut self) {
        self.initialize_location_counters();

        let mut pass1_file = File::create("src/out_pass1.txt").unwrap();
        let mut symbol_table = File::create("src/symbTable.txt").unwrap();
        let mut literals_table = File::create("src/litTable.txt").unwrap();
        for i in 0..self.lines.len() {
            self.update_locctr(i, &mut pass1_file, &mut symbol_table, &mut literals_table);
        }
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
