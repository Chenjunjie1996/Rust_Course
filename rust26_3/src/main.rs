#![allow(warnings, unused)]

// 模式适用场景
// 模式一般有：字面值 ，解构的数组、枚举、结构体或者元组 变量，通配符，占位符
fn main() {
    // 模式匹配有可驳模式和不可驳模式两种。if let 和 while let 就属于可驳模式匹配。
    // if let 往往用于匹配一个模式
    if let condition = true{
        println!("{}", condition);
    }else{
        panic!("error");
    }

    let info = Info {
        name: Some("info_name".to_string()),
        addr: Some("info_addr".to_string()),
        phone: Some(19912345678),
    };
    check_info(info);

    // while let 条件循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // 只要模式匹配就一直进行 while 循环
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a','b','c'];
    for (index, value) in v.iter().enumerate(){
        println!("{} is at index {}",value, index);
    }
}

struct Info {
    name: Option<String>,
    addr: Option<String>,
    phone: Option<usize>,
}

fn check_info(info: Info) -> bool {
    if let (Some(name), Some(addr), Some(phone)) = (info.name, info.addr, info.phone) {
        println!("name{:?},addr:{:?},phone:{:?}", name, addr, phone);
        return true;
    }
    false
}
