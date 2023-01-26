fn main() {
    let mut num = 10;
    println!("num = {}, {:p}", num, &num);
    change_v1(num);
    change_v2(&mut num);
}
fn change_v1(num: i32) {
    println!("num = {}, {:p}", num, &num);
}
fn change_v2(num: &mut i32) {
    *num = 100;
    println!("num = {:p}", num);
}
