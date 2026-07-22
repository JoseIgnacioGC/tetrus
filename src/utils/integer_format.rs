const SUPERSCRIPTS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

pub fn to_superscript(num: usize) -> String {
    num.to_string()
        .bytes()
        .map(|digit_b| SUPERSCRIPTS[(digit_b - b'0') as usize])
        .collect()
}

pub fn to_superscript_with_separator(num: usize) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| {
            chunk
                .iter()
                .map(|&digit_b| SUPERSCRIPTS[(digit_b - b'0') as usize])
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("·")
}
