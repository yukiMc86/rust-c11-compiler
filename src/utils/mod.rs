/// Parses an integer at the beginning of the string and returns (value, remaining string)
///
/// If the first character is not a digit, returns 0
pub fn parse_number(s: &str) -> (i32, &str) {
    let s = s.trim_start();
    let mut chars = s.chars();

    let mut len = 0;

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            len += 1;
        } else {
            break;
        }
    }

    match len {
        0 => (0, s),
        _ => (s[..len].parse().unwrap(), &s[len..]),
    }
}
