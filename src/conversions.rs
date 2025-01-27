use std::collections::HashMap;

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
}

impl LocctrExtensions for HashMap<usize, String> {
    fn turn_to_hexa(&mut self, i: usize, new_locctr: usize, instr_type: &str, ref_type: &str) {
        // convert new_locctr to a hexadecimal string
        let hexalocctr = usize_to_string(new_locctr);

        // handle empty strings
        if hexalocctr.is_empty() {
            if let Some(next_locctr) = self.get(&i).cloned() {
                self.insert(i, hexalocctr.clone());
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
}

