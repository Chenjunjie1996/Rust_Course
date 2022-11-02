#![allow(unused,warnings)]
// 动态数组 Vector
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

trait IpAddr_trait {
    /// .
    fn display(&self);
}
struct V4(String);
impl IpAddr_trait for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr_trait for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    // Vec::new创建动态数组是最 rusty 的方式，它调用了new关联函数
    let mut v = Vec::new();
    v.push(1);
    // 预先知道要存储的元素个数，可以使用 Vec::with_capacity(capacity)
    let mut v: Vec<i32> = Vec::with_capacity(5);
    v.push(1);
    // 宏 vec! 来创建数组 能在创建同时给予初始化值
    let v = vec![1,2,3];
    
    // 增 push
    // 查 读取指定位置的元素: 索引或get
    let mut v = vec![1,2,3,4,5];
    let third = &v[2];
    let index = match v.get(2){
        Some(third) => third,
        None => &2,
    };
    
    // 遍历
    let v = vec![1,2,3];
    for i in &v{
        let a = &v;
    }
    println!("{:?}",v);
    let mut v = vec![1,2,3];
    for i in v.iter_mut(){
        *i += 10;
    }
    println!("{:?}",v);
    
    // 存储不同类型的元素 (枚举和特征对象)
    let v = vec![
        IpAddr::V4(("V4").to_string()),
        IpAddr::V6(("V6").to_string()),    
    ];
    println!("{:?}",v);

    // 手动地指定类型：Vec<Box<dyn IpAddr>>，表示数组 v 存储的是特征 IpAddr 的对象
    let v:Vec<Box<dyn IpAddr_trait>> = vec![
        Box::new(V4("V4".to_string())),
        Box::new(V6("V6".to_string())),
    ];
    for i in v.iter(){
        i.display();
    }
}
