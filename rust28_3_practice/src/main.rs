#![allow(unused, warnings)]
// 特征对象
// 当函数返回多个类型时，impl Trait 是无法使用的

// Practice 1 使用 dyn 返回特征
trait Bird{
    fn quak(&self)->String;
}
struct Duck{}
impl Duck {
    fn swim(&self) {
        println!("duck is swimming");
    }
}
struct Swan{}
impl Swan {
    fn fly(&self) {
        println!("swan in flying");
    }
}
impl Bird for Duck {
    fn quak(&self)->String {
        "duck".to_string()
    }
}
impl Bird for Swan {
    fn quak(&self)->String {
        "swan".to_string()
    }
}
fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
    if species == 1 {Box::new(Duck{})} else {Box::new(Swan{})}
}

// Practice 2 数组中使用特征对象
trait Bird1 {
    fn quack(&self);
}
struct Duck1;

impl Duck1 {
    fn fly(&self) {
        println!("duck fly")
    }
}

impl Bird1 for Duck1 {
    fn quack(&self) {
        println!("duck duck")
    }
}
struct Swan1;

impl Swan1 {
    fn fly(&self) {
        println!("swan fly")
    }
}

impl Bird1 for Swan1 {
    fn quack(&self) {
        println!("fly fly")
    }
}
pub struct Screen {
    pub components: Vec<Box<dyn Bird1>>
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter(){
            component.quack()
        }
    }
}


// Practice 3 &dyn and Box<dyn>
trait Draw {
    fn draw(&self) -> String;
}
impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}
fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}
fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}

// Practice4 静态分发和动态分发
trait Foo {
    fn method(&self) -> String;
}
impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}
// 泛型实现
fn static_dispatch<T: Foo>(x: T) {
    x.method();
}
// 通过特征对象实现
fn dynamic_dispatch(y: &dyn Foo) {
    y.method();
}
// 特征对象Box实现
fn dynamic_dispatch_box(y: Box<dyn Foo>) {
    y.method();
}

// Practice5 对象安全
// 返回类型不能为Self, 不能使用泛型参数
trait Mytrait{
    fn f(&self) -> String;
}
impl Mytrait for u32 {
    fn f(&self) -> String {
        "pass".to_string()
    }
}
impl Mytrait for String {
    fn f(&self) -> String {
        "pass".to_string()
    }
}
fn my_function(x: Box<dyn Mytrait>) {
    x.f();
}

fn main() {
    // P1 使用 dyn 返回特征
    let duck = Duck{};
    duck.swim();
    let bird = hatch_a_bird(1);
    assert_eq!(bird.quak(), "duck");
    let bird = hatch_a_bird(2);
    assert_eq!(bird.quak(), "swan");

    // P2 数组中使用特征对象
    // 静态数组
    let birds: [Box<dyn Bird1>; 2] = [Box::new(Duck1),Box::new(Swan1)];
    for bird in birds.iter(){
        bird.quack(); 
    }
    // 动态数组
    let screen = Screen{
        components: vec![Box::new(Duck1),
        Box::new(Swan1)]};
    screen.run();
    // let sc1 = &screen.components[0];
    // sc1.fly()
// 一定要注意，sc1是Bird1的特征对象的实例，而不再是具体类型 
// Duck1的实例，而且sc1的vtable只包含了实现自特征 
// Bird1的那些方法（比如 quack），因此sc1只能调用实现于
// 特征Bird1的quack方法，而不能调用类型 
// Duck1本身实现的方法和类型Duck1实现于其他特征的方法。
// 也就是说，sc1是哪个特征对象的实例,它的vtable中就包含了该特征的方法。
    
    // P3 &dyn and Box<dyn>
    let x = 1.1f64;
    let y = 8u8;
    draw_with_box(Box::new(x));
    draw_with_ref(&y);
    
    // Practice4 静态分发和动态分发
    let x = 5u8;
    let y = "Hello".to_string();
    static_dispatch(x);
    dynamic_dispatch(&y);

    // Practice5 对象安全
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

}
