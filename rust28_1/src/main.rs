#![allow(warnings, unused)]


// 泛型和特征是 Rust 中最最重要的抽象类型
// 泛型 Generics
fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    let number_list =vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    
    
    // 结构体中使用泛型
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y:4.0};
    // 枚举中使用泛型
    // 方法中使用泛型
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());
    // 除了结构体中的泛型参数，我们还能在该结构体的方法中定义额外的泛型参数
    let p1 = Point1{x:5, y:10.4};
    let p2 = Point1{x: "hello", y:"c"};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // 为具体的泛型类型实现方法
    let p_float = Point1{x: 1f32, y: 2f32};
    println!("{}",p_float.distance_from_origin());
    // const 泛型（Rust 1.51 版本引入的重要特性）
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32;2] = [1,2];
    display_array(arr);
}   

fn add<T: std::ops::Add<Output=T>>(a:T, b:T) -> T {
    a + b
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// x 和 y 是相同的类型
struct Point<T> {
    x: T,
    y: T,
}
// 卧龙 
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
// 凤雏
enum Result<T, E> {
    Ok(T),
    Error(E),
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T{
        &self.y
    }
}

// 泛型参数的名称你可以任意起
#[derive(Debug)]
struct Point1<T, U> {
    x: T,
    y: U,
}
impl<T,U> Point1<T,U> {
    fn mixup<V,W>(self, other:Point1<V, W>) -> Point1<T,W>{
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}
// 为具体的泛型类型实现方法
impl Point1<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// const 泛型（Rust 1.51 版本引入的重要特性）,针对值的泛型
// fn display_array(arr: [i32; 3]) {
//     println!("{:?}", arr);
// }
// 修改代码，让 display_array 能打印任意长度的 i32 数组
// fn display_array<T:std::fmt::Debug>(arr:&[T]) {
//     println!("{:?}", arr);
// }
// 针对值的泛型，正好可以用于处理数组长度的问题
//  Rust 的一些数组库，在使用的时候都限定长度不超过 32 
fn display_array<T: std::fmt::Debug, const N:usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

// 假设我们某段代码需要在内存很小的平台上工作，因此需要限制函数参数占用的内存大小
