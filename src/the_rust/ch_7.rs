mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    //enumは公開にすると中身も全て公開になる
    pub enum Appertizer {
        Soup,
        Salad,
    }
}

// pub fn eat_at_restaurant(){
//     //Absolute Path
//     crate::the_rust::ch_7::front_of_house::hosting::add_to_waitlist();

//     //Relative Path
//     front_of_house::hosting::add_to_waitlist();
// }

use crate::the_rust::ch_7::front_of_house::hosting;
// use self::front_of_house::hosting;

//関数のインポートは親モジュールまでuseして使用時にモジュールも明示するのが慣例
//ex hosting::add_to_waitlist();

//構造体やenumはフルパスでuseするのが慣例
//ex Appertizer::Soup;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//asを使うことで名前を変更可能
use std::io::Result as IoResult;
