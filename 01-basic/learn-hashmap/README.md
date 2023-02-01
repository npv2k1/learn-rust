# Hashmap

Bạn có thể sử dụng thư viện `std::collections::HashMap` trong Rust để tạo một hashmap. Ví dụ như sau:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);
    map.insert("Yellow".to_string(), 50);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
```
