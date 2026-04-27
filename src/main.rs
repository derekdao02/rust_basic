use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let name = args.get(1).map(String::as_str).unwrap_or("World");

    greet(name);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
