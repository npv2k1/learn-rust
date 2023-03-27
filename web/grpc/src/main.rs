use rand::prelude::*;

fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1; // Khi gán thế này thì quyền sở hữu (ownership bị chuyển hoàn toàn sang s2 và s1 bị hủy)

    // let s3 = &s2;
    // println!("{:?}",s2);

    let x: u8 = random();
    println!("{}", x);

    // println!("{}, world!", s1);
}
