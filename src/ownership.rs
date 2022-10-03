pub fn run() {
    //所有権の移譲(move)
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    //静的領域に確保される値はディープコピーされる
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("stack address {:p} {:p}", &i1, &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("stack address : {:p} {:p}", &sl1, &sl2);

    //heap領域の値でもcloneを使うことでディープコピー可能
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("stack address of s3 is {:p}", &s3);
    println!("stack address of s4 is {:p}", &s4);
    println!("heap memory address of hello:{:?}", s3.as_ptr());
    println!("heap memory address of hello:{:?}", s4.as_ptr());

    //関数の引数に渡す際にも所有権は移譲される
    let s5 = String::from("hello");
    println!("stack address of s5:{:p}", &s5);
    println!("heap address of hello: {:?}", s5.as_ptr());
    take_ownership(s5);
    //println!("{}", s5);

    //返り値として受け取ることで新たなスタックアドレスを作成し、参照可能
    let s6 = String::from("hello");
    println!("stack address of s6:{:p}", &s6);
    println!("heap address of hello: {:?}", s6.as_ptr());
    let s7 = take_giveback_ownership(s6);
    println!("stack address of s7:{:p}", &s7);
    println!("heap address of hello: {:?}", s7.as_ptr());

    //参照渡しで引数をとることで元の値の所有権は移譲されない
    //immuutableの参照
    //複数作ることが可能
    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("the len of s8 is {}", len);

    //mut参照で元の変数を書き換えすることが可能(所有権は移譲されない)
    //複数は×
    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    //有効範囲の関係
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello updated");
    println!("{}", r3);

    //ダングリングポインタ
    //参照は実体より長生きしてはいけない
    //let r;
    //{
    //let x = 5;
    //r = &x;
    //}
    //println!("r:{}",r);
}

fn take_ownership(s: String) {
    println!("stack address of s5:{:p}", &s);
    println!("heap address of hello: {:?}", s.as_ptr());
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    println!("stack address of s5:{:p}", &s);
    println!("heap address of hello: {:?}", s.as_ptr());
    println!("{}", s);
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
