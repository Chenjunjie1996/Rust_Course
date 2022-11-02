// 枚举
// 枚举(enum 或 enumeration)允许你通过列举可能的成员来定义一个枚举类型
// 扑克总共有四种花色，而这里我们枚举出所有的可能值
// 枚举类型是一个类型，它会包含所有可能的枚举成员
// 枚举值是该类型中的具体某个成员的实例。
#![allow(warnings, unused)]
use crate::List::*;

// Practice 6
enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}
// 为枚举实现方法
impl List {
    // 创建空链表
    fn new() -> List {
        // 枚举成员 Nil 的类型是 List
        Nil
    }
    // 在老的链表前面新增一个节点，并返回新的链表
    fn prepend(self, elem: u32)-> List{
        Cons(elem, Box::new(self))
    }
    //length
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String{
        match self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }

        }
    }
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}
#[derive(Debug)]
enum PokerCardEasy {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

// example in std
// 枚举成员包含的类型更复杂了，变成了结构体
struct Ipv4Addr {}
struct Ipv6Addr {}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
//该枚举类型代表一条消息，它包含四个不同的成员
enum Message {
    Quit, //未关联数据
    Move {x:i32, y:i32}, //匿名结构体
    Write(String), //包含一个 String 字符串
    ChangeColor(i32, i32, i32), //包含三个 i32
}

// 当然，我们也可以用结构体的方式来定义这些消息：
struct QuitMessage; // 单元结构体
struct MoveMessage{x:i32, y:i32}
struct WriteMessage(String); //元组结构体
struct ChangeColorMessage(i32, i32, i32); //元组结构体

//由于每个结构体都有自己的类型，因此我们无法在需要同一类型的地方进行使用，
//例如某个函数它的功能是接受消息并进行发送，那么用枚举的方式，就可以接收不同的消息
//但是用结构体，该函数无法接受 4 个不同的结构体作为参数。
//从代码规范角度来看，枚举的实现更简洁，代码内聚性更强，不像结构体的实现，分散在各个地方。

// Rust 标准库
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
   // 创建 PokerSuit 枚举类型的两个成员实例
   let heart = PokerSuit::Hearts;
   let diamond = PokerSuit::Diamonds;
   print_suit(heart);
   print_suit(diamond);

    let c1 = PokerCard{
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard{
        suit:PokerSuit::Diamonds,
        value: 12,
    };

    // 简化
    let c1 = PokerCardEasy::Clubs(6);
    let c2 = PokerCardEasy::Spades((9));

    // message
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::ChangeColor(255, 255, 0);
    let m4 = Message::Write("message".to_string());
    println!("{:?},{:?},{:?},{:?}",m1, m2, m3, m4);

    // Option 枚举用于处理空值
    // Rust 抛弃null 改为Option枚举变量来表述
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x= 5;
    let y = Some(5);
    // let sum = x + y; Option<T> 需转化为T

    // match 表达式就是这么一个处理枚举的控制流结构：它会根据
    // 枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?},{:?}", six, none);

    // Practice
    // 1
    println!("{:?}", Number1::Zero as u8);
    println!("{:?}", Number1::One as u8);
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    // 2
    let msg1 = Message2::Move { x: 1, y: 2 };
    let msg2 = Message2::Write(String::from("rust"));
    println!("{:?},{:?}",msg1, msg2);

    //3
    let msg = Message2::Move { x: 1, y: 1 };
    if let Message2::Move { x, y } = msg {
        assert_eq!(x, y);
    }else {
        panic!("error");
    }

    //4 使用枚举对类型同一化
    let msgs = [
        Message2::Quit,
        Message2::Move { x: 1, y: 3 },
        Message2::ChangeColor(255, 255, 0),
    ];
    for msg in msgs {
        show_message(msg);
    }

    // Option<T> 枚举处理空值
    let five = Some(5);
    let seven = plus_two(five);
    let none = plus_two(None);

    // 若 let 将 seven 解构成 Some(n)，则执行
    if let Some(n) = seven{
        println!("{}", n);
    }else{
        panic!("error");
    }

    // 6 枚举实现链表
    let mut list = List::new();
    for i in 1..4{
        list = list.prepend(i);
    };
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

// 1
enum Number {
    Zero,
    One,
    Two,
}
enum Number1 {
    Zero = 0,
    One,
    Two,
}
// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

// 2
#[derive(Debug)]
enum Message2 {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

// 4
fn show_message(msg: Message2) {
    println!("{:?}", msg);
}

//5 
fn plus_two(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x+2),
    }
}