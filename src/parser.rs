/// Parse user input into a numeric value
/// Accepts formats like:
/// - "400 billion", "400B", "400b"
/// - "4e11", "4E11"
/// - "4 × 10^11", "4 * 10^11", "4x10^11"
/// - Plain numbers: "400000000000"
pub fn parse_answer(input: &str) -> Option<f64> {
    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return None;
    }

    // Try scientific notation first (4e11, 4E11)
    if let Some(val) = parse_scientific(&input) {
        return Some(val);
    }

    // Try word suffixes (million, billion, etc.)
    if let Some(val) = parse_word_suffix(&input) {
        return Some(val);
    }

    // Try letter suffixes (K, M, B, T)
    if let Some(val) = parse_letter_suffix(&input) {
        return Some(val);
    }

    // Try caret notation (4 × 10^11)
    if let Some(val) = parse_caret_notation(&input) {
        return Some(val);
    }

    // Try plain number
    input.replace(",", "").replace(" ", "").parse().ok()
}

fn parse_scientific(input: &str) -> Option<f64> {
    // Handle 4e11, 4E11 format
    if input.contains('e') {
        return input.parse().ok();
    }
    None
}

fn parse_word_suffix(input: &str) -> Option<f64> {
    let suffixes = [
        ("trillion", 1e12),
        ("billion", 1e9),
        ("million", 1e6),
        ("thousand", 1e3),
    ];

    for (suffix, multiplier) in suffixes {
        if input.ends_with(suffix) {
            let num_part = input.strip_suffix(suffix)?.trim();
            let num: f64 = num_part.parse().ok()?;
            return Some(num * multiplier);
        }
    }
    None
}

fn parse_letter_suffix(input: &str) -> Option<f64> {
    let last_char = input.chars().last()?;
    let multiplier = match last_char {
        't' => 1e12,
        'b' => 1e9,
        'm' => 1e6,
        'k' => 1e3,
        _ => return None,
    };

    // Make sure it's not just a word ending in these letters
    let num_part = &input[..input.len() - 1].trim();
    if num_part.is_empty() {
        return None;
    }

    // Check if the part before is actually a number
    let num: f64 = num_part.parse().ok()?;
    Some(num * multiplier)
}

fn parse_caret_notation(input: &str) -> Option<f64> {
    // Handle formats like "4 × 10^11", "4 * 10^11", "4x10^11", "4 x 10^11"
    let input = input
        .replace("×", "x")
        .replace("*", "x")
        .replace(" ", "");

    if !input.contains("x10^") {
        return None;
    }

    let parts: Vec<&str> = input.split("x10^").collect();
    if parts.len() != 2 {
        return None;
    }

    let mantissa: f64 = parts[0].parse().ok()?;
    let exponent: i32 = parts[1].parse().ok()?;

    Some(mantissa * 10_f64.powi(exponent))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scientific() {
        assert_eq!(parse_answer("4e11"), Some(4e11));
        assert_eq!(parse_answer("4E11"), Some(4e11));
        assert_eq!(parse_answer("3.5e6"), Some(3.5e6));
    }

    #[test]
    fn test_word_suffix() {
        assert_eq!(parse_answer("400 billion"), Some(400e9));
        assert_eq!(parse_answer("3.5 million"), Some(3.5e6));
        assert_eq!(parse_answer("50 thousand"), Some(50e3));
    }

    #[test]
    fn test_letter_suffix() {
        assert_eq!(parse_answer("400B"), Some(400e9));
        assert_eq!(parse_answer("3.5M"), Some(3.5e6));
        assert_eq!(parse_answer("50K"), Some(50e3));
    }

    #[test]
    fn test_caret_notation() {
        assert_eq!(parse_answer("4 × 10^11"), Some(4e11));
        assert_eq!(parse_answer("4 * 10^11"), Some(4e11));
        assert_eq!(parse_answer("4x10^11"), Some(4e11));
    }

    #[test]
    fn test_plain_number() {
        assert_eq!(parse_answer("400000000000"), Some(400000000000.0));
        assert_eq!(parse_answer("400,000,000,000"), Some(400000000000.0));
    }
}
