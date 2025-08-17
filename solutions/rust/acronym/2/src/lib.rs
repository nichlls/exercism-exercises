pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-', '_'])
        .filter(|s| !s.is_empty())
        .flat_map(|word| {
            if word.is_empty() {
                vec![]
            } else {
                let mut chars = vec![];
                let mut prev_lowercase = false;
                for (i, c) in word.chars().enumerate() {
                    if c.is_alphabetic() && (i == 0 || (prev_lowercase && c.is_uppercase())) {
                        chars.push(c.to_ascii_uppercase());
                    }
                    prev_lowercase = c.is_lowercase();
                }
                chars
            }
        })
        .collect()
}
