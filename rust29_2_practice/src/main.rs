#![allow(unused,warnings)]
use std::collections::HashMap;
fn main() {
    // Practice 1
    let mut scores = HashMap::new();
    scores.insert("sunface", 98);
    scores.insert("daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    let score = scores.get("sunface");
    assert_eq!(score, Some(&98));
    if scores.contains_key("daniel"){
        let score = scores["daniel"];
        assert_eq!(score, 95);
        scores.remove("daniel");
    }
    assert_eq!(scores.len(), 3);
    for  ( k,v ) in scores{
        println!("{}, {}", k, v);
    }

    // Practice 2
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];
    let mut teams_map1 = HashMap::new();
    for team in teams.iter() {
        teams_map1.insert(team.0, team.1);
    }
    let teams_map2: HashMap<_, _> = teams.into_iter().collect();
    assert_eq!(teams_map2, teams_map1);
    let teams_map2 = HashMap::from(teams);
    assert_eq!(teams_map2, teams_map1);

    // Practice 3
    let mut player_stats = HashMap::new();
    player_stats.entry("health").or_insert(100);
    assert_eq!(player_stats["health"],100);

    // 通过函数来返回新的值
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"],100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(*health, 100);
    *health -= 50;
    assert_eq!(*health, 50);

    // Practice 4
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }

    // 容量
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    // 容量收缩
    map.shrink_to(50);
    assert!(map.capacity() >= 50);
    // 让 Rust  自行调整到一个合适的值
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);

    // P5 所有权
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("{:?}",m1);
    
    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    m2.insert(&v2, &v2);
    println!("{:?}",m2);
    


}

fn random_stat_buff() -> u8 {42}

#[derive(Debug,Hash,PartialEq,Eq)]
struct Viking {
    name: String,
    country: String,
}
impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Self { name: name.to_string(), country: country.to_string() }
    }
}