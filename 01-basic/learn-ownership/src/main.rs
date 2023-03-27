fn main() {
    let mut num = 10;
    println!("num = {}, {:p}", num, &num);
    change_v1(num);
    change_v2(&mut num);

    let s = String::from("hello");
    let r = &s as *const String;
    let s2 = unsafe { &*r };
    let s3 = unsafe { &*r };

    println!("s = {}, {:p}", s, &s);
    println!("r =, {:p}", r);
    println!("S2 ={}, {:p}", s2, &s2);
    println!("S3 ={}, {:p}", s3, &s3);
}
fn change_v1(num: i32) {
    println!("num = {}, {:p}", num, &num);
}
fn change_v2(num: &mut i32) {
    *num = 100;
    println!("num = {:p}", num);
}
