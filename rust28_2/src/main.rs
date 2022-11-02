#![allow(warnings, unused)]
// 特征
// 特征定义了一个可以被共享的行为，只要实现了特征，你就能使用该行为。
// 如果不同的类型具有相同的行为，那么我们就可以定义一个特征，
// 然后为这些类型实现该特征。定义特征是把一些方法组合在一起，
// 目的是定义一个实现某些目标所必需的行为的集合。

use std::fmt::{Display, Debug};

// 定义特征
pub trait Summary {
    // 特征方法的签名
    fn default(&self) -> String;
    // 特征方法
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}
// 为类型实现特征
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}
pub struct Weibo {
    pub username: String,
    pub content: String,
}
impl Summary for Post {
    fn default(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
    // summarize 默认实现
}
impl Summary for Weibo {
    fn default(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
    // 方法重载
    fn summarize(&self) -> String {
        "方法重载".to_string()
    }
}
// 特征如果仅仅是用来实现方法，那真的有些大材小用，
// 使用特征作为函数参数，真正可以让特征大放光彩的地方。
// 实现了Summary特征 的 item 参数。
pub fn notify(item: &impl Summary) { 
    println!("Breaking news! {}", item.summarize())
}
// T: Summary 说明了 T 必须实现 Summary 特征。
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// 函数两个参数是不同的类型
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("item1: &impl Summary, item2: &impl Summary")
}
// 特征约束 强制函数的两个参数是同一类型
// 泛型类型 T 说明了 item1 和 item2 必须拥有同样的类型
pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("notify3<T: Summary>(item1: &T, item2: &T)");
}


// 多重约束
// 除了让参数实现 Summary 特征外，还可以让参数实现 Display 特征以控制它的格式化输出
// 语法糖形式
pub fn notify4(item: &(impl Summary + Display)) {
    println!("item: &(impl Summary + Display)");
}
// 特征约束形式
pub fn notify5<T: Summary + Display>(item: &T){
    println!("<T: Summary + Display>(item: &T)")
}
// 通过这两个特征，就可以使用 item.summarize 方法，以及通过 println!("{}", item) 来格式化输出 item。

impl Display for Post{
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.title, self.author, self.content)
   } 
}

// Where约束
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {3}
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{3}
fn some_function2<T, U>(t: &T, u: &U) -> String
    where T: Summary + Display,
          U: Summary,
{"Where约束".to_string()}

// 特征约束，可以让我们在指定类型 + 指定特征的条件下去实现方法
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self{
        Self {
            x,
            y
        }
    }
}
// cmp_display 方法，并不是所有的 Pair<T> 结构体对象都可以拥有
// 只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 函数返回中的 impl Trait
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        ),
    }
}
// 虽然我们知道这里是一个 Weibo 类型，但是对于 returns_summarizable
// 调用者而言，他只知道返回了一个实现了 Summary 特征的对象
// impl Trait 形式的返回值，在一种场景下非常非常有用，那就是返回的真实类型非常复杂
// 以用 impl Trait 的方式简单返回

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 通过 derive 派生特征

// 调用方法需要引入特征

fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.default());
    println!("{}",post.summarize());
    println!("{}",weibo.default());
    println!("{}",weibo.summarize());

    // 使用特征作为函数参数 
    // 任何实现了 Summary 特征的类型作为该函数的参数 weibo post
    notify(&post);
    notify(&weibo);

    notify2(&post, &weibo);
    let post1 = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    notify3(&post, &post1);

    println!("{}",post1);
    notify4(&post1);
    notify5(&post1);

    println!("{}", some_function2(&post1, &weibo));

    // 特征约束，可以让我们在指定类型 + 指定特征的条件下去实现方法
    let pair = Pair::new(2, 2);
    pair.cmp_display();

    // 可以通过 impl Trait 来说明一个函数返回了一个类型，该类型实现了某个特征
    println!("{}",returns_summarizable().summarize());
}
