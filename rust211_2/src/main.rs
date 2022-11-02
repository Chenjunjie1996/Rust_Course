#![allow(unused,warnings)]
use std::fs;
use std::io;
use std::fs::{File, read_to_string};
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    // 对返回值的错误处理
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };
    
    // unwrap 和 except
    // unwrap 简而言之：成功则返回值，失败则 panic
    // expect 也是遇到错误直接 panic重载了错误打印的函数
    // let f = File::open("hello1.txt").unwrap();
    // let f = File::open("hello1.txt").expect("Failed to open hello.txt");

    // 传播错误
    // 实际应用中，大概率会把错误层层上传然后交给调用链的上游函数进行处理，错误传播将极为常见。



}

// 函数从文件中读取用户名，然后将结果进行返回
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? 就是一个宏，它的作用跟上面的 match 几乎一模一样
fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? 不仅仅可以用于 Result 的传播，还能用于 Option 的传播
// 如果 get 的结果是 None，则直接返回 None，
// 如果是 Some(&i32)，则把里面的值赋给 v。
fn first(arr: &[i32]) -> Option<&i32> {
    let v = arr.get(0)?;
    Some(v)
}
/*
代码展示了在链式调用中使用 ? 提前返回 None 的用法，
.next方法返回Option类型：如果返回 Some(&str)，那么继续调用 chars
如果返回 None，则直接从整个函数中返回 None，不再继续进行链式调用。
*/
fn last_char_of_first_line(text: &str) -> Option<char> { 
    text.lines().next()?.chars().last()
}