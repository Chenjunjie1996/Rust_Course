// 变量绑定与解构

// 不可变可以带来安全性,
// 可变变量最大的好处就是使用上的灵活性和性能上的提升。

// fn main() {
//     let x = 5;
//     // x = 6;
//     let mut y = 5;
//     println!("befero y: {}", y);
//     y = 6;
//     println!("x: {}, y: {}", x, y)
// }

// 不可变可以带来安全性,
// 可变变量最大的好处就是使用上的灵活性和性能上的提升。
// 使用下划线开头忽略未使用的变量

// fn main() {
//     let _x = 5;
// }

// 变量解构
// fn main() { 
//     let (a, mut b) = (true, false);
//     // a不可变，b可变
//     println!("a = {}, b = {}", a, b);

//     b = true;
//     assert_eq!(a, b);
// }

// 解构式赋值
// struct Struct{
//     e: i32,
// }

// fn main() { 
//     let (a,b,c,d,e);
//     (a,b) = (1,2);
//     [c, .., d, _] = [1,2,3,4,5];
//     Struct {e, ..} = Struct {e: 5};
//     assert_eq!([1,2,1,4,5], [a,b,c,d,e])
// }

// 变量和常量之间的差异
// const MAX_POINTS: u32 = 100_000;
// fn main(){
//     println!("{}", MAX_POINTS);
// }

// 变量遮蔽
// 这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，
// 两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，
// 而 mut 声明的变量，可以修改同一个内存地址上的值，
// 并不会发生内存对象的再分配，性能要更好。
// fn main() { 
//     let x = 5;
//     // 在main函数的作用域内对之前的x进行遮蔽
//     let x = x + 1;
//     {
//         // 在当前的花括号作用域内，对之前的x进行遮蔽
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }
//     println!("The value of x is: {}", x);
// }

// Practice
fn main() { 
    let x = 3;
    let _y = 2;
    println!("x is equal to {}", x);

    let mut z = 1;
    z += 1;
    println!("z = {}", z);

    let a = 10;
    {
        let a = 5;
        println!("a的值是: {}", a);
    }
    println!("a的值是: {}", a);

    let b = 5;
    {
        let b = 12;
        assert_eq!(b, 12);
    }
    assert_eq!(b, 5);
    let b = 42;
    println!("{}", b);

    let mut _c = 1;
    _c = 7;
    _c += 3;

    let _d = 4;
    let _d = "I can alsoe be bound to text!";

    // 解构
    let (mut e, f) = (1, 2);
    e += 2;
    assert_eq!(e, 3);
    assert_eq!(f, 2);

    // 解构式赋值
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);

    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}