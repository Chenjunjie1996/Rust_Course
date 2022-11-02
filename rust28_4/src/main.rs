#![allow(unused,warnings)]

use std::ops::Add;
use std::fmt;
use std::fmt::Display;
// 深入了解特征
// 1. 关联类型
// 关联类型是在特征定义的语句块中，
// 申明一个自定义类型，这样就可以在特征的方法签名中使用该类型
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) ->Option<Self::Item>{
//     }
// }
struct Counter{x:u32, y:u32}
impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
// 对于 next 方法而言，Self 是调用者 c 的具体类型： Counter，
// 而 Self::Item 是 Counter 中定义的 Item 类型: u32。

// 增加可读性
// 使用泛型，导致函数头部也必须增加泛型的声明，
// 而使用关联类型，将得到可读性好得多的代码：
// trait Container<A,B> {
//     fn contains(&self, a: A, b: B) -> bool;
// }
// fn difference<A,B,C>(container: &C) -> i32 
//     where
//     C: Container<A,B> {todo!()}
trait Container {
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool; 
}
fn difference<C: Container>(container: &C){}

// 默认泛型类型参数 例如std::ops::Add
// trait Add<RHS=Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }
// 相同类型相加
#[derive(Debug,PartialEq)]
struct Point { x:i32, y:i32 }
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// 不同类型相加
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// 调用同名的方法
struct Human;
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
// 调用同名方法 没self参数
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

//特征定义中的特征约束
//有时我们会需要让某个特征A能使用另一个特征B的功能(另一种形式的特征约束)，
//这种情况下不仅仅要为类型实现特征A还要为类型实现特征B才行,这就是 supertrait
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4)); 
    }
}
// OutlinePrint: Display很像特征约束只不过用在了特征定义中而不是函数的参数中，
// 在某种意义上来说，这和特征约束非常类似，都用来说明
// 一个特征需要实现另一个特征，如果你想要实现 OutlinePrint 特征，
// 首先你需要实现 Display 特征。
struct Point1 {
    x: i32,
    y: i32,
}
impl OutlinePrint for Point1{
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4)); 
    }
}
impl fmt::Display for Point1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 在外部类型上实现外部特征(newtype)
// 孤儿规则，特征或者类型必需至少有一个是本地的，才能在此类型上定义特征。
struct Wrapper(Vec<String>); // 元组结构体
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}


fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 });

    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
