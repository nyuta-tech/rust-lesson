pub fn ownership(){

    let mut s = String::from("");

    change(&mut s);

    change(&mut s);

    println!("{}",s);

    let s = String::from("hello world");
    //{
        //let x = 1.9;
        //let y = 111;
        //let add_msg = format!("test {}, 2: {}", x, y);

        //s.push_str(&add_msg);
    //}
    let word = first_word(&s);

    println!("{}", word);




    
}

fn change(some_string:&mut String){
    some_string.push_str(", world");
}

//fn first_word(s:&String) -> usize{
    //let bytes = s.as_bytes();

    //for (i, &item) in bytes.iter().enumerate(){
        //if item == b' '{
            //return i;
        //}
    //}
    //return s.len();
//}

fn first_word(s:&str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item ) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s[..];
}