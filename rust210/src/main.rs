// 类型转换
// Rust 是类型安全的语言，因此在 Rust 中做类型转换不是一件简单的事
#![allow(unused,warnings)]
use std::convert::TryInto;
use std::collections::HashMap;

fn main() {
    // as 转换
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8; // 将字符'a'转换为整数，97
    println!("{},{},{}",a,b,c);
    
    // 内存地址转为指针
    let mut values = [1, 2];
    let p1 = values.as_mut_ptr();
    println!("{:?}",p1);
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    println!("{:?}",first_address);
    println!("{:?}",second_address);
    println!("{:?}",p2);
    unsafe{
        *p2 += 1;
    }
    assert_eq!(values[1], 3);

    // TryInto 转换
    let a = 10;
    let b: i16  = 1500;
    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };

    // 通用类型转换 将一个结构体转换为另外一个结构体
    struct Foo {
        x: u32,
        y: u16,
    }
    struct Bar {
        a: u32,
        b: u16,
    }
    fn reinterpret(foo: Foo) -> Bar {
        let Foo { x, y } = foo;
        Bar { a: x, b: y}
    }
    // 强制类型转换
    // 一个类型 T 可以强制转换为 U，不代表 impl T 可以强制转换为 impl U
    let t = &mut 0;
    // foo(t);
    // &i32 实现了特征 Trait， &mut i32 可以转换为 &i32，
    // 但是 &mut i32 依然无法作为 Trait 来使用。

}
trait Trait {}
fn foo<X: Trait>(t: X) {}
impl<'a> Trait for &'a i32 {}