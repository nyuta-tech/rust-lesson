pub mod sub_a;
mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("here is vars module");
    //sub_a::func_a();
    //sub_b::func_b();
    let mut x = 5;
    println!(" x is {}", x);
    x = 6;
    println!(" x is {}", x);

    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("max points address {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("i2 points address {:p}", &i2);
    println!("i3 points address {:p}", &i3);

    let y = 5;
    println!("y points address {:p}", &y);
    let y = y + 1;
    println!("y points address {:p}", &y);
    let y = y * 2;
    println!("y points address {:p}", &y);
    println!("y is {}", y);
    {
        let y = 0;
        println!("y is {}", y);
    }
    println!("y is {}", y);

    // tuple
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("t1 is {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("t2 is {:?}", t2);

    //list
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);
}
