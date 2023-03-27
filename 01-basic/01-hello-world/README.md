# Learn rust

## Bắt đầu một ứng dụng với rust

```bash
cargo init
```

## Hello world

```rust
fn main() {
    println!("Hello, world!");
}
```

notes:

- `fn` là keyword dùng để khởi tạo một hàm
- `main` là tên của hàm. Chương trình rust luôn chạy hàm main đầu tiên
- `println!` là `macro` dùng để in ra màn hình. Khái niệm `macro` sẽ được giới thiệu sau

### Để chạy ta dùng lệnh

Check các thư viện và các file trong thư mục

```bash
cargo check
```

Chạy chương trình

```bash
cargo run
```
