/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let trimmed = code.replace(" ", "");
    if trimmed.len() <= 1 || !trimmed.chars().all(|char| char.is_ascii_digit()) {
        return false;
    }
    let sum = trimmed
        .chars()
        .rev()
        .enumerate()
        .map(|(index, char)| {
            let mut digit = char.to_digit(10).unwrap();
            if index % 2 == 1 {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            digit
        })
        .sum::<u32>();

    sum % 10 == 0
}
