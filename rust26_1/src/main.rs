#![allow(warnings, unused)]
// 模式匹配 
// match 和 if let

// 将模式与 target 进行匹配，即为模式匹配，而模式匹配不仅仅局限于 match
// match target {
//    模式1 => 表达式1,
//    模式2 => {
//        语句1;
//        语句2;
//        表达式2
//    },
//    _ => 表达式3
// }
fn main() {
    // example
    //匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
    //每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
    // X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };

    let target = value_in_cents(Coin::Penny);
    println!("{}",target);

    // 使用match表达式赋值
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "192.168.2.19",
        _ => "192.168.2.10",
    };
    println!("{}", ip_str);

    // 模式绑定
    // state 变量将被绑定 UsState::Alaska 的枚举值
    let target = value_in_cents_us(CoinUs::Quarter(UsState::Alabama));
    println!("{:?}",target);

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(2, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    // for action in actions 所有权转移
    for action in actions.iter(){
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(R, G, B) => {
                println!("color RGB is {}, {}, {}", R, G, B);
            },
        }
    }
    // println!("{:#?}", actions);

}

enum Direction {
    East,
    West,
    North,
    South,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
enum IpAddr {
    Ipv4,
    Ipv6,
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum CoinUs {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分
}
// 希望在模式匹配中，获取到 25 美分硬币上刻印的州的名称
fn value_in_cents_us(coin:CoinUs) -> u8 {
    match coin {
        CoinUs::Penny => 1,
        CoinUs::Nickel => 5,
        CoinUs::Dime => 10,
        CoinUs::Quarter(state) => {
            println!("State from {:?}", state);
            25
        }
    }
}
#[derive(Debug)]
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}