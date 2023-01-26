fn main() {
    let mut i = 0;

    for j in 0..10 {
        println!("i = {}", j)
    }

    // while loop
    while i < 3 {
        println!("i = {}", i);
        i += 1;
    }
    // infinitive loop

    loop {
        println!("i = {}", i);
        break;
    }
}
