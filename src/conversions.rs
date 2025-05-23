use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

pub fn string_to_usize(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.parse::<usize>().unwrap_or(0)
    }
}

pub fn usize_to_string(number: usize) -> String {
    if number == 0 {
        String::new()  // return an empty string if the number is 0
    } else {
        format!("{:04X}", number)  // return the number as a hexadecimal string
    }
}

pub trait LocctrExtensions {
    fn turn_to_hexa(&mut self, i: usize, new_locctr: usize, instr_type: &str, ref_type: &str);

    fn write_to_file(&self, pass1_file: &mut File, i: usize, instr_type: &str, ref_type: &str, labels: &str) -> Result<(), io::Error>;

    fn generate_symbol_table(&self, symbol_table: &mut File, labels_type: &str, i: usize) -> Result<(), io::Error>;

    fn generate_literal_table(&self, literal_table: &mut File, labels_type: &str, i: usize) -> Result<(), io::Error>;
}

impl LocctrExtensions for HashMap<usize, String> {
    fn turn_to_hexa(&mut self, i: usize, new_locctr: usize, instr_type: &str, ref_type: &str) {
        // convert new_locctr to a hexadecimal string
        let hexalocctr = format!("{:04X}", new_locctr);

        // handle empty strings
        if instr_type == "BASE" || instr_type == "LTORG" || instr_type == "END"  {
            if let Some(next_locctr) = self.get(&i).cloned() {
                self.insert(i, " ".to_string());
                self.insert(i + 1, next_locctr);
            }
        }
        // if the instruction is equ and ref_type is not a literal, keep the same locctr
        else if instr_type.starts_with("EQU") && !ref_type.starts_with('*') {
            if let Some(next_locctr) = self.get(&i).cloned() {
                self.insert(i, hexalocctr.clone());
                self.insert(i + 1, next_locctr);
            }
        }
        // default case: update the next locctr
        else {
            self.insert(i + 1, hexalocctr.clone());
        }
    }

    fn write_to_file(&self, pass1_file: &mut File, i: usize, instr_type: &str, ref_type: &str, labels: &str) -> Result<(), io::Error> {
        // get the locctr value
        let final_locctr_value = self.get(&i);
            writeln!(pass1_file, "{}\t{}\t{}\t{}", final_locctr_value.unwrap_or(&String::new()), labels, instr_type, ref_type)?;
        Ok(())
    }

    fn generate_symbol_table(&self, symbol_table: &mut File, labels_type: &str, i: usize) -> Result<(), io::Error> {
        if labels_type != "&" && labels_type != "*" {
            let final_locctr_value = self.get(&i);
            writeln!(symbol_table, "{}\t{}\t", labels_type, final_locctr_value.unwrap_or(&String::new()))?;
        }
        Ok(())
    }

    fn generate_literal_table(&self, literals_table: &mut File, literals_type: &str, i: usize) -> Result<(), io::Error> {
        if literals_type.starts_with('=') {
            let final_locctr_value = self.get(&i);
            writeln!(literals_table, "{}\t{}\t", literals_type, final_locctr_value.unwrap_or(&String::new()))?;
        }
        Ok(())
    }
}

