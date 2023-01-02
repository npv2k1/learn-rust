#![allow(dead_code)]

use std::{io, num, str::FromStr};

fn read_string() -> String {
    println!("Nhập một dòng văn bản: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Không thể đọc dòng văn bản");
    return input;
}

fn read_array<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    println!("Nhập một mảng số cách nhau bởi khoảng trắng: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Không thể đọc dòng văn bản");
    input
        .split_whitespace()
        .map(|s| s.parse().expect("Không thể chuyển đổi thành số"))
        .collect()
}

fn main() {
    let numbers = read_array::<i32>();
    println!("Mảng số nguyên: {:?}", numbers);
}
