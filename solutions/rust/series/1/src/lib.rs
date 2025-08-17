pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }

    if digits.is_empty() {
        return vec![];
    }

    let mut substrings: Vec<String> = vec![];

    for i in 0..=digits.len() - len {
        let new = &digits[i..i + len];
        substrings.push(new.to_string());
    }

    substrings
}
