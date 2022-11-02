#![allow(warnings, unused)]

// 引用与解引用
// 借用规则
// 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
// 引用必须总是有效的
fn main() {
    // & 符号即是引用，它们允许你使用值，但是不获取所有权
    let x = 5;
    let y = &x; // 引用
    assert_eq!(5 ,x);
    assert_eq!(5, *y); // 解引用
    // 不允许比较整数与引用，因为它们是不同的类型。必须使用解引用运算符解出引用所指向的值。

    // 不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // error 所有权改变
    // let s1: String = String::from("hello");
    // let len = calculate_length_comp(s1);
    // println!("The length of '{}' is {}", s1, len);

    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s);

    // 同一作用域，特定数据只能有一个可变引用
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    // 引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方
    // 变量的作用域从创建持续到某一个花括号 }
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3); // r3作用域在这里结束

    // 悬垂引用
    // let reference_to_nothing = dangle();
    let reference = no_dangle();
    println!("{}",reference);

    // Practice
    // 1
    let x = 5;
    let p = &x;
    println!("x的内存地址是{:p}", p);

    // 2
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);

    // 3
    let mut s = String::from("hello,");
    borrow_object(&mut s);
    println!("{}",s);

    // 4
    let mut s = String::from("hello, ");
    push_str(&mut s);

    // 5
    let mut s: String = String::from("hello, ");
    let p = &mut s;
    p.push_str("world");

    // 6 ref 与 & 类似，可以用来获取一个值的引用，但是它们的用法有所不同。
    let c = '中';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    //判断两个内存地址的字符串是否相等
    assert_eq!(get_address(r1), get_address(r2));

    // 7 借用规则
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // 8 9 可变性
    let mut s = String::from("hello, ");
    borrow_object(&mut s);

    // 10
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    println!("{}", s);

}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn calculate_length_comp(s: String) -> usize{
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

// fn dangle() -> &String{
//     let s = String::from("hello");
//     &s
// } // 这里 s 离开作用域并被丢弃。其内存被释放。

fn no_dangle() -> String{
    let s:String = String::from("hello");
    s
}

fn borrow_object(a_string:&mut String) {
    a_string.push_str("world");
}

fn push_str(s: &mut String){
    s.push_str("world");
}

fn get_address(r: &char) -> String{
    println!("{:p}", r);
    format!("{:p}", r)
}