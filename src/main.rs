fn main() {
    let a: i32 = 1;
    let b: i32 = 2;

    let result = add(a, b);

    println!("Hello, world!");
    println!("{} + {} = {}", a, b, result);
}

fn add(a : i32, b : i32) -> i32 {
    a + b
}
