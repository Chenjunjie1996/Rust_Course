#![allow(warnings, unused)]

use std::{ops::Add, clone, fmt::Display};
// 为自定义类型实现 + 操作
#[derive(Debug)]
#[derive(Clone, Copy)]
struct Point<T: Add<T, Output = T>> {
    //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}
impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }    
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

// 自定义类型的打印输出 #[derive(Debug)]
// 自定义 std::fmt::Display 特征：
#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state:FileState,
}
impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}
impl File {
    fn new(name: &str) -> File{
        File { name: String::from(name), data: (Vec::new()), state: FileState::Closed }
    }
}



fn main() {
    // 为自定义类型实现 + 操作
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1.clone(), p2.clone()));
    let result = p1.add(p2);
    println!("{:?}", result);

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));

    // 自定义 std::fmt::Display 特征：
    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
}
