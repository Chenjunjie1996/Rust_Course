#![allow(unused,warnings)]
use std::collections::HashMap;
// KV 存储 HashMap
// 所有的 K 必须拥有同样的类型，V 也是如此。
fn main() {
    // 增
    let mut my_gems = HashMap::new();
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);
    println!("{:?}",my_gems);

    // 使用迭代器和 collect 方法创建
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);

    // 所有权
    // 若类型实现 Copy 特征，该类型会被复制进 HashMap
    // 若没实现 Copy 特征，所有权将被转移给 HashMap中
    let name = String::from("Sunface");
    let age = 18;
    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);
    println!("{:?}", handsome_boys);

    // 查 get
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 改 
    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.insert("blue", 20); //覆盖

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5);
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入

    // 文本中统计词语出现的次数
    let test = "hello world wonderful world";
    let mut map: HashMap<&str, u32> = HashMap::new();
    for word in test.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    
}
