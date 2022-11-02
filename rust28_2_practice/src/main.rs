#![allow(unused, warnings)]
// 我们可以使用特征通过抽象的方式来定义这种共享行为
// 还可以使用特征约束来限定一个泛型类型必须要具有某个特定的行为。
use std::{ops::{Add, Sub}, ops::Mul, clone, fmt::Display};

// example
struct Sheep {naked: bool, name: String}
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
        }
        self.naked = true;
    }
}
trait Animal {
    // 关联函数签名；`Self` 指代实现者的类型
    fn new(name:String) -> Self;
    // 方法签名
    fn name(&self) -> String;
    fn noise(&self) -> String;
    // 方法默认实现
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
}
impl Animal for Sheep {
    fn new(name:String) -> Sheep {
        Sheep{name, naked: false}
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn noise(&self) -> String {
        if self.is_naked() {
            "baaaaaah?".to_string()
        } else {
            "baaaaaah!".to_string()
        }
    }
}

// Practice 1
trait Hello {
    fn say_hi(&self) -> String{
        "hi".to_string()
    }
    fn say_something(&self) -> String;
}
#[derive(Debug)]
enum Subject {
    Chinese,
    Math,
    English,
}
struct Student {
    name: String,
    age: u8,
}
#[derive(Debug)]
struct Teacher {
    name: String,
    age: u8,
    subject: Subject,
}
impl Hello for Student {
    fn say_something(&self) -> String {
        "I am a student".to_string()
    }
}
impl Display for Teacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i am a {}, {} ,{:?} teacher", self.name, self.age, self.subject)
    }
}
impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Subject::Chinese => write!(f, "Chinese"),
            Subject::Math => write!(f, "Math"),
            Subject::English => write!(f, "English"),
        }
    }
}
impl Hello for Teacher {
    fn say_something(&self) -> String {
        "i am a ".to_string() +  &self.subject.to_string() + " teacher "
    }
}

// P2
// `Centimeters`, 一个元组结构体，可以被比较大小
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);
// `Inches`, 一个元组结构体可以被打印
#[derive(Debug)]
struct Inches(i32);
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Self(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}
#[derive(Debug,PartialEq,PartialOrd)]
struct Seconds(i32);

// P3 *
fn multiply<T: Mul<T, Output=T>>(a:T, b:T) -> T {
    a * b
}

// P4 自定义类型的相加
struct Foo;
struct Bar;
#[derive(PartialEq, Debug)]
struct FooBar;
#[derive(PartialEq, Debug)]
struct BarFoo;
// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, rhs: Bar) -> Self::Output {
        FooBar
    }
}
impl Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, rhs: Bar) -> Self::Output {
        BarFoo
    }
}

// P5 使用特征作为函数参数
trait Summary {
    fn summarize(&self) -> String;
}
#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}
#[derive(Debug)]
struct Weibo {
    username: String,
    content: String
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}
fn summary<T: Summary>(item: &T) {
    println!("{}",item.summarize());
}

// P6 使用特征作为函数返回值
struct Dog {}
struct Cat {}
trait Pet {
    fn noise(&self) -> String {
        "lovely".to_string()
    }
}
impl Pet for Dog {
    fn noise(&self) -> String {
        "wang!".to_string()
    }
}
impl Pet for Cat {
    fn noise(&self) -> String {
        "miao!".to_string()
    }
}
// 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
// 这种返回值方式有一个很大的限制：只能有一个具体的类型
fn random_animal(random_number: f64) -> impl Pet {
    if random_number < 0.5 {
        Dog {}
    } else {
        Dog {}
    }
}
// 动态发布
// rust使用dyn关键字解决。Box返回指向堆的指针。绕过类型不一样的限制。
fn random_animal1(random_number: f64) -> Box<dyn Pet> {
    if random_number < 0.5 {
        Box::new(Cat{})
    } else{
        Box::new(Dog{})
    }
}

// P7 特征约束
fn sum<T: std::ops::Add<Output=T>>(x: T, y: T) -> T {
    x + y
}
fn sum1<T>(x: T, y: T) -> T
where T: std::ops::Add<Output = T>,
{
    x + y
}



struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);

// 9 
fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32 > {
        calculation: T,
        value: Option<u32>,
    }
    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher { calculation, value: None }
        }
        fn value(&mut self, arg:u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }
    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 11);
    
}

fn example2() {
    // use 'where' to constrain 'T'
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher { calculation, value: None }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(20), 21);
    assert_eq!(cacher.value(25), 21);
}

fn main() {
    // 这里的类型注释时必须的
    let mut dolly: Sheep = Animal::new("Dolly".to_string());
    // TODO ^ 尝试去除类型注释，看看会发生什么
    dolly.talk();
    dolly.shear();
    dolly.talk();

    // p1 
    let chinese_teacher = Teacher {name:"wang".to_string(), age:36, subject: Subject::Chinese};
    println!("{:?}",chinese_teacher);
    println!("{}", chinese_teacher);
    println!("{}", chinese_teacher.say_hi());
    println!("{}", chinese_teacher.say_something());

    // p2
    let _one_second = Seconds(1);
    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);
    let _this_is_true = (_one_second > _one_second);
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter 
    { "smaller" } else { "bigger" };
    println!("One foot is {} than one meter.", cmp);

    // p3
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    // p4  
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    // P5 使用特征作为函数参数
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };
    summary(&post);
    summary(&weibo);
    println!("{:?}", post);
    println!("{:?}", weibo);

    // P6 使用特征作为函数返回值
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
    
    // P7 特征约束
    assert_eq!(sum(1, 2), 3);

    // P8
    let pair = Pair{
        x: Unit(1),
        y: Unit(3),
    };
    pair.cmp_display();

    // P9
    example1();
    example2();

}
