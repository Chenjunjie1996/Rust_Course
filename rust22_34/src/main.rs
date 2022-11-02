#![allow(warnings, unused)]

// 语句和表达式
// Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值
fn add_with_extra(x:i32, y:i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn main() {
    let x = 5;
    let y = 6;
    let z = add_with_extra(x, y);
    println!("{}",z);

    // 语句
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a,c) = ("hi", false);

    //表达式
    //表达式会进行求值，然后返回一个值。例如 5 + 6，在求值后，返回值 11
    //总之，能返回值，它就是表达式
    //表达式不能包含分号。这一点非常重要，一旦你在表达式后加上分号，
    //它就会变成一条语句，再也不会返回一个值，请牢记！

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // 最后，表达式如果不返回任何值，会隐式地返回一个 () 。
    assert_eq!(ret_unit_type(), ());


    // Practice 语句与表达式
    // 1
    let v = {
        let x = 1;
        x + 2
    };
    assert_eq!(v, 3);

    let v = {
        let mut x = 1;
        x += 2
    };
    assert_eq!(v, ());

    // 2
    let v = {
        let x = 3;
        x
    };
    assert!(v == 3);

    // 3
    let s = sum(1, 2);
    assert_eq!(s, 3);


    // 函数
    another_function(5, 6.1);
    let x = plus_five(5);
    println!("The value of x is: {}", x);

    let x = plus_or_minus(5);
    println!("The value of x is: {}", x);

    let x = plus_or_minus(6);
    println!("The value of x is: {}", x);

    // Practice 函数
    print();
    // never_return_1();
    // never_return_2();

    // 发散函数( Diverging function )不会返回任何值
    // 因此它们可以用于替代需要返回任何值的地方

    let b = false;
    let _v = match b {
        true => 1,
        // // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };
    println!("Exercise Failed if printing out this line!");

}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let z = if x % 2 == 1 {"odd"} else {"even"};
}

fn sum(x:i32, y:i32) -> i32{
    x + y
}

fn another_function(x:i32, y:f32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_five(x:i32) -> i32{
    x + 5
}

fn plus_or_minus(x: i32) -> i32{
    if x > 5{
        return x - 5
    }
    x + 5
}

fn print() -> (){
    println!("Hello, World");
}

fn never_return_1() -> ! {
    // implement this function, don't modify fn signatures
    panic!("I return nothing!")
}

fn never_return_2() -> ! {
    loop {
        println!("I return nothing");
        // sleeping for 1 second to avoid exhausting the cpu resource
        std::thread::sleep(std::time::Duration::from_secs(1));   
    }
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    never_return_fn()
}

fn never_return_fn () -> ! {
    // panic!()
    // unimplemented!()
    todo!();
}

