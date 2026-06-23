use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use crate::conversions::{get_register_value, string_to_hex, hex_string_to_hex, integer_to_hex};

const FORMAT1: [&str; 6] = ["FIX", "FLOAT", "HIO", "SIO", "TIO", "NORM"];  
const FORMAT2: [&str; 11] = ["ADDR", "CLEAR", "COMPR", "DIVR", "MULR", "RMO", "SHIFTR", "SHIFTL", "SUBR", "SVC", "TIXR"];  
const FORMAT3: [&str; 41] = ["ADD", "ADDF", "AND", "COMP", "COMPF", "DIV", "J", "JEQ", "JGT", "JLT", "JSUB", "LDA", "LDB", "LDCH", "LDF", "LDL", "LDS", "LDT", "LDX", "LPS", "MUL", "MULF", "OR", "RD", "RSUB", "SSK", "STA", "STB", "STCH", "STF", "STI", "STL", "STS", "STSW", "STT", "STX", "SUB", "SUBF", "TD", "TIX", "WD"];  
const FORMAT4: [&str; 5] = ["CADD", "CSUB", "CLOAD", "CSTORE", "CJUMP"];  

pub struct Pass2 {
    pub labels: Vec<String>,
    pub instr: Vec<String>,
    pub operands: Vec<String>,
    pub locctrs: Vec<usize>,
    pub blocks: Vec<String>,
    pub symbol_table: HashMap<String, String>,
    pub literal_table: HashMap<String, String>,
    pub object_code: HashMap<usize, String>,
    pub opcode_table: HashMap<String, String>,
    pub base_addr: Option<usize>,
    pub current_block: String,
    pub block_bases: HashMap<String, usize>,
    pub program_name: String,
    pub start_addr: usize,
    pub program_length: usize,
}

impl Pass2 {
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            instr: Vec::new(),
            operands: Vec::new(),
            locctrs: Vec::new(),
            blocks: Vec::new(),
            symbol_table: HashMap::new(),
            literal_table: HashMap::new(),
            object_code: HashMap::new(),
            opcode_table: Self::create_opcode_table(),
            base_addr: None,
            current_block: "DEFAULTB".to_string(),
            block_bases: HashMap::new(),
            program_name: String::new(),
            start_addr: 0,
            program_length: 0,
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

