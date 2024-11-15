use dnp3::app::measurement::{DoubleBit};
use rand::{random, Rng};
use rand::distr::Alphanumeric;

pub fn generate_random_bool() -> bool {
    random()
}

pub fn generate_random_float(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn generate_random_double_bit() -> DoubleBit {
    match rand::thread_rng().gen_range(0..4) {
        0 => DoubleBit::Intermediate,
        1 => DoubleBit::DeterminedOff,
        2 => DoubleBit::DeterminedOn,
        _ => DoubleBit::Indeterminate,
    }
}

pub fn generate_random_int(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..=max)
}

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}