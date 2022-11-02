#![allow(warnings, unused)]

// 全模式列表 检索查阅（模式匹配在我们的开发中会经常用到）


fn main() {
    // 匹配字面值 希望代码获得特定的具体值
    let x = 1;
    let target = match_value(&x);
    let target = match_value2(&x);
    println!("{}", target);

    // 匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 单分支 多个模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("others"),
    }

    // 匹配值的范围
    let x = 5;
    match x {
        1..=5 => println!("1-5"),
        _ => println!("others"),
    }

    // 解构并分解值
    let point = Point{x:0, y:7};
    let Point{x,y}=point;
    assert_eq!(0,x);
    assert_eq!(7,y);
    let p @Point{x:px, y:py}= Point{x:0, y:0};
    assert_eq!(px, 0);
    assert_eq!(py, 0);
    // 固定某个字段的匹配方式
    let p = Point{ x:0, y:7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 解构枚举
    let msg = Message::ChangeColorRGB(0, 160, 255);
    match msg {
        Message::Quit => println!("QUIT"),
        Message::Move { x, y } => println!("{},{}",x,y),
        Message::Write(text) => println!("{}", text),
        Message::ChangeColorRGB(r, g, b) => println!("{},{},{}",r,g,b),
    }
    
    // 解构嵌套的结构体和枚举
    let msg = Message1::ChangeColorRGB(Color::HSV(0, 160, 255));
    match msg {
        Message1::ChangeColorRGB(Color::HSV(h, s, v)) => {
            println!("hsv: {}, {}, {}", h, s, v);
        },
        _ => println!("rgb"),
    }

    // 解构结构体和元组
    let ((feet, inches), Point{x,y}) = ((3,10), Point{x:3, y:10});

    // 使用 _ 忽略整个值, 用 .. 忽略剩余值
    foo(3, 4);

    let mut setting_value = Some(2);
    let new_setting_value = Some(3);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("cannot overwrite"),
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);

    let condition = iflettest((setting_value, new_setting_value));
    println!("{}", condition);

    // _x 仍会将值绑定到变量，而 _ 则完全不会绑定。
    let s = Some(String::from("rust"));
    if let Some(_) = s {
        println!("found string")
    }
    println!("{:?}", s);

    // 用 .. 忽略剩余值
    let origin = Point {x:0, y:0};
    match origin {
        Point{x, ..} => println!("x is {}", x),
    }

    let numbers = (2,3,4,5,6,7,8,9);
    match numbers {
        (first, .., last) => println!("first is {}, last is {}", first, last),
    }

    // 匹配守卫提供的额外条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than 5: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        // if n==5
        // 匹配守卫中使用外部的 y
        Some(n) if n==y => println!("matched n = {}", n),
        _ => println!("Default case, x= {:?}", x),
    }
    println!("end x = {:?}, y = {:?}", x, y);

    // 可以在匹配守卫中使用 或 运算符 |
    let x = 4;
    let y = false;
    match x {
        // 此分支只匹配 x 值为 4、5 或 6 同时 y 为 true 的情况
        4 | 5 | 6 if !y => println!("yes"),
        _ => println!("no"),
    }

    // @绑定 当你既想要限定分支范围，又想要使用分支的变量时
    let msg = Message2::Hello { id: 5 };
    let res: bool = bangding(msg);
    println!("{}", res);

    // @ 还可以在绑定新变量的同时，对目标进行解构
    let p@Point{x:px, y:py} = Point{x:10, y:23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point{ x:10, y:5 };
    if let p @Point{ x:10, y} = &point { 
        println!("x is 10, y is {} in {:?}", y, p);
    } else {
        println!("x is not 10");
    }
    println!("{:?}", p);

    if let Point{x:10, y} = point {
        println!("x is 10, y is {} in {:?}", y, point);
    }


    // Practice
    // 1
    let n = 7;
    match n {
        1 => println!("one"),
        2|3|4|5 => println!("2-5"),
        6..=10 => println!("6-10"),
        _ => println!(">11"),
    }

    // 2 @
    let p=Point{x:2, y:10};
    match p {
        Point { x, y:0 } => println!("on the x-axis at {}", x),
        Point { x: x@0..=5, y: y@(10 |20| 30) } => println!("x={},y={}",x,y),
        Point { x, y} => println!("x={},y={}",x,y),
    }

    // 3
    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello { id: id @3..=7 } => {
            println!("id range 3-7, {}", id);
        },
        Message2::Hello { id: id@ (10|11|12) } => {
            println!("id =10|11|12, {}", id);
        },
        Message2::Hello { id } => println!("{}", id),
    }

    // 4 匹配守卫
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    // 5 .. 忽略值
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
    match numbers {
        (first,..,last) => {
            assert_eq!(first,2);
            assert_eq!(last,2048);
        }
    }

    // 使用模式 &mut V 去匹配一个可变引用时，你需要格外小心，因为匹配出来的 V 是一个值，而不是可变引用
    let mut v = String::from("hello");
    let r = &mut v;
    match r {
        value => value.push_str(",world!"),
    }

}

fn match_value(x: &i32) {
    match x {
        1 => println!("one"),
        _ => println!("others"),
    }
}

fn match_value2(x: &i32) -> String {
    if let x = 1 {
        println!("1");
        return "one".to_string();
    }
    "others".to_string()
}

#[derive(Debug)]
struct Point {
    x:i32,
    y:i32,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColorRGB(i32,i32,i32),
}
enum Color {
    RGB(i32,i32,i32),
    HSV(i32,i32,i32),
}
enum Message1 {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColorRGB(Color),
}

fn foo(_:i32, y:i32) {
    println!("y = {}", y);
}
fn iflettest(x:(Option<i32>, Option<i32>)) -> bool {
    if let x = ( Some(5), Some(10)) {
        println!("{:?}", x);
        return true;
    } 
    false
}

enum Message2 {
    Hello { id: i32},
}

fn bangding(msg: Message2) -> bool {
    match msg {
        Message2::Hello { id: id @ 3..=7 } => {
            println!("Found target id {}", id);
            true
        },
        Message2::Hello { id: 10..=12 } => {
            // 未绑定变量
            // println!("Found id {}", id);
            false
        },
        Message2::Hello { id } => {
            println!("Found some other id: {}", id);
            false
        }
        Message2::Hello { id } if id < 1 => {
            println!("Found id < 1 {}", id);
            false
        }
        Message2::Hello { id } if id>1 && id<3 => {
            println!("Found 1<id<3: {}", id);
            false
        }
    }
}