pub fn find_missing_letter(chars: &[char]) -> char {
    let my_ascii_lower = "abcdefghijklmnopqrstuvwxyz";
    let my_chars = chars
        .iter()
        .map(|d| d.to_lowercase().collect::<Vec<char>>()[0])
        .collect::<Vec<char>>();
    let start = my_ascii_lower
        .chars()
        .position(|d| d == my_chars[0])
        .unwrap();
    let v = my_ascii_lower[start..chars.len() + start]
        .chars()
        .enumerate()
        .filter(|(i, d)| if my_chars[*i] == *d { false } else { true })
        .map(|(_, d)| d)
        .collect::<Vec<char>>();
    if chars[0].is_uppercase() {
        v[0].to_uppercase().collect::<Vec<char>>()[0]
    } else {
        v[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_missing_letter_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}
