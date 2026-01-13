use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
pub struct Challenge {
    pub num1: f64,
    pub num2: f64,
    pub is_division: bool,
}

impl Challenge {
    pub fn answer(&self) -> f64 {
        if self.is_division {
            self.num1 / self.num2
        } else {
            self.num1 * self.num2
        }
    }
}

pub fn get_daily_seed() -> u64 {
    let date = js_sys::Date::new_0();
    let year = date.get_full_year();
    let month = date.get_month() + 1;
    let day = date.get_date();
    let date_str = format!("{year}-{month:02}-{day:02}");

    let mut hasher = DefaultHasher::new();
    date_str.hash(&mut hasher);
    hasher.finish()
}

pub fn generate_challenges(seed: u64, count: usize) -> Vec<Challenge> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..count).map(|_| generate_single(&mut rng)).collect()
}

fn generate_single(rng: &mut ChaCha8Rng) -> Challenge {
    let num1 = generate_number(rng);
    let num2 = generate_number(rng);
    let is_division = rng.gen_bool(0.3); // 30% division problems

    Challenge { num1, num2, is_division }
}

fn generate_number(rng: &mut ChaCha8Rng) -> f64 {
    // Generate exponent between 3 and 9
    let exp: i32 = rng.gen_range(3..=9);

    // Generate mantissa between 1.1 and 9.9 (avoid too-round numbers)
    let mantissa: f64 = rng.gen_range(1.1..9.9);

    // Round to one decimal place
    let mantissa = (mantissa * 10.0).round() / 10.0;

    mantissa * 10_f64.powi(exp)
}

pub fn format_number(n: f64) -> String {
    let abs = n.abs();

    if abs >= 1e12 {
        format!("{:.1} trillion", n / 1e12)
    } else if abs >= 1e9 {
        format!("{:.1} billion", n / 1e9)
    } else if abs >= 1e6 {
        format!("{:.1} million", n / 1e6)
    } else if abs >= 1e3 {
        format!("{:.1} thousand", n / 1e3)
    } else {
        format!("{n:.1}")
    }
}
