use std::env;

fn input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn main() {
    let html = input();
    println!("{:?}", html);
}
