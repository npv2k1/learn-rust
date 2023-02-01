use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);
    map.insert("Yellow".to_string(), 50);
    map.insert("10".to_string(), 10);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
