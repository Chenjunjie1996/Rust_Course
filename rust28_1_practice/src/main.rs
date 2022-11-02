#![allow(warnings, unused)]
#![allow(incomplete_features)]

// Practice 1
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

// Practice 2
fn sum<T: std::ops::Add<Output=T>>(x:T, y:T) -> T {
    x + y
}

// Practice345 结构体和impl
struct Point<T, U> {
    x: T,
    y: U,
} 
struct Val<T>{
    val: T,
}
impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

// Practice 67 方法
struct Point1<T,U>{
    x: T,
    y: U,
}
impl<T, U> Point1<T,U> {
    fn mixup<V,W>(self, other: Point1<V, W>) -> Point1<T, W>{
        Point1{
            x: self.x,
            y: other.y,
        }
    }
}
struct Point2<T> {
    x: T,
    y: T,
}
impl Point2<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 针对类型实现的泛型，所有的泛型都是为了抽象不同的类型
// 针对值的泛型 Const 泛型。
// example
//下面的例子同时使用泛型和 const 泛型来实现一个结构体，该结构体的字段中的数组长度是可变的
struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}
impl<T: std::fmt::Debug , const N: usize> ArrayPair<T, N> {
    // ...
}
// 目前，const 泛型参数只能使用以下形式的实参:
// 一个单独的 const 泛型参数
// 一个字面量 (i.e. 整数, 布尔值或字符).
// 一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)
fn foo<const N: usize>() {}
fn bar<T, const M: usize>() {
    foo::<M>();
    foo::<2012>();
    foo::<{20*100+20*10+1}>();
}
// const 泛型还能帮我们避免一些运行时检查，提升性能
pub struct MinSlice<T, const N: usize> {
    pub head: [T; N],
    pub tail: [T],
}
// Practice 1
struct Array<T, const N:usize> {
    data: [T; N]
}
// Practice 2
fn print_array<T:std::fmt::Debug, const N:usize> (arr: [T;N]) {
    println!("{:?}", arr);
}

fn main() {
    reg_fn(S(A));          // 具体的类型
    gen_spec_t(SGen(A));   // 隐式地指定类型参数  `A`.
    gen_spec_i32(SGen(6)); // 隐式地指定类型参数`i32`.
    // 显式地指定类型参数 `char`
    generic::<char>(SGen('a'));
    // 隐式地指定类型参数 `char`.
    generic(SGen('c'));
    
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = Point{x: 5, y : "hello".to_string()};

    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());

    let p1 = Point1 { x: 5, y: 10 };
    let p2 = Point1 { x: "Hello", y: '中'};
    let p3 = p1.mixup(p2);
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    let p = Point2{x: 5.0_f32, y: 10.0_f32};
    println!("{}",p.distance_from_origin());

    // 泛型 example
    let slice = b"Hello, world";
    let reference = slice.get(6);
    assert!(reference.is_some());
    println!("{:?}", slice);

    // Practice 1
    let arrays = [
        Array{
            data:[1,2,3]
        },
        Array{
            data:[1,2,3],
        },
        Array{
            data:[1,2,4]
        },
    ];
    // Practice 2
    let arr = [1, 2, 3];
    print_array(arr);
    let arr = ["hello", "world"];
    print_array(arr);

}
