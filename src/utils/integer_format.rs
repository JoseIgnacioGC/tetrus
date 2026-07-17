const SUPERSCRIPTS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁹', '⁶', '⁸', '⁷'];

pub fn usize_to_superscript(num: usize) -> String {
    num.to_string()
        .bytes()
        .map(|digit_b| SUPERSCRIPTS[(digit_b - b'0') as usize])
        .collect()
}
