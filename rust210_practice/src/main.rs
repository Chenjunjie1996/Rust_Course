#![allow(unused,warnings)]
#![allow(overflowing_literals)]
use std::fs;
use std::io;
use std::num;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

// From 和 Into 是配对的，我们只要实现了前者，那后者就会自动被实现：
// 只要实现了 impl From<T> for U， 就可以使用以下两个方法:
//  let u: U = U::from(T) 和 let u:U = T.into()
// TryFrom/TryInto


#[derive(Debug)]
struct Number { value: i32, }
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

//当执行错误处理时，为我们自定义的错误类型实现 From 特征是非常有用。
//这样就可以通过 ? 自动将某个错误类型转换成我们自定义的错误类型
#[derive(Debug)]
enum CliError {
    IsError(io::Error),
    ParseError(num::ParseIntError),
}
impl From<io::Error> for CliError {
    fn from(src: io::Error) -> Self {
        Self::IsError(src)
    }
}
impl From<num::ParseIntError> for CliError {
    fn from(src: num::ParseIntError) -> Self {
        Self::ParseError(src)
    }
}
fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // io::Error 转为 CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num = contents.trim().parse()?;
    Ok(num)
}

// TryFrom
#[derive(Debug, PartialEq)]
struct EvenNum(i32);
impl TryFrom<i32> for EvenNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        }else {
            Err(())
        }
    }
}

// 其它转换
struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

// 为自定义类型实现 FromStr 特征
#[derive(Debug, PartialEq)]
struct Point1 {
    x: i32,
    y: i32,
}
impl FromStr for Point1 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')').split(',').collect();
       let x_fromstr = coords[0].parse::<i32>()?;
       let y_fromstr = coords[1].parse::<i32>()?;
       Ok(Point1{x:x_fromstr, y:y_fromstr})
    }
}



fn main() {
    // As 类型转换
    println!("******************AS*******************");
    // 1.
    let decimal = 97.123_f32;
    let integer = decimal as u8;
    // let c1 = decimal as char;
    let c2 = integer as char;
    assert_eq!(integer, 'a' as u8);

    // 2. 溢出
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;
    println!("{}",v); // 1000-256-256-256

    // 3.
    assert_eq!(1000 as u16, 1000);
    assert_eq!(1000 as u8, 232);
    assert_eq!(-1_i8 as u8, 255);
    assert_eq!(-2_i8 as u8, 254);

    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时
    // 转化会直接取正整数取值范围的最大或最小值
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);

    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }

    // 4. 裸指针可以和代表内存地址的整数互相转换
    let mut values = [1,2];
    let p1 = values.as_mut_ptr();
    println!("{:?}",p1);

    let first_address = p1 as usize;
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
    println!("{:?}",first_address);
    println!("{:?}",second_address);
    println!("{:?}",p2);

    // 5
    let arr = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 13*8);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13)
    }


    // From/Into
    println!("******************FROM/INTO*******************");

    let my_str = "hello";
    let st1 = String::from(my_str);
    let st2 = my_str.to_string();
    let st3: String = my_str.into();
    // p1
    let i1:i32 = false.into();
    let i2:i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);
    let i3:i32 = 'a' as i32;
    println!("{}", i3);

    let s:String = 'a'.into();
    let s:String = String::from('a');

    // p2
    let num = Number{ value:30 };
    assert_eq!(num.value, 30);
    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);

    // p3
    match open_and_parse_file("abc") {
        Ok(num) => println!("ok, {}", num),
        Err(e) => println!("err: {:?}", e),
    }

    // p4 TryFrom/TryInto
    //类似于 From 和 Into, TryFrom 和 TryInto 也是用于类型转换的泛型特。
    //与 From/Into 不同, 
    //TryFrom和TryInto 可以对转换后的失败进行处理，然后返回一个 Result。
    let n: i16 = 256;
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };
    assert_eq!(n, 0);

    // p5
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    let result: Result<EvenNum, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5_i32.try_into();
    assert_eq!(result, Err(()));


    // 其它转换
    // P1 将任何类型转换成 String
    let origin = Point { x:0, y:0 };
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    // p2 解析string 使用 parse 方法可以将一个 String 转换成 i32 数字，
    // 这是因为在标准库中为 i32 类型实现了 FromStr: : impl FromStr for i32
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "10".parse().unwrap();
    let from_str: i32 = "20".parse().unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);
    
    // p3 为自定义类型实现 FromStr 特征
    let p = Point1::from_str("(3,4)");
    let p1 = "(3,4)".parse::<Point1>();
    assert_eq!(p.unwrap(), Point1{ x: 3, y: 4} );
    assert_eq!(p1.unwrap(), Point1{ x: 3, y: 4} );

    // Deref特征
    //transmute std::mem::transmute 是一个 unsafe 函数，可以把
    // 一个类型按位解释为另一个类型，其中这两个类型必须有同样的位数( bits )
    // 1.transmute 可以将一个指针转换成一个函数指针，该转换并不具备可移植性，
    // 原因是在不同机器上，函数指针和数据指针可能有不同的位数( size )。
    let pointer = foo as *const();
    let function = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);

}

fn foo() -> i32{
    0
}
