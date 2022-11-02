#![allow(unused,warnings)]
// Result<T> 是一个枚举类型用于描述返回的结果或错误，
// Ok(T): 返回一个结果值 T
// Err(e): 返回一个错误，e 是具体的错误值
// 简而言之，如果期待一个正确的结果，就返回 Ok，反之则是 Err。
use std::num::ParseIntError;
use std::fs::File;
use std::io::{self, Read};

// P1 
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}


// P2 ? 跟 unwrap 非常像，但是 ? 会返回一个错误，而不是直接 panic.
fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

// P3
fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s);
    Ok(s)
}

// P4
// map and and_then 是两个常用的组合器( combinator )，
// 可以用于 Result<T, E> (也可用于 Option<T>).
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|num| num+2)
}
fn add_two1(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|num| Ok(num+2))
}

// P5
fn multiply2(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1) => {
            match n2_str.parse::<i32>() {
                Ok(n2) => {
                    Ok(n1 * n2)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e)
    }
}
fn multiply3(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str.parse::<i32>().and_then(|n1| {
        n2_str.parse::<i32>().map(|n2| n1 * n2)
    })
}

fn multiply4(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

// P6 类别名
type Res<T> = Result<T, ParseIntError>;
fn multiply5(n1_str: &str, n2_str: &str) -> Res<i32>{
    n1_str.parse::<i32>().and_then(|n1| {
        n2_str.parse::<i32>().map(|n2| n1*n2)
    })
}
fn multiply6(n1_str: &str, n2_str: &str) -> Res<i32> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn main() {
    // P1
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));
    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    // P2
    assert_eq!(multiply1("3", "4").unwrap(), 12);

    // P3
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());

    // P4
    assert_eq!(add_two("4").unwrap(), 6);
    assert_eq!(add_two1("4").unwrap(), 6);

    // P5
    let result = multiply3("3", "4");
    println!("{:?}", result);
    let result = multiply4("3", "4");
    println!("{:?}", result);

}
