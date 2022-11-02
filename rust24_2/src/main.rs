// 元组
#![allow(warnings, unused)]

//元组是由多种类型组合到一起形成的，
//因此它是复合类型，元组的长度和元素顺序是固定的
fn main() {
    let tup = (500, 6.4, 1);
    // 模式匹配解构元组
    let (x, y, z) = tup;
    // 索引访问
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;   
    //示例
    let s1 = String::from("hello");
    let (s2,len)= calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    // Practice
    // 1
    let _t0 = (0, -1);
    let _t1 = (0, (-1 ,1));
    let t = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    // 2
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
    // 3
    // 过长的元组无法被打印输出
    // 4 模式匹配解构元组
    let tup = (1, 6.4, "hello");
    let (x, y, z) =tup;
    assert_eq!(x, 1);
    assert_eq!(y, 6.4);
    assert_eq!(z, "hello");
    // 5 解构式赋值
    let (x, y, z);
    (y, z, x) = (1,2,3);
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
    // 6 元组可以用于函数的参数和返回值
    let (x, y) = sum_multiply((2,3));
    assert_eq!(x, 5);
    assert_eq!(y, 6);
    
}

fn calculate_length(some_string: String) -> (String, usize){
    let length = some_string.len();
    (some_string, length)
}

fn sum_multiply(nums:(i32, i32)) -> (i32, i32){
    (nums.0 + nums.1, nums.0 * nums.1)
}
