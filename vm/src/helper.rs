pub fn i32toi64(high: i32, low: i32) -> i64 {
    let high_converted = (high as i64) << 32;
    let low_converted = low as u32/*to prevent sign extension*/ as i64;

    high_converted | low_converted
}

pub fn i64_to_vec(value: i64) -> Vec<i32> {
    let low = value as i32;
    let high = (value >> 32) as i32;

    vec![high, low]
}