fn hello_world() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print(s: &str) {
    println!("{}", s);
}

fn increment(x: &mut i32) {
    *x += 1;
}


fn main() {
    hello_world();
    println!("1 + 2 = {}", add(1, 2));
    print("Hello, world!");

    let mut x = 1;
    increment(&mut x);
    println!("x = {}", x);


}
