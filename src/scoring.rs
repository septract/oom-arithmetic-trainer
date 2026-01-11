/// Calculate the order of magnitude distance between two numbers
pub fn oom_distance(user_answer: f64, correct_answer: f64) -> f64 {
    if user_answer <= 0.0 || correct_answer <= 0.0 {
        return f64::MAX;
    }

    (user_answer.log10() - correct_answer.log10()).abs()
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScoreResult {
    Exact,      // Within 0.1 OOM
    Close,      // Within 0.5 OOM
    Partial,    // Within 1.0 OOM
    Wrong,      // More than 1.0 OOM off
}

impl ScoreResult {
    pub fn points(&self) -> u32 {
        match self {
            ScoreResult::Exact => 100,
            ScoreResult::Close => 75,
            ScoreResult::Partial => 25,
            ScoreResult::Wrong => 0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ScoreResult::Exact => "Exact!",
            ScoreResult::Close => "Close!",
            ScoreResult::Partial => "Partial",
            ScoreResult::Wrong => "Off",
        }
    }
}

pub fn evaluate(user_answer: f64, correct_answer: f64) -> ScoreResult {
    let distance = oom_distance(user_answer, correct_answer);

    if distance <= 0.1 {
        ScoreResult::Exact
    } else if distance <= 0.5 {
        ScoreResult::Close
    } else if distance <= 1.0 {
        ScoreResult::Partial
    } else {
        ScoreResult::Wrong
    }
}

pub fn format_oom_difference(user_answer: f64, correct_answer: f64) -> String {
    let distance = oom_distance(user_answer, correct_answer);

    if distance < 0.01 {
        "Spot on!".to_string()
    } else {
        let direction = if user_answer > correct_answer {
            "high"
        } else {
            "low"
        };
        format!("{:.1} OOM {}", distance, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oom_distance() {
        assert!((oom_distance(1e6, 1e6) - 0.0).abs() < 0.001);
        assert!((oom_distance(1e6, 1e7) - 1.0).abs() < 0.001);
        assert!((oom_distance(1e6, 1e8) - 2.0).abs() < 0.001);
        assert!((oom_distance(5e6, 1e6) - 0.699).abs() < 0.01);
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate(1e6, 1e6), ScoreResult::Exact);
        assert_eq!(evaluate(2e6, 1e6), ScoreResult::Close);
        assert_eq!(evaluate(5e6, 1e6), ScoreResult::Partial);
        assert_eq!(evaluate(1e8, 1e6), ScoreResult::Wrong);
    }
}
