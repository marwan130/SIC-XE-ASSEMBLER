use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use crate::conversions::get_register_value;

const FORMAT1: [&str; 6] = ["FIX", "FLOAT", "HIO", "SIO", "TIO", "NORM"];  
const FORMAT2: [&str; 11] = ["ADDR", "CLEAR", "COMPR", "DIVR", "MULR", "RMO", "SHIFTR", "SHIFTL", "SUBR", "SVC", "TIXR"];  
const FORMAT3: [&str; 41] = ["ADD", "ADDF", "AND", "COMP", "COMPF", "DIV", "J", "JEQ", "JGT", "JLT", "JSUB", "LDA", "LDB", "LDCH", "LDF", "LDL", "LDS", "LDT", "LDX", "LPS", "MUL", "MULF", "OR", "RD", "RSUB", "SSK", "STA", "STB", "STCH", "STF", "STI", "STL", "STS", "STSW", "STT", "STX", "SUB", "SUBF", "TD", "TIX", "WD"];  
const FORMAT4: [&str; 5] = ["CADD", "CSUB", "CLOAD", "CSTORE", "CJUMP"];  

pub struct Pass2 {
    pub labels: Vec<String>,
    pub instr: Vec<String>,
    pub operands: Vec<String>,
    pub symbol_table: HashMap<String, String>,
    pub literal_table: HashMap<String, String>,
    pub object_code: HashMap<usize, String>,
    pub opcode_table: HashMap<String, String>,
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
            opcode_table: Self::create_opcode_table(),
        }
    }

    fn create_opcode_table() -> HashMap<String, String> {  
        let mut table = HashMap::new();
        
        table.insert("FIX".to_string(), "C4".to_string());
        table.insert("FLOAT".to_string(), "C0".to_string());
        table.insert("HIO".to_string(), "F4".to_string());
        table.insert("SIO".to_string(), "F0".to_string());
        table.insert("TIO".to_string(), "E0".to_string());
        table.insert("NORM".to_string(), "C8".to_string());
        
        table.insert("ADDR".to_string(), "90".to_string());
        table.insert("CLEAR".to_string(), "B4".to_string());
        table.insert("COMPR".to_string(), "A0".to_string());
        table.insert("DIVR".to_string(), "9C".to_string());
        table.insert("MULR".to_string(), "98".to_string());
        table.insert("RMO".to_string(), "AC".to_string());
        table.insert("SHIFTR".to_string(), "A8".to_string());
        table.insert("SHIFTL".to_string(), "A4".to_string());
        table.insert("SUBR".to_string(), "94".to_string());
        table.insert("SVC".to_string(), "B0".to_string());
        table.insert("TIXR".to_string(), "B8".to_string());
        
        table.insert("ADD".to_string(), "18".to_string());
        table.insert("ADDF".to_string(), "58".to_string());
        table.insert("AND".to_string(), "40".to_string());
        table.insert("COMP".to_string(), "28".to_string());
        table.insert("COMPF".to_string(), "88".to_string());
        table.insert("DIV".to_string(), "24".to_string());
        table.insert("J".to_string(), "3C".to_string());
        table.insert("JEQ".to_string(), "30".to_string());
        table.insert("JGT".to_string(), "34".to_string());
        table.insert("JLT".to_string(), "38".to_string());
        table.insert("JSUB".to_string(), "48".to_string());
        table.insert("LDA".to_string(), "00".to_string());
        table.insert("LDB".to_string(), "68".to_string());
        table.insert("LDCH".to_string(), "50".to_string());
        table.insert("LDF".to_string(), "70".to_string());
        table.insert("LDL".to_string(), "08".to_string());
        table.insert("LDS".to_string(), "6C".to_string());
        table.insert("LDT".to_string(), "74".to_string());
        table.insert("LDX".to_string(), "04".to_string());
        table.insert("LPS".to_string(), "D0".to_string());
        table.insert("MUL".to_string(), "20".to_string());
        table.insert("MULF".to_string(), "60".to_string());
        table.insert("OR".to_string(), "44".to_string());
        table.insert("RD".to_string(), "D8".to_string());
        table.insert("RSUB".to_string(), "4C".to_string());
        table.insert("SSK".to_string(), "EC".to_string());
        table.insert("STA".to_string(), "0C".to_string());
        table.insert("STB".to_string(), "78".to_string());
        table.insert("STCH".to_string(), "54".to_string());
        table.insert("STF".to_string(), "80".to_string());
        table.insert("STI".to_string(), "D4".to_string());
        table.insert("STL".to_string(), "14".to_string());
        table.insert("STS".to_string(), "7C".to_string());
        table.insert("STSW".to_string(), "E8".to_string());
        table.insert("STT".to_string(), "84".to_string());
        table.insert("STX".to_string(), "10".to_string());
        table.insert("SUB".to_string(), "1C".to_string());
        table.insert("SUBF".to_string(), "5C".to_string());
        table.insert("TD".to_string(), "E0".to_string());
        table.insert("TIX".to_string(), "2C".to_string());
        table.insert("WD".to_string(), "DC".to_string());
        
        table.insert("CADD".to_string(), "58".to_string());
        table.insert("CSUB".to_string(), "5C".to_string());
        table.insert("CLOAD".to_string(), "50".to_string());
        table.insert("CSTORE".to_string(), "54".to_string());
        table.insert("CJUMP".to_string(), "30".to_string());
        
        table
    }

    pub fn detect_instruction_format(&self, instr: &str) -> u8 {
        let instr_upper = instr.to_uppercase();
        
        if instr_upper.starts_with('+') {
            let instr_without_plus = &instr_upper[1..];
            if FORMAT4.contains(&instr_without_plus) {
                return 4;
            }
            if FORMAT3.contains(&instr_without_plus) {
                return 4;
            }
        }
        
        if FORMAT1.contains(&instr_upper.as_str()) {
            return 1;
        }
        
        if FORMAT2.contains(&instr_upper.as_str()) {
            return 2;
        }
        
        if FORMAT4.contains(&instr_upper.as_str()) {
            return 4;
        }
        
        if FORMAT3.contains(&instr_upper.as_str()) {
            return 3;
        }
        
        3
    }

    pub fn get_opcode(&self, instr: &str) -> Option<String> {  
        let instr_upper = instr.to_uppercase();
        
        let instr_key = if instr_upper.starts_with('+') {
            &instr_upper[1..]
        } else {
            instr_upper.as_str()
        };
        
        self.opcode_table.get(instr_key).cloned()
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

    pub fn generate_format1_object_code(&self, instr: &str) -> Option<String> {
        self.get_opcode(instr)
    }

    pub fn generate_format2_object_code(&self, instr: &str, operand: &str) -> Option<String> {
        let opcode = self.get_opcode(instr)?;
        
        let parts: Vec<&str> = operand.split(',').collect();
        let reg1 = if parts.len() >= 1 { parts[0].trim() } else { "" };
        let reg2 = if parts.len() >= 2 { parts[1].trim() } else { "" };
        
        let reg1_val = get_register_value(reg1);
        let reg2_val = get_register_value(reg2);
        
        let register_byte = (reg1_val << 4) | reg2_val;
        
        Some(format!("{:02X}{:02X}", 
            usize::from_str_radix(&opcode, 16).ok()?,
            register_byte))
    }
}