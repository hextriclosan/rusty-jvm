// use crate::stack::stack_frame::i32toi64;
// 
// pub(crate) fn subtract_exact_wrp(args: &[i32]) -> Vec<i32> {
//     let i = i32toi64(args[1], args[0]);
//     let j = i32toi64(args[3], args[2]);
//     
//     let millis = subtract_exact(i, j);
// 
//     let low = millis as i32;
//     let high = (millis >> 32) as i32;
// 
//     vec![high, low]
// }
// fn subtract_exact(first: i64, second: i64) -> i64 {
//     first.checked_sub(second).expect("Subtraction overflow")
// }
