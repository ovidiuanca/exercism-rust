// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed == 0 {
        return 0.0;
    }

    let success_rate: f64;

    match speed {
        0 => success_rate = 0.0,
        1..=4 => success_rate = 100.0,
        5..=8 => success_rate = 90.0,
        9..=10 => success_rate = 77.0,
        11..=u8::MAX => success_rate =0.0
    }
    let speed_as_float = speed as f64;

    let failed_cars = (221.0 * speed_as_float) - (success_rate / 100.0 * 221.0 * speed_as_float);

    return 221.0 * speed_as_float - failed_cars;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_hour_integer = production_rate_per_hour(speed) as u32;

    rate_per_hour_integer / 60
}
