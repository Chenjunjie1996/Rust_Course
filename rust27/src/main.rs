#![allow(warnings, unused)]
// 方法
// Rust 的方法往往跟结构体、枚举、特征(Trait)一起使用
// Rust 的对象定义和方法定义是分离的
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例 
    fn new(x:f64, y:f64, radius:f64) -> Circle{
        Circle { x, y, radius }
    }
    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // fn width(&self) -> bool {
    //     self.width > 0
    // }
    // Rectangle 的字段设置为私有属性，只需把它的 new 和 width 方法设置为公有
    pub fn new(width:u32, height:u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }
    // 多个参数
    fn can_hold(&self, other:&Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangleColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("enum call method")
    }
}


fn main() {
    let circle = Circle::new(1f64, 2f64, 3f64);
    let area = circle.area();
    println!("{}",area);

    let rectangle = Rectangle{width: 10, height: 10};
    println!("{}", rectangle.area());
    // if rectangle.width() {
    //     println!("{}",true);
    // }

    // 一般来说，方法跟字段同名，往往适用于实现 getter 访问器
    let rect1 = Rectangle::new(30, 50);
    println!("{}", rect1.width);
    println!("{}", rect1.width());

    // self、&self 和 &mut self 非常重要
    // &self 其实是 self: &Self 的简写
    // Self 指代被实现方法的结构体类型, self 指代的是结构体实例
    // self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
    // &self 表示该方法对 Rectangle 的不可变借用
    // &mut self 表示可变借用
    // Rust 有一个叫 自动引用和解引用的功能
    
    let rect1 = Rectangle{width:30, height:50};
    let rect2 = Rectangle{width:10, height:40};
    let rect3 = Rectangle{width:60, height:45};
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    // 关联函数，例如 String::from
    // Rust 允许我们为一个结构体定义多个 impl 块
    
    // 为枚举实现方法
    let msgs = [
        Message::Quit,
        Message::Write("hello".to_string()),
    ];
    for msg in msgs.iter(){
        msg.call();
    }
}
