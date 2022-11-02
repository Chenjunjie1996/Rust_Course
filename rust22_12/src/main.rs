// 基本类型
// 数值类型: 有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
// 字符串：字符串字面量和字符串切片 &str
// 布尔类型： true和false
// 字符类型: 表示单个 Unicode 字符，存储为 4 个字节
// 单元类型: 即 () ，其唯一的值也是 ()

#![allow(warnings, unused)]
use std::mem::size_of_val;
use std::ops::{Range,RangeInclusive};
fn main() {
    // 1. 数值类型

    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("undefined math action")
    }

    let one_million:i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中第一个值，控制小数位为2位
    println!("{:.2}", forty_twos[0]);

    // 序列Range
    for i in 1..=5{
        println!("{}", i)
    }
    for i in 'a'..='c'{
        println!("{}", i)
    }

    //  2. 字符，布尔，单元类型
    let (a, b, c) = ('z', 'ℤ', '国');
    let heart_eyed_cat = '😻';
    // Rust 的字符只能用 '' 来表示， "" 是留给字符串的
    println!("字符'中'占用了{}字节的内存",std::mem::size_of_val(&c));

    // 3. 布尔
    let t = true;
    let f = false;
    if f {
        println!("false");
    }
    else {
        println!("true");
    }

    // 4. 单元类型
    // 例如常见的 println!() 的返回值也是单元类型 ()。

    // 数值类型Practice
    // 1
    let x = 5;
    let y = x;
    let z = 10;
    println!("{}",y);

    // 2
    let v: u16 = 38_u8 as u16;

    // 3
    let x:u32 =5;
    assert_eq!("u32".to_string(), type_of(&x));

    // 4
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    // 5
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}", v1, v2);

    // 6
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    
    // 7
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    // 8
    assert!(0.1_f32+0.2_f32==0.3_f32);
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);

    // 9
    let mut sum = 0;
    for i in -3..=2{
        sum += i
    }
    assert!(sum == -3);

    let mut sum = 0;
    for i in -3..2{
        sum += i
    }
    assert!(sum == -5);

    for c in 'a'..='z'{
        println!("{}",c as u8);
    }

    // 10
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1,5));

    // 字符，布尔，单元类型Practice
    // 1
    let c1 = 'a';
    let c2 = '中';
    assert_eq!(size_of_val(&c1),4);
    assert_eq!(size_of_val(&c2),4);

    // 2 Rust 的字符只能用 '' 来表示， "" 是留给字符串的
    let c1 = '中';
    print_char(c1);

    // 3
    let f = false;
    let t = true;
    if t {
        println!("Success!")
    }
    if !f {
        println!("Failed!")
    }

    // 4
    let f = true;
    let t = true || false;
    assert_eq!(t, f);
    println!("Success!");
    
    // 5
    let v = ();
    let s = (2,3);
    assert_eq!(v, implicitly_ret_unit());

    // 6
    let unit = ();
    assert!(size_of_val(&unit) == 0);

    // 计算
    // 整数加法
    assert!(1u32 + 2 == 3);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    
    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0_f32); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);
    
    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
}

fn type_of<T>(_: &T) -> String{
    format!("{}", std::any::type_name::<T>())
}

fn print_char(c:char){
    println!("{}", c);
}

fn implicitly_ret_unit(){
    println!("I will return a ()")
}

fn explicitly_ret_unit() -> (){
    println!("I will return a ()")
}