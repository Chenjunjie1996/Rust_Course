#![allow(warnings, unused)]
// example
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}
impl Point {
    // 关联函数
    fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    fn area(&self) -> f64 {
        let Point{ x: x1, y: y1} = self.p1;
        let Point{ x: x2, y: y2} = self.p2;
        ((x1-x2) * (y1-y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    //该方法要求调用者是可变的 `&mut self`是`self: &mut Self`的语法糖
    fn translate(&mut self, x:f64, y:f64) { 
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}
// Pair 持有两个分配在堆上的整数
struct Pair(Box<i32>, Box<i32>);
impl Pair {
    // 该方法会拿走调用者的所有权
    // `self` 是 `self: Self` 的语法糖
    fn destroy(self){
        let Pair(first, second) = self;
        println!("Destroying Pair({},{})", first, second);
    }
}

// Practice 1
struct Rectangle1 {
    width: i32,
    height:i32,
}
impl Rectangle1 {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}
// Practice 2 3
// self 会拿走当前结构体实例(调用对象)的所有权，而 &self 却只会借用一个不可变引用，&mut self 会借用一个可变引用
#[derive(Debug)]
struct TrafficLight {
    color: String,
}
impl TrafficLight {
    // &self 是 self: &Self的语法糖
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }
}
// Practice 4 关联函数
#[derive(Debug)]
struct TrafficLight1 {
    color: String,
}
impl TrafficLight1 {
    pub fn new(color: String) -> Self {
        Self { color }
    }
    pub fn get_state(&self) -> &str {
        &self.color
    }
}
// Practice 5 多个impl 语句块
struct Rectangle2 {
    width: u32,
    height: u32,
}
impl Rectangle2 {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}
impl Rectangle2 {
    fn can_hold(&self, other:&Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Practice 6 为枚举定义方法
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}
impl TrafficLightColor {
    fn color(&self) -> String {
        match *self {
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Green => "green".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
        }
    }
}


fn main() {
    let rectangle = Rectangle {
        // 关联函数的调用 ::
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("perimeter: {}", rectangle.perimeter());
    println!("area: {}", rectangle.area());

    // translate方法要求一个可变对象
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0, 1.0);
    println!("{:?}", square);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // pair.destroy();

    let light = TrafficLight{
        color: "red".to_owned(),
    };
    light.show_state();
    println!("{:?}", light);

    let light = TrafficLight1::new("red".to_string());
    assert_eq!(light.get_state(), "red");

    let colors = [
        TrafficLightColor::Green,
        TrafficLightColor::Red,
        TrafficLightColor::Yellow,
    ];
    for color in colors.iter() {
        println!("{:#?}", color);
        println!("{}", color.color());
    }


}
