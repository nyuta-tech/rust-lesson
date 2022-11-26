use std::io;

pub fn convert_temp() {
    loop {
        println!("Please input temperture ℃");
        println!("'finish' is end word");
        let mut guess = String::from("");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess == "finish" {
            break;
        }

        let temperture: f64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number");
                continue;
            }
        };

        println!("摂氏 {}℃", temperture);

        let temperture: f64 = (temperture * 9.0 / 5.0) + 32.0;
        println!("華氏 {}°F", temperture);
    }
}

pub fn fibonacci() {
    let mut x = 1;
    let mut y = 1;
    println!("{}", x);
    println!("{}", y);

    loop {
        println!("x + y= {}", x + y);
        let z = x + y;

        x = y;
        y = z;

        if y > 300 {
            break;
        }
    }
}
