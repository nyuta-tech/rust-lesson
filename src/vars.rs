pub mod sub_a;
mod sub_b;

pub fn run() {
    println!("here is vars module");
    sub_a::func_a();
    sub_b::func_b();
}