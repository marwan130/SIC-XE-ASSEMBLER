pub fn get_register_value(reg: &str) -> u8 {
    match reg.to_uppercase().as_str() {
        "A" => 0,
        "X" => 1,
        "L" => 2,
        "B" => 3,
        "S" => 4,
        "T" => 5,
        "F" => 6,
        "PC" => 8,
        "SW" => 9,
        _ => 0,
    }
}

pub fn string_to_hex(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        result.push_str(&format!("{:02X}", c as u8));
    }
    result
}

pub fn hex_string_to_hex(s: &str) -> String {
    let cleaned = s.trim_start_matches("X'").trim_end_matches('\'').trim();
    cleaned.to_uppercase()
}

pub fn integer_to_hex(value: usize, bytes: usize) -> String {
    match bytes {
        1 => format!("{:02X}", value & 0xFF),
        2 => format!("{:04X}", value & 0xFFFF),
        3 => format!("{:06X}", value & 0xFFFFFF),
        _ => format!("{:X}", value),
    }
}
