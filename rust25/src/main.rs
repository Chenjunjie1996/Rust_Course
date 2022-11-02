#![allow(warnings, unused)]
// 流程控制 if else
fn main() {
    let condition = true;
    // 若 let 将 true 解构成 condition，则执行
    if let condition = true{
        println!("{}", condition);
    }else{
        panic!("error");
    }

    // 循环 for while loop
    for i in 1..=5 {
        println!("{}", i);
    }
    // 集合 所有权转移
    let array = [1,2,3];
    // 实现copy特征的数组 不会转移所有权
    for i in array {
        println!("{}",i);
    }
    println!("{:?}", array);

    // enumerate
    let a = [1,2,3,4];
    for (i,v) in a.iter().enumerate(){
        println!("{},{}",i,v);
    }

    // continue break

    // Practice
    // 1 ifelse if else
    // 2  if/else 可以用作表达式来进行赋值
    let n = 5;
    let big_n = {
        if n < 10 && n > -10 {
            n * 10
        }else {
            n / 2
        }
    };
    println!("{},{}",n, big_n);

    // 4
    // 所有权转移
    let names = [String::from("liming"), String::from("wangming")];
    for name in &names{
        println!("{}", name);
    }
    println!("{:?}",names);
    // 实现copy 无需转移
    let numbers = [1,2,3];
    for n in numbers{
        println!("{}", n);
    }
    println!("{:?}",numbers);

    // 5 enumerate
    // 6
    let mut n = 1;
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        }else if n % 3 == 0 {
            println!("fizz");
        }else if n % 5 == 0 {
            println!("buzz");
        }else {
            println!("{}", n);
        }
        n += 1;
    }
    println!("while over");

    //7 break
    let mut n = 0;
    for _ in 0..=100 {
        if n == 66{
            break;
        }
        n += 1;
    }
    assert_eq!(n, 66);

    //8 continue
    let mut count = 0;
    for _ in 0..=100 {
        if n != 66{
            n += 1;
            continue;
        }
        break;
    }
    assert_eq!(n, 66);

    // 9 loop 无限循环
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}",count);

        if count == 5 {
            println!("enough");
            break;
        }
    }
    assert_eq!(count, 5);

    // 10 loop 是表达式 配合break返回值
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break 2 * count;
        }
    };
    assert_eq!(result, 20);

    // 11 标签 'label
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 跳出 inner1
                break 'inner1;
            }
            count += 2;
        }
        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }
    
    assert_eq!(count, 30);

}
