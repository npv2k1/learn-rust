# Nhập xuất cơ bản

Để nhập một mảng số nguyên từ bàn phím, bạn có thể sử dụng hàm read_line() để nhập một dòng văn bản, sau đó sử dụng hàm split_whitespace() để tách chuỗi văn bản thành các phần tử cách nhau bởi khoảng trắng. Sau đó, bạn có thể dùng hàm parse() của str để chuyển các phần tử này thành các số nguyên.

```rust
use std::io;

fn main() {
    println!("Nhập một mảng số nguyên cách nhau bởi khoảng trắng: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Không thể đọc dòng văn bản");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Không thể chuyển đổi thành số"))
        .collect();
    println!("Mảng số nguyên: {:?}", numbers);
}
```
