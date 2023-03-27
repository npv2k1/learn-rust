# Varibale

Biến là nơi để chứa dữ liệu.

Trong rust khai báo biến bằng keyword `let`. Mặc định khi khai báo sẽ là `immutable` (không thể thay đổi giá trị).

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
