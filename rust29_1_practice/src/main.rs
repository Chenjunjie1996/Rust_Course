#![allow(unused,warnings)]
// 相比 [T; N] 形式的数组， Vector 最大的特点就是可以动态调整长度。
fn main() {
    // p1
    let arr = [1,2,3];
    let v = Vec::from(arr);
    is_vec(&v);
    let v = vec![1,2,3];
    is_vec(&v);
    let mut v1 = Vec::new();
    for i in v.iter(){
        v1.push(*i);
    }
    is_vec(&v1);
    assert_eq!(v,v1);

    // p2 extend
    let mut v1 = Vec::from([1,2,3]);
    v1.pop();
    v1.push(3);
    let mut v2 = Vec::new();
    v2.extend([1,2,3]);
    assert_eq!(v1,v2);

    // p3 类型转换
    let arr = [1,2,3];
    let v1 = Vec::from(arr);
    let v2 = arr.to_vec();
    assert_eq!(v1,v2);

    let s = "hello".to_string();
    let v1: Vec<u8> = s.clone().into();
    let v2 = s.clone().into_bytes();
    assert_eq!(v1,v2);
    let s = "hello";
    let v3 = Vec::from(s);
    println!("{:?}",v3);

    // 迭代器 Iterators 可以通过 collect 变成 Vec
    let v4: Vec<u8> = [0; 10].into_iter().collect();
    println!("{:?}",v4);
    let mut iter_v4 = [0;10].into_iter();
    let result = match iter_v4.next() {
        Some(_) => "number",
        None => "other",
    };
    println!("{}",result);

    // p4 索引
    let mut v = Vec::from([1,2,3]);
    for i in 1..5 {
        println!("{:?}", v.get(i))
    }
    for i in 0..5{
        if let Some(x) = v.get(i){
            v[i] = x+1
        } else {
            v.push(i + 2)
        }
    }
    println!("{:?}",v);

    // p5 切片 
    // 切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
    let mut v = vec![1,2,3];
    let slice1 = &v[..];
    let slice2 = &v[0..v.len()];
    assert_eq!(slice1, slice2);

    let vec_ref = &mut v;
    (*vec_ref).push(4);
    println!("{:?}",vec_ref);
    let slice3 = &mut v[0..];
    assert_eq!(slice3, [1, 2, 3, 4]);

    // p6 提前分配好足够的容量，尽量减少内存分配。
    let mut vec: Vec<u32> = Vec::with_capacity(10);
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);

    // P7 在 Vec 中存储不同类型的元素 枚举 特征对象
    let v = vec![
        IpAddr::V4(("127.0.0.1").to_string()),
        IpAddr::V6(("::1").to_string()),
    ];
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    // P8 特征对象
    let v:Vec<Box<dyn Mytrait>> = vec![
        Box::new(Duck),
        Box::new(Swan),
    ];
    for i in v.iter(){
        i.method();
    }


    let v:Vec<Box<dyn Mytrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for i in v.iter(){
        i.method()
    }

}
fn is_vec(v: &Vec<u8>) {}

#[derive(Debug,PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}

trait Mytrait {
    fn method(&self);
}
struct Duck;
struct Swan;
impl Mytrait for Duck{
    fn method(&self) {
        println!("duck");
    }
}
impl Mytrait for Swan {
    fn method(&self) {
        println!("swan");
    }
}

struct V4(String);
struct V6(String);
impl Mytrait for V4 {
    fn method(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
impl Mytrait for V6 {
    fn method(&self) {
        println!("ipv6: {:?}",self.0)
    }
}
