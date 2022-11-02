#![allow(unused,warnings)]
// unwinding 和 abort
// 栈展开和直接终止
fn main() {
    // 2
    assert_eq!("abc".as_bytes(), [96, 97, 98]);
    let v = vec![1, 2, 3, 4];
    let ele = v[3];
    let ele = v.get(3).unwrap();
    let v = production_rate_per_hour(2);
    divide(15, 1)
}

fn production_rate_per_hour(arg: i32) -> f64 {
    let cph = 221;
    match arg {
        1..=4 => arg as f64 * cph as f64,
        5..=8 => arg as f64 * cph as f64 * 0.9,
        9..=10 => arg as f64 * cph as f64 * 0.77,
        _ => 0 as f64,
    }
}
fn divide(x:u8, y:u8) {
    println!("{}", x / y)
}
pub fn working_items_per_minute(arg: u8) -> u32 {
    (production_rate_per_hour(arg.into()) / 60 as f64) as u32
}