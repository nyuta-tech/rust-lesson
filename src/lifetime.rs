pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");

    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }
}

//'aをつけることでライフタイムが短い方に合わせる事が可能
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//関数内で値を作成し、参照を返すとダングリングポインタが発生するのでエラーになる
//fn dummy1<'a>() -> &'a str {
//let s = String::from("demo");
//&s
//}

//fn dummy<'a>() -> &'a i32 {
//let x = 10;
//&x
//}

//実体を返すのでOK
fn dummy3() -> String {
    let s = String::from("demo");
    s
}
