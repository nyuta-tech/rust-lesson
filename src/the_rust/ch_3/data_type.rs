pub fn data_type() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //配列
    //型は全て同じで要素数も固定
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5]; //[3,3,3,3,3]

    let first = a[0];
    let second = a[1];
}
