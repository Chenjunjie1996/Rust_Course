// 字符串和切片
#![allow(warnings, unused)]

//对于 Rust 而言，安全和性能是写到骨子里的核心特性，
//如果使用 GC，那么会牺牲性能；如果使用手动管理内存，
//那么会牺牲安全，这该怎么办？为此，Rust 的开发者想出了一
//个无比惊艳的办法：变量在离开作用域后，就自动释放其占用的内存：
fn main() {
    // let my_name = "Pascal";
    // greet(my_name);

    // 切片
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s[..];
    println!("{},{}",hello,world);
    println!("{}",slice);

    // 每个汉字占用三个字节
    let s = "中国人";
    let a = &s[0..3];
    println!("{}",a);

    // 字符串切片的类型标识是 &str
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    //println!("the first word is: {}", word);

    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // 字符串字面量是切片，&str 是一个不可变引用。
    let s = "Hello, world!";

    // 当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型
    // String与&str转换
    // &str -> String
    let s = String::from("hello, world");
    let s = "hello, world".to_string();
    // String -> &str 取引用即可
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    // 字符串索引 在 Rust 中就会报错

    // 操作字符串
    // 增
    let mut s = String::from("Hello ");
    s.push('r'); // 追加字符
    s.push_str("ust"); // 追加字符串
    println!("{}", s);
    // 插入
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    s.insert_str(6, " I like");
    println!("插入后: {}", s);

    // 替换
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(2..8, "R");
    dbg!(string_replace_range);

    // 删
    // pop
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    // remove
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    let removed_string = string_remove.remove(0);
    println!("{}",removed_string);
    println!("{}",string_remove);
    //truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(7);
    dbg!(string_truncate);

    // clear —— 清空字符串 相当于 truncate() 方法参数为 0
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 连接
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust; // &string_rust会自动解引用为&str
    let mut result = result + "!";
    result += "!!!";
    println!("连接字符串 + -> {}", result);

    // + 相当于调用std::string 标准库中的 add()
    // fn add(self, s: &str) -> String
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // String = String + &str + &str + &str + &str
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);

    // Practice 字符串
    // 1
    let s = "hello, world";
    //2
    let s: Box<str> = "hello, world".into();
    println!("{}",s);
    greetings(&s);
    //3
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");
    //4 
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";
    println!("{}",s);
    //5
    let mut s = String::from("I like dogs");
    s = s.replace("dogs", "cats");
    assert_eq!(s, "I like cats");
    //6
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    println!("{}",s1);
    //7
    let s = "hello, world".to_string();
    let s = String::from("hello");
    //8
    let s = "hello, world".to_string();
    let s1 = &s;

    // 11
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; 
    assert_eq!(h, "h");

    let h1 = &s1[3..6];
    assert_eq!(h1, "中");

    //12
    for c in "你好，世界".chars() {
        println!("{}", c)
    }

    // Practice 切片
    //1
    let arr = [1,2,3];
    let s1 = &arr[0..2];
    println!("{:?}",s1);

    //2
    let arr = ['中','国','人'];
    let slice = &&arr[..2];
    println!("{:?}", slice);
    let s = "中国人";
    let slice = &s[..6];
    println!("{}",slice);

    //3
    let arr = [1,2,3,4,5];
    let slice = &arr[1..4];
    assert_eq!(slice, &[2,3,4]);

    //6
    let mut s = String::from("hello world");
    let word = first_word_str(&s);
    println!("the first word is: {}", word);
    s.clear();

    // Practice String
    // 1
    let mut s = String::from("hello, ");
    s.push_str("world");
    s.push('!');
    move_ownership(s.clone());
    assert_eq!(s, "hello, world!");

    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');
    borrow_string(&s);
    assert_eq!(s, "hello, world!");

    //2
    let mut s = String::from("hello, world");
    let slice1 = s.as_str();
    // let slice1 = &s;
    assert_eq!(slice1, "hello, world");
    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");
    let slice3 = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    //4 
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1];
    assert_eq!(slice1, "h");
    let slice2 = &s[7..10];
    assert_eq!(slice2, "世");
    for (i,c) in s.chars().enumerate(){
        if i==7{
            assert_eq!(c, '世')
        }
    }

    // 5
    let mut s = String::new();
    s.push_str("hello");
    let v = vec![104,101,108,108,111];
    let s1 = String::from_utf8(v).unwrap();
    assert_eq!(s, s1);

    // 6
    let mut s = String::with_capacity(25);
    println!("{}", s.capacity());
    for _ in 0..2{
        s.push_str("hello");
        println!("{}", s.capacity());
    }
    println!("{}", s);

    //7
    let story = String::from("Rust By Practice");
    // Prevent automatically dropping the String's data
    let mut story = std::mem::ManuallyDrop::new(story);
    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();
    assert_eq!(16,len);
    let s = unsafe {
        String::from_raw_parts(ptr, len, capacity)
    };
    assert_eq!(*story, s);

}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn first_word(s: &String) -> &str{
    &s[..1]
}

fn say_hello(s: &str) {
    println!("{}", s);
}

fn greetings(s: &str){
    println!("{}", s);
}

fn first_word_str(s: &str) -> &str{
    &s[..1]
}

fn move_ownership(s: String){
    println!("ownership of \"{}\" is moved here", s);
}

fn borrow_string(s: &str) {
    println!("ownership of \"{}\" is still with the variable 's', only the reference is passed", s)
}