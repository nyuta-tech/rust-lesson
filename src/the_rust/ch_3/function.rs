pub fn function() {
    another_function(5);

    //文 => 値を返さない
    //let y = (let x =6); => コンパイルエラー

    //式 => 値を返す
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
}

pub fn another_function(x: i32) {
    //スネークケース
    println!("The value of x is {}", x);
}
