use std::env;
use vm::vm::VM;

const PATH: &str = "tests/test_data";

pub fn setup() -> VM {
    env::set_current_dir(PATH).expect("Failed to change working directory");
    VM::new("std").expect("Failed to create VM")
}

#[allow(dead_code)]
pub fn get_int(locals: Option<Vec<i32>>) -> i32 {
    *locals.unwrap().last().unwrap()
}

#[allow(dead_code)]
pub fn get_long(locals_opt: Option<Vec<i32>>) -> i64 {
    let locals = locals_opt.unwrap();

    let two = &locals[locals.len().saturating_sub(2)..];
    let low = two[0];
    let high = two[1];

    let high_i64 = (high as i64) << 32;
    let low_i64 = low as u32 as i64;

    high_i64 | low_i64
}

#[allow(dead_code)]
pub fn get_float(locals: Option<Vec<i32>>) -> f32 {
    let value = *locals.unwrap().last().unwrap();

    f32::from_bits(value as u32)
}

#[allow(dead_code)]
pub fn get_double(locals_opt: Option<Vec<i32>>) -> f64 {
    let locals = locals_opt.unwrap();

    let two = &locals[locals.len().saturating_sub(2)..];
    let low = two[0] as u32;
    let high = two[1] as u32;

    let high_i64 = (high as u64) << 32;
    let low_i64 = low as u64;

    f64::from_bits(high_i64 | low_i64)
}
