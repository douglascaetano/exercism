pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return input.into();
    }

    let normalized: String = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    let columns: usize = (normalized.len() as f64).sqrt().ceil() as usize;

    let chunks = normalized
        .as_bytes()
        .chunks(columns)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    (0..columns)
        .map(|i| {
            chunks
                .iter()
                .map(|r| r.get(i..=i).unwrap_or(" "))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
