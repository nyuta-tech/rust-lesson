pub fn control_flow() {
    //if
    let num = 3;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is {}", num);

    //loop
    //ラベルで上位階層のループを抜けられる
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Lift off!");
}
