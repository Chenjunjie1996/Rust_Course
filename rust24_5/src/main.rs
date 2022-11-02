// 数组
// 在 Rust 中，最常用的数组有两种，
//第一种是速度很快但是长度固定的 array，数组
//第二种是可动态增长的但是有性能损耗的 Vector，动态数组

// unwrap 和 expect()
// http://www.codebaoku.com/rust-example/rust-example-047.html
use std::io;

fn main() {
    let a = [1,2,3,4,5];
    let b = [3; 5];
    // 元素类型大小固定，且长度也是固定，因此数组 array 是存储在栈上
    // 动态数组 Vector 是存储在堆上，因此长度可以动态改变。
    println!("{:?}",a);
    println!("{:?}",b);
    
    // 下面是一个接收用户的控制台输入
    let a = [1,2,3,4,5];
    println!("Please enter an array index");
    let mut index = String::new();
    // 读取控制台的输出
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {} is : {}", index, element);

    // 数组切片
    let slice = &a[1..3];
    assert_eq!(slice, [2,3]);

    // example
    let one = [1,2,3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    // 二维数组
    let arrays = [one, two, blank1, blank2];
    for a in & arrays{
        println!("{:?}", a);
        for n in a.iter() {
            println!("\t{} + 10 = {}", n, n+10);
        }

        let mut sum = 0;
        for i in 0..a.len(){
            sum += a[i]
        }
        println!("\t({:?} = {})", a, sum);
    }

    // Practice 
    // 数组的类型是 [T; Length], 长度是数组类型的一部分
    // 1
    let arr= [1,2,3,4,5];
    assert_eq!(arr.len(), 5);

    // 2 数组中的每个 char 元素占用 4 字节的内存空间
    let arr0 = [1,2,3];
    let arr1 = ['a', 'b', 'c'];
    println!("{:?}", std::mem::size_of_val(&arr0));
    println!("{:?}", std::mem::size_of_val(&arr1));

    // 3
    let list = [1;100];
    assert_eq!(list[0],1);
    assert_eq!(list.len(),100);

    // 4  数组中的所有元素必须是同一类型
    // let _arr = [1, 2, '3'];

    // 6
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
    let name0 = names.get(0);
    println!("{:?}", name0);
    println!("{}", name0.unwrap());

    let name1 = &names[0];
    println!("{},{:?}",name1, names);

    let name1 = names.get(2);
    println!("{:?}", name1);
    // let name1 = &names[2];

}
