pub fn struct_learn(){
    let mut user = User{
        username: String::from("Nyuta"),
        email:String::from("nyuta.main@gmail.com"),
        sign_in_account:1,
        active:true,
    };

    //一部のフィールドのみ可変はできず、全体が可変になる
    user.username = String::from("uta");
    
    let user2=  User {
        username:String::from("nybin"),
        ..user
    };

    //あとで調べる
    //drop(user); => error原因: user2でuserを不変参照してるのにdropでmoveしてるからuser2が参照できなくなりアウト

    let color = Color(0, 0, 0);
    let point = Point(0, 0, 0);

}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn drop(user:User){
    println!("drop");
}

fn build_user(email:String, username:String) -> User {
    User {
        username, //省略系でかける
        email,
        active:true,
        sign_in_account:1
    }
}

struct User {
    username:String,
    email:String,
    sign_in_account:u64,
    active:bool,
}

pub fn sample_main(){
    let rect1 = Rectangle{width: 30, height:50};

    println!("The value of square of rectangle area is {}", area(&rect1));

    println!("{}", rect1.width);
    println!("{:?}", rect1);

    println!("The value of square of rectangle area is {}", rect1.area());
}

fn area(rectangle:&Rectangle) -> u32{
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}
