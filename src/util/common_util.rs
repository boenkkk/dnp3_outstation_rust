use dnp3::app::measurement::DoubleBit;
use rand::distr::Alphanumeric;
use rand::{random, Rng};

pub fn generate_random_bool() -> bool {
    random()
}

pub fn generate_random_float(min: Option<f64>, max: Option<f64>) -> f64 {
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(0.0);

    if min >= max {
        eprintln!(
            "Warning: `min` ({}) is not less than `max` ({}). Returning default value 0.0.",
            min, max
        );
        return 0.0;
    }

    rand::rng().random_range(min..max)
}

pub fn get_double_bit(val: &u8) -> DoubleBit {
    match val {
        0 => DoubleBit::Intermediate,
        1 => DoubleBit::DeterminedOff,
        2 => DoubleBit::DeterminedOn,
        _ => DoubleBit::Indeterminate,
    }
}

pub fn generate_random_double_bit() -> DoubleBit {
    match rand::rng().random_range(0..4) {
        0 => DoubleBit::Intermediate,
        1 => DoubleBit::DeterminedOff,
        2 => DoubleBit::DeterminedOn,
        _ => DoubleBit::Indeterminate,
    }
}

pub fn generate_random_int(min: u32, max: u32) -> u32 {
    rand::rng().random_range(min..=max)
}

pub fn generate_random_string(length: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