    pub fn is_format4f(&self, instr: &str) -> bool {
        let instr_upper = instr.to_uppercase();
        FORMAT4.contains(&instr_upper.as_str())
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

            if parts.is_empty() {
                continue;
            }

            if parts.len() >= 2 && parts[1] == "START" {
                self.labels.push(parts[0].clone());
                self.instr.push("START".to_string());
                self.operands.push(if parts.len() > 2 { parts[2].clone() } else { "0".to_string() });
                self.locctrs.push(0);
                self.blocks.push("DEFAULT".to_string());
            } else if parts.len() >= 4 {
                let locctr = usize::from_str_radix(&parts[0], 16).unwrap_or(0);
                let label = if parts[1] == "&" { "&".to_string() } else { parts[1].clone() };
                let instr = parts[2].clone();
                let operand = if parts.len() > 3 { parts[3].clone() } else { "&".to_string() };
                
                self.labels.push(label);
                self.instr.push(instr.clone());
                self.operands.push(operand.clone());
                self.locctrs.push(locctr);
                
                if instr == "USE" {
                    self.blocks.push(operand);
                } else {
                    self.blocks.push(self.current_block.clone());
                }
            } else if parts.len() == 3 {
                let locctr = usize::from_str_radix(&parts[0], 16).unwrap_or(0);
                self.labels.push("&".to_string());
                self.instr.push(parts[1].clone());
                self.operands.push(parts[2].clone());
                self.locctrs.push(locctr);
                self.blocks.push(self.current_block.clone());
            } else if parts.len() == 2 {
                self.labels.push("&".to_string());
                self.instr.push(parts[0].clone());
                self.operands.push(parts[1].clone());
                self.locctrs.push(0);
                self.blocks.push(self.current_block.clone());
            } else if parts.len() == 1 {
                self.labels.push("&".to_string());
                self.instr.push(parts[0].clone());
                self.operands.push("&".to_string());
                self.locctrs.push(0);
                self.blocks.push(self.current_block.clone());
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

    pub fn detect_addressing_mode(&self, operand: &str) -> (bool, bool, bool) {
        let is_immediate = operand.starts_with('#');
        let is_indirect = operand.starts_with('@');
        let is_indexed = operand.to_uppercase().ends_with(",X");
        
        (is_immediate, is_indirect, is_indexed)
    }

    pub fn calculate_displacement(&self, operand: &str, locctr: usize, base_addr: Option<usize>) -> Option<(i32, bool, bool)> {
        let operand_clean = operand.trim_start_matches('#').trim_start_matches('@').trim_end_matches(",X").trim();
        
        if let Some(target_addr) = self.symbol_table.get(operand_clean) {
            let target = usize::from_str_radix(target_addr, 16).ok()?;
            let pc_next = locctr + 3;
            
            let pc_disp = target as i32 - pc_next as i32;
            let use_pc = pc_disp >= -2048 && pc_disp <= 2047;
            
            if let Some(base) = base_addr {
                let base_disp = target as i32 - base as i32;
                let use_base = base_disp >= 0 && base_disp <= 4095;
                Some((pc_disp, use_pc, use_base))
            } else {
                Some((pc_disp, use_pc, false))
            }
        } else {
            None
        }
    }

    pub fn generate_format3_object_code(&self, instr: &str, operand: &str, locctr: usize, base_addr: Option<usize>) -> Option<String> {
        let opcode = self.get_opcode(instr)?;
        let (is_immediate, is_indirect, is_indexed) = self.detect_addressing_mode(operand);
        
        let (disp, use_pc, use_base) = self.calculate_displacement(operand, locctr, base_addr)?;
        
        let n = if is_immediate { 0 } else { 1 };
        let i = if is_immediate { 1 } else { 0 };
        let x = if is_indexed { 1 } else { 0 };
        let b = if use_base { 1 } else { 0 };
        let p = if use_pc { 1 } else { 0 };
        let e = 0;
        
        let opcode_num = usize::from_str_radix(&opcode, 16).ok()?;
        let opcode_bits = opcode_num >> 2;
        
        let flags = ((n << 5) | (i << 4) | (x << 3) | (b << 2) | (p << 1) | e) as u8;
        
        let disp_bits = if use_base {
            disp as u16 & 0xFFF
        } else if use_pc {
            (disp + 2048) as u16 & 0xFFF
        } else {
            disp as u16 & 0xFFF
        };
        
        let first_byte = (opcode_bits << 4) | ((flags >> 2) as usize);
        let second_byte = (((flags & 0x3) << 4) as usize) | ((disp_bits >> 8) & 0xF) as usize;
        let third_byte = (disp_bits & 0xFF) as usize;
        
        Some(format!("{:02X}{:02X}{:02X}", first_byte, second_byte, third_byte))
    }

    pub fn generate_format4_object_code(&self, instr: &str, operand: &str) -> Option<String> {
        let opcode = self.get_opcode(instr)?;
        let (is_immediate, is_indirect, is_indexed) = self.detect_addressing_mode(operand);
        
        let operand_clean = operand.trim_start_matches('#').trim_start_matches('@').trim_end_matches(",X").trim();
        
        let target_addr = if let Some(addr) = self.symbol_table.get(operand_clean) {
            usize::from_str_radix(addr, 16).ok()?
        } else {
            usize::from_str_radix(operand_clean, 16).ok()?
        };
        
        let n = if is_immediate { 0 } else { 1 };
        let i = if is_immediate { 1 } else { 0 };
        let x = if is_indexed { 1 } else { 0 };
        let b = 0;
        let p = 0;
        let e = 1;
        
        let opcode_num = usize::from_str_radix(&opcode, 16).ok()?;
        let opcode_bits = opcode_num >> 2;
        
        let flags = ((n << 5) | (i << 4) | (x << 3) | (b << 2) | (p << 1) | e) as u8;
        
        let first_byte = (opcode_bits << 4) | ((flags >> 2) as usize);
        let second_byte = (((flags & 0x3) << 4) as usize) | ((target_addr >> 16) & 0xF) as usize;
        let third_byte = (target_addr >> 8) & 0xFF;
        let fourth_byte = target_addr & 0xFF;
        
        Some(format!("{:02X}{:02X}{:02X}{:02X}", first_byte, second_byte, third_byte, fourth_byte))
    }

    pub fn generate_format4f_object_code(&self, instr: &str, operand: &str) -> Option<String> {
        let opcode = self.get_opcode(instr)?;
        
        let parts: Vec<&str> = operand.split(',').collect();
        
        let register = if parts.len() >= 1 { parts[0].trim() } else { "" };
        let memory = if parts.len() >= 2 { parts[1].trim() } else { "" };
        let condition = if parts.len() >= 3 { parts[2].trim() } else { "" };
        
        let reg_val = get_register_value(register);
        
        let condition_flag = match condition.to_uppercase().as_str() {
            "Z" => 0b00,
            "N" => 0b01,
            "C" => 0b10,
            "V" => 0b11,
            _ => 0b00,
        };
        
        let target_addr = if let Some(addr) = self.symbol_table.get(memory) {
            usize::from_str_radix(addr, 16).ok()?
        } else {
            usize::from_str_radix(memory, 16).ok()?
        };
        
        let opcode_num = usize::from_str_radix(&opcode, 16).ok()?;
        
        let first_byte = (((opcode_num & 0xFC) << 2) as u8) | ((reg_val & 0xF) >> 2) as u8;
        let second_byte = ((reg_val & 0x3) << 6) | ((condition_flag & 0x3) << 4) | ((target_addr >> 16) & 0xF) as u8;
        let third_byte = ((target_addr >> 8) & 0xFF) as u8;
        let fourth_byte = (target_addr & 0xFF) as u8;
        
        Some(format!("{:02X}{:02X}{:02X}{:02X}", first_byte, second_byte, third_byte, fourth_byte))
    }

    pub fn handle_literal(&self, literal: &str) -> Option<String> {
        if let Some(addr) = self.literal_table.get(literal) {
            return Some(addr.clone());
        }
        
        if literal.starts_with("=C'") {
            let content = literal.trim_start_matches("=C'").trim_end_matches('\'');
            Some(string_to_hex(content))
        } else if literal.starts_with("=X'") {
            Some(hex_string_to_hex(literal))
        } else if literal.starts_with('=') {
            let value = literal.trim_start_matches('=').parse::<usize>().ok()?;
            Some(format!("{:06X}", value))
        } else {
            None
        }
    }

    pub fn handle_directive(&mut self, instr: &str, operand: &str, locctr: usize) -> Option<String> {
        match instr.to_uppercase().as_str() {
            "WORD" => {
                let value = if operand.starts_with('#') {
                    operand.trim_start_matches('#').parse::<usize>().ok()?
                } else if let Some(addr) = self.symbol_table.get(operand) {
                    usize::from_str_radix(addr, 16).ok()?
                } else {
                    operand.parse::<usize>().ok()?
                };
                Some(integer_to_hex(value, 3))
            }
            "BYTE" => {
                if operand.starts_with("C'") {
                    let content = operand.trim_start_matches("C'").trim_end_matches('\'');
                    Some(string_to_hex(content))
                } else if operand.starts_with("X'") {
                    Some(hex_string_to_hex(operand))
                } else {
                    None
                }
            }
            "RESW" | "RESB" => {
                Some(String::new())
            }
            "BASE" => {
                if let Some(addr) = self.symbol_table.get(operand) {
                    self.base_addr = Some(usize::from_str_radix(addr, 16).ok()?);
                }
                Some(String::new())
            }
            "LTORG" | "EQU" | "START" | "END" => {
                Some(String::new())
            }
            _ => None,
        }
    }

    pub fn handle_memory_block(&mut self, instr: &str, operand: &str) {
        match instr.to_uppercase().as_str() {
            "USE" => {
                self.current_block = operand.to_string();
            }
            _ => {}
        }
    }

    pub fn adjust_address_for_block(&self, addr: usize) -> usize {
        match self.current_block.as_str() {
            "DEFAULT" => addr,
            "DEFAULTB" => addr + 4096,
            "CDATA" => addr + 8192,
            "CBLKS" => addr + 12288,
            _ => addr,
        }
    }

    pub fn generate_header_record(&self) -> String {
        let name_padded = format!("{:<6}", &self.program_name[..self.program_name.len().min(6)]);
        let start_hex = format!("{:06X}", self.start_addr);
        let length_hex = format!("{:06X}", self.program_length);
        format!("H{}{}{}", name_padded, start_hex, length_hex)
    }

    pub fn generate_text_records(&self) -> Vec<String> {
        let mut records = Vec::new();
        let mut sorted_addrs: Vec<_> = self.object_code.keys().cloned().collect();
        sorted_addrs.sort();

        let mut i = 0;
        while i < sorted_addrs.len() {
            let start_addr = sorted_addrs[i];
            let mut current_addr = start_addr;
            let mut obj_code = String::new();
            let mut bytes_count = 0;

            while i < sorted_addrs.len() && bytes_count < 30 {
                let addr = sorted_addrs[i];
                if addr == current_addr {
                    if let Some(code) = self.object_code.get(&addr) {
                        obj_code.push_str(code);
                        bytes_count += code.len() / 2;
                        current_addr += code.len() / 2;
                    }
                    i += 1;
                } else if addr < current_addr {
                    i += 1;
                } else {
                    break;
                }
            }

            if !obj_code.is_empty() {
                let start_hex = format!("{:06X}", start_addr);
                let length_hex = format!("{:02X}", bytes_count);
                records.push(format!("T{}{}{}", start_hex, length_hex, obj_code));
            }
        }

        records
    }

    pub fn generate_end_record(&self) -> String {
        format!("E{:06X}", self.start_addr)
    }

    pub fn write_object_program(&self, output_path: &str) -> io::Result<()> {
        let mut file = File::create(output_path)?;

        writeln!(file, "{}", self.generate_header_record())?;

        for record in self.generate_text_records() {
            writeln!(file, "{}", record)?;
        }

        writeln!(file, "{}", self.generate_end_record())?;

        Ok(())
    }

    pub fn pass2_generator(&mut self, intermediate_path: &str, symbol_path: &str, literal_path: &str, output_path: &str) -> io::Result<()> {
        self.read_intermediate_file(intermediate_path)?;
        self.read_symbol_table(symbol_path)?;
        self.read_literal_table(literal_path)?;

        self.calculate_block_bases();

        for i in 0..self.instr.len() {
            let instr = self.instr[i].clone();
            let operand = if i < self.operands.len() { self.operands[i].clone() } else { "&".to_string() };
            let label = if i < self.labels.len() { self.labels[i].clone() } else { "&".to_string() };

            self.handle_memory_block(&instr, &operand);

            if instr.to_uppercase() == "START" {
                self.program_name = label.clone();
                self.start_addr = usize::from_str_radix(&operand, 16).unwrap_or(0);
                continue;
            }

            if instr.to_uppercase() == "END" {
                if let Some(addr) = self.symbol_table.get(&operand) {
                    self.start_addr = usize::from_str_radix(addr, 16).unwrap_or(self.start_addr);
                }
                continue;
            }

            if instr.to_uppercase() == "USE" {
                continue;
            }

            let locctr = if i < self.locctrs.len() {
                self.locctrs[i]
            } else {
                0
            };

            let block = if i < self.blocks.len() {
                self.blocks[i].clone()
            } else {
                "DEFAULT".to_string()
            };

            let block_base = *self.block_bases.get(&block).unwrap_or(&0);
            let absolute_locctr = locctr + block_base;

            let format = self.detect_instruction_format(&instr);

            if let Some(obj_code) = self.handle_directive(&instr, &operand, absolute_locctr) {
                if !obj_code.is_empty() {
                    self.object_code.insert(absolute_locctr, obj_code);
                }
            } else if operand.starts_with('=') {
                if let Some(obj_code) = self.handle_literal(&operand) {
                    self.object_code.insert(absolute_locctr, obj_code);
                }
            } else {
                let obj_code = if self.is_format4f(&instr) {
                    self.generate_format4f_object_code(&instr, &operand)
                } else {
                    match format {
                        1 => self.generate_format1_object_code(&instr),
                        2 => self.generate_format2_object_code(&instr, &operand),
                        3 => self.generate_format3_object_code(&instr, &operand, absolute_locctr, self.base_addr),
                        4 => self.generate_format4_object_code(&instr, &operand),
                        _ => None,
                    }
                };

                if let Some(code) = obj_code {
                    self.object_code.insert(absolute_locctr, code);
                }
            }
        }

        if let Some(&max_addr) = self.object_code.keys().max() {
            self.program_length = max_addr - self.start_addr + 1;
        }

        self.write_object_program(output_path)?;

        Ok(())
    }

    fn calculate_block_bases(&mut self) {
        let mut block_sizes: HashMap<String, usize> = HashMap::new();
        block_sizes.insert("DEFAULT".to_string(), 0);
        block_sizes.insert("DEFAULTB".to_string(), 0);
        block_sizes.insert("CDATA".to_string(), 0);
        block_sizes.insert("CBLKS".to_string(), 0);

        for i in 0..self.locctrs.len() {
            if i < self.blocks.len() {
                let block = self.blocks[i].clone();
                let locctr = self.locctrs[i];
                let instr = self.instr[i].clone();
                let operand = if i < self.operands.len() { self.operands[i].clone() } else { "&".to_string() };
                
                let increment = match instr.as_str() {
                    "WORD" => 3,
                    "RESW" => operand.parse::<usize>().unwrap_or(0) * 3,
                    "RESB" => operand.parse::<usize>().unwrap_or(0),
                    "BYTE" => {
                        if operand.starts_with("X'") {
                            (operand.len() - 3) / 2
                        } else if operand.starts_with("C'") {
                            operand.len() - 3
                        } else {
                            1
                        }
                    }
                    _ => 3,
                };
                
                let final_locctr = locctr + increment;
                *block_sizes.get_mut(&block).unwrap_or(&mut 0) = (*block_sizes.get(&block).unwrap_or(&0)).max(final_locctr);
            }
        }

        let mut base = 0;
        let blocks = ["DEFAULTB", "CDATA", "CBLKS", "DEFAULT"];
        for block in blocks {
            self.block_bases.insert(block.to_string(), base);
            base += *block_sizes.get(block).unwrap_or(&0);
        }
    }
}