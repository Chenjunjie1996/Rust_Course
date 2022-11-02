#![allow(unused,warnings)]

use std::fmt;
use std::hash::Hash;
use std::ops::Sub;

// 提升代码的可读性
pub trait CacheableItem: Clone + Default + fmt::Debug {
    type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
    fn is_null(&self) -> bool;
}

// Practice 1 关联函数
struct Container(i32, i32);
trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _:&Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}
impl Contains for  Container{
    type A = i32;
    type B = i32;
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }

}

// 2. 定义默认的泛型类型参数
#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T: std::ops::Sub<Output=T>> Sub for Point<T> {
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// 3 完全限定语法
// 在 Rust 中，两个不同特征的方法完全可以同名，且你可以为同一个类型
// 同时实现这两个特征。这种情况下，就出现了一个问题：该如何调用这两个
// 特征上定义的同名方法。为了解决这个问题，我们需要使用完全限定语法( Fully Qualified Syntax )。
trait UsernameWidget{
    fn get(&self) -> String;
}
trait AgeWidget {
    fn get(&self) -> u8;
}
struct Form {
    username: String,
    age: u8,
}
impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}
impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}


// 4. Supertraits
// 有些时候我们希望在特征上实现类似继承的特性，例如让一个特征A使用
// 另一个特征B的功能。这种情况下，一个类型要实现特征 A 首先要实现特征 B，特征B就被称为supertrait
trait Person {
    fn name(&self) -> String;
}
// Person 是 Student 的 supertrait.
// 实现 Student 需要同时实现 Person.
trait Student: Person {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
}
// CompSciStudent (computer science student) 是 Programmer 
// 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}
struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String
}
// 为 CSStudent 实现所需的特征
impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

// 5 孤儿规则
struct Pretty(String);
impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    // 1 关联类型
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}", &number_1, &number_2, container.contains(&number_1, &number_2));

    // 2 定义默认的泛型类型参数
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 });

    // 3. 完全限定语法
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    println!("{}", UsernameWidget::get(&form));
    println!("{}", AgeWidget::get(&form));



    // 4 Supertraits
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string()
    };

    println!("{}", comp_sci_student_greeting(&student));

    // 5 孤儿规则
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
