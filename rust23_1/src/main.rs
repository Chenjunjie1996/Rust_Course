// 所有权和借用
//垃圾回收机制(GC)，在程序运行时不断寻找不再使用的内存，典型代表：Java、Go
//手动管理内存的分配和释放, 在程序中，通过函数调用的方式来申请和释放内存，典型代表：C++
//通过所有权来管理内存，编译器在编译时会根据一系列规则进行检查

// Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    //转移所有权
    //对于基本类型（存储在栈上），Rust 会自动拷贝
    //但是String不是基本类型，而且是存储在堆上的
    let x = 5;
    let y = x;
    println!("{},{}", x, y);
    
    let x = "hello, world";
    let y = x;
    println!("{},{}", x,y);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}",s1);

    // 克隆（深拷贝） 使用 clone 会极大的降低程序性能
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{},{}",s1,s2);

    // 浅拷贝只发生在栈上，因此性能很高
    let x = 5;
    let y = x;
    println!("{},{}", x, y);
    //原因是像整型这样的基本类型在编译时是已知大小的，会被存储在栈上
    //这里没有深浅拷贝的区别，因此这里调用 clone 并不会与通常的浅拷贝有什么不同
    //可以理解成在栈上做了深拷贝

    // 函数传值与返回
    let s = String::from("hello"); // s进入作用域
    takes_ownership(s); // s 的值移动到函数里
    // println!("{}",s); // 所以到这里不再有效
    let x = 5; // x进入作用域
    makes_copy(x); // i32 是 Copy 的，所以在后面可继续使用 x
    println!("{}", x);

    // 函数返回值也有所有权
    let s1 = gives_ownership();
    // let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{},{}",s1,s3);

    // Practice 所有权
    // 1
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);

    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);

    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);

    let x = 10;
    let y = x;
    println!("{},{}",x,y);

    // 2
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);
    println!("{}",s2);

    // 3
    let s = give_ownership();
    println!("{}", s);

    // 4
    let s = String::from("hello, world");
    print_str(s);
    // println!("{}", s);

    // 5 不要使用 clone，使用 copy 的方式替代
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);

    // 当所有权转移时，可变性也可以随之改变。
    // 6
    let s = String::from("hello, ");
    let mut s1 = s;
    s1.push_str("world");

    // 7
    let x  =Box::new(5);
    assert_eq!(*x, 5);

    // 部分move 
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let person = Person{
        name: String::from("Alice"),
        age: Box::new(20),
    };
    println!("{:?}", person);
    println!("{}, {}", person.name, person.age);
    let Person {name, ref age} = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    // Error! person 的一部分已经被转移了所有权，因此我们无法再使用它
    // println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使
    println!("The person's age from person struct is {}", person.age);
    
    // 8
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    println!("{}", t.1);

    // 9
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = t.clone();
    println!("{},{},{:?}", s1, s2, t);

}

fn takes_ownership(some_string: String){ 
    // some_sring 进入作用域
    println!("{}", some_string);
}//some_string移出作用域并调用 drop方法,占用内存释放

fn makes_copy(some_integer: i32) -> () {
    println!("{}", some_integer);
}//some_integer 移出作用域。不会有特殊操作

// 将返回值移动给调用它的函数
fn gives_ownership() -> String{
    //some_string 进入作用域.
    let some_string = String::from("hello");
    some_string //返回 some_string 并移出给调用的函数
}

// 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String{
    a_string  // 返回 a_string 并移出给调用的函数
}

// Practice
fn take_ownership(s: String) -> String{
    s
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    s
}

fn print_str(s: String) {
    println!("{}", s)
}