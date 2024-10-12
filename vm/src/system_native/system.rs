pub(crate) fn current_time_millis_wrp(_args: &[i32]) -> Vec<i32> {
    let millis = current_time_millis();

    let high = ((millis >> 32) & 0xFFFFFFFF) as i32;
    let low = (millis & 0xFFFFFFFF) as i32;

    vec![high, low]
}
fn current_time_millis() -> i64 {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as i64
}
