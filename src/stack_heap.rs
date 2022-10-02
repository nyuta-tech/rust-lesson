pub fn run() {
    //let a1: [u8; 9000000] = [1; 9000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("stack address of v1 {:p}", &v1);
    println!("stack address of v2 {:p}", &v2);
    println!("heap memory of v1 {:?}", v1.as_ptr());
    println!("length of v1 {:?}", v1.len());
    println!("capacity of v1 {:?}", v1.capacity());

    v1.insert(1, 10);
    println!("{:?}", v1);

    v1.remove(0);
    println!("{:?}", v1);

    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);
}
