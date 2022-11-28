use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

pub fn ch_9_panic() {
    let f = File::open("hello.txt");

    //err型をmatchを用いて分岐させる
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            println!("There was a problem opening the file; {:?}", error);
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    //↑だと冗長なのでunwrap(), expect()というものがある
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Filed to open hello.txt");
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
