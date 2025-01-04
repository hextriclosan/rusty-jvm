use crate::error::Error;
use crate::method_area::method_area::with_method_area;

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

pub fn vec_to_i64(value: &[i32]) -> i64 {
    match value.len() {
        1 => value[0] as i64,
        2 => i32toi64(value[0], value[1]),
        _ => panic!("Invalid value length: {}", value.len()),
    }
}

pub fn clazz_ref(class_name: &str) -> crate::error::Result<i32> {
    with_method_area(|area| {
        let clazz_ref = area.load_reflection_class(class_name)?;
        Ok::<i32, Error>(clazz_ref)
    })
}
