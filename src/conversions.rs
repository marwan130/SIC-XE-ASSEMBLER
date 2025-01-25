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

