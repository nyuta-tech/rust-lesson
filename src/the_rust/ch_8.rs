pub fn ch_8_vec() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let mut v = vec![1, 2, 3, 4, 5];

    {
        let third = &v[2];
        println!("The Third element is {}", third);
    }

    match v.get(2) {
        //=> indexが存在しない場合でもパニックしない
        Some(third) => println!("The Third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.0),
    ];
}

pub fn ch_8_string() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('l');

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s = format!("{}-{}-{}", s2, s2, s3);

    let s1 = String::from("Hola");
    let len = s1.len();

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}

use std::collections::HashMap;
pub fn ch_8_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
