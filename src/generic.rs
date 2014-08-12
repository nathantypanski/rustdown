/// If string `s` starts with `c`, then return the remaining characters
/// after `c` has been trimmed from the beginning, along with the number
/// of occurrences of `c` in the beginning of string `s`.
pub fn starting_chars(s: &str, c: char) -> Option<(String, uint)> {
    let mut result = None;
    let cs = c.to_string();
    let ch = cs.as_slice();
    if s.starts_with(ch) {
        let mut count = 0u;
        let mut found = false;
        let words: String = s.chars().filter_map(
            |letter|
            match letter {
                l if l == c && !found => {
                    count += 1;
                    None
                }
                other => {
                    found = true;
                    Some(other)
                }
            }
        ).collect();
        result = Some((words.as_slice().trim_left_chars(' ').to_string(), count));
    }
    return result;
}

pub fn all_same(c: char, s: &str) -> bool {
    let cs = c.to_string();
    let ch = cs.as_slice();
    if s.starts_with(ch) {
        for badchar in s.as_slice().chars().filter_map(
                    |e| if e == c { None } else { Some(c) }) {
            return false;
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::starting_chars;
    use super::all_same;


    #[test]
    fn test_all_same() {
        assert!(!all_same('-', "- This is a bullet"));
    }

    #[test]
    fn test_starting_chars() {
        assert_eq!(starting_chars("### haha", '#'), Some(("haha".to_string(), 3)));
        assert_eq!(starting_chars("    - bullet", ' '), Some(("- bullet".to_string(), 4)));
    }
}
