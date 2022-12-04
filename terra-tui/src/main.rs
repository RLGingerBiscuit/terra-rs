use terra_core::add;

fn main() {
    println!("Hello, world!");
    println!("{}", format!("{} + {} = {}", 5, 3, add(5, 3)));
}
