#![allow(unused, warnings)]
// Box<T>包裹的值会被强制分配在堆上
trait Draw {
    fn draw(&self) -> String;
}
impl Draw for u8 {
    fn draw(&self) -> String {
        println!("u8: {}", *self);
        format!("u8: {}", *self)
    }
}
impl Draw for f64 {
    fn draw(&self) -> String {
        println!("f64: {}", *self);
        format!("f64: {}", *self)
    }
}
// 若T实现了 Draw 特征，则调用该函数时传入的 Box<T> 可以被
// 隐式转换成函数参数签名中的 Box<dyn Draw>
// 由于实现了 Deref 特征，Box 智能指针会自动解引用为
// 它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
fn draw1(x: Box<dyn Draw>) {
    x.draw();
}
fn draw2(x: &dyn Draw) {
    x.draw();
}

// 可以通过 & 引用或者 Box<T> 智能指针的方式来创建特征对象。


fn main() {
    let x = 1.1f64;
    let y = 8u8;
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}
