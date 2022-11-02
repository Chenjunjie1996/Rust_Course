#![allow(warnings, unused)]
// 结构体
// 结构体跟之前讲过的元组有些相像：都是由多种类型组合而成。
// 与元组不同的是，结构体可以为内部的每个字段起一个富有含义的名称。

// 创建结构体
#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;
impl AlwaysEqual {
    fn print_unit(&self){
        todo!();
    }
}

#[derive(Debug)]
struct Useryinyong<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Dbgtest {
    index: i32,
    content: String,
}

fn main() {
    // 创建User结构体实例
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // 创建可变的User结构体实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{:?}",user1);

    // 简化结构体创建 fn build_user
    let user1 = build_user(
        String::from("chenjunjie@singleron.com"),
        String::from("chenjunjie"),
    );
    println!("{:?}",user1);

    // 根据已有的结构体实例，创建新的结构体实例
    let user2 = User{
        email: String::from("another@example.com"),
        ..user1
    };
//结构体更新语法跟赋值语句 = 非常相像，
//因此在上面代码中，user1 的部分字段所有权被转移到 user2 中：
//username 字段发生了所有权转移，作为结果，user1 无法再被使用。
// 其中 bool 和 u64 类型就实现了 Copy 特征，因此 active 和 
// sign_in_count 字段在赋值给 user2 时，仅仅发生了拷贝，
// 而不是所有权转移。
//username 所有权被转移给了 user2，导致了 user1 无法再被使用，
// 但是并不代表 user1 内部的其它字段不能被继续使用
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    // println!("{:?}", user1); 无法再使用

    // 结构体的内存排列
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
 
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    // 元组结构体
    // 结构体必须要有名称，但是结构体的字段可以没有名称
    // 元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // 单元结构体
    // 定义一个类型，但是不关心该类型的内容, 只关心它的行为时
    let subject = AlwaysEqual;

    // 结构体数据的所有权
    // 可以让 User 结构体从其它对象借用数据
    // 生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小。
    let user1 = Useryinyong {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    let user2 = Useryinyong {
        active: user1.active,
        username: user1.username,
        email:"another@example.com",
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    println!("{:?}", user1);

    // 使用 #[derive(Debug)] 来打印结构体的信息
    // 当结构体较大时，我们可能希望能够有更好的输出表现，
    // 此时可以使用 {:#?} 来替代 {:?}
    println!("{:#?}", user1);

    // dbg! 输出到标准错误输出 stderr，而 println! 输出到标准输出 stdout
    dbg!(&user1);
    //面的例子中清晰的展示了 dbg! 如何在打印出信息的同时，还把表达式的值赋给了 width
    let scale = 2;
    let rect1 = Rectange {
        width: dbg!(30*scale),
        height: 50,
    };
    dbg!(rect1);
    // println!("{:#?}", rect1);

    //  对比
    let index = 1;
    let content = String::from("PIS");
    let dbgtest1 = Dbgtest {
        index: dbg!(index*2),
        content: dbg!(content + "20"),
    };
    println!("{}",index);
    dbg!(dbgtest1);
    // println!("{}",content);

    // Practice
    // 1
    let age = 30;
    let p = Person{
        name: String::from("sunface"),
        age,
        hobby: String::from("basketball"),
    };

    // 2
    let u = Unit;
    do_sth_with_unit(u);

    // 3 元组结构体
    let v = Point(0, 127, 255);
    check_color(v);

    // 4 结构体上的一些操作
    let age = 18;
    let mut p = Person{
        name: String::from("sunface"),
        age,
        hobby: String::from("basketball"),
    };
    p.age=30;
    p.name = String::from("sunfei");
    println!("{:#?}",p);

    // 5使用结构体字段初始化缩略语法可以减少一些重复代码
    p = build_person("name".to_string(), 16, "hobby".to_string());
    println!("{:#?}", p);

    //6 你可以使用结构体更新语法基于一个结构体实例来构造另一个
    let user1 = User{
        email:String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = set_email(user1);
    println!("{:#?}", user2);

    // 7 
    let scale = 2;
    let rect1 = Rectangle {
        // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr
    println!("{:#?}", rect1);

    // 结构体的所有权
    let p = Person8{
        name: String::from("Alice"),
        age: Box::new(20),
    };
    let Person8{name, ref age} = p;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    // println!("The person struct is {:?}", p);
    println!("The person's age from person struct is {}", p.age);
    // println!("The person's name from person struct is {}", p.name);

    // 8
    let f = File8 {
        name: String::from("readme.md"),
        data: "Rust by practice".to_string(),
    };
    let _name = f.name;
    println!("{}, {}", _name, f.data);

}

fn build_user(email:String, username:String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Practice
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

struct Unit;
trait ATrait {
    // ...定义一些行为
}
impl ATrait for Unit {  }

fn do_sth_with_unit(u: Unit) {}

fn check_color(p: Point) {
    let Point(x, y, z) = p;
    assert_eq!(x,0);
    assert_eq!(y,127);
    assert_eq!(z,255);
}

//5 结构体字段初始化
fn build_person(name: String, age:u8, hobby:String) ->Person {
    Person { name, age, hobby }
}

//6 
fn set_email(u: User) -> User {
    User { email: String::from("contact@im.dev"),
    ..u
    }
}

// 7
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//8
#[derive(Debug)]
struct Person8 {
    name: String,
    age: Box<u8>,
}

#[derive(Debug)]
struct File8 {
    name: String,
    data: String,
}