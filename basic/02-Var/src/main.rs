use std::io;

#[allow(unused_doc_comments)]
#[allow(unused_variables)]
fn main() -> io::Result<()> {
    // Mặc định khi khai báo là immutable
    let _name = "Nguyen";
    // Biến có thể thay đổi
    let _age = 21;
    // Hằng số
    let _time = 10;
    // const vs immutable
    /// * immutable có thể khai báo rồi gấn dữ liệu sau đó.
    ///  * const khi khai báo phải gán luôn
    ///  * cả 2 đều ko thể thay đổi giá trị.
    // Kiểu dữ iệu
    // Scalar: integer, float, char, bool
    // Compound: Array, Tuple, Slice
    let x = 10; //
    let xi16: i16 = 10;

    // array
    // kích thước cố định, các kiểu dữ liệu giống nhau, mặc định là immutable
    let arr: [i32; 3] = [1, 2, 3];
    println!("{:?}", arr);
    let [a, b, c] = arr; // destructure array
    println!("a = {}", a);
    // Tuple
    // kích thước cố định, các kiểu dữ liệu khác nhau
    let tup: (i32, u64, &str) = (-10, 5, "a");
    println!("{:?}", tup);

    // Slice
    let a: [i32; 4] = [1, 2, 3, 4]; // Parent Array
    let b: &[i32] = &a; // Slicing whole array
    let c = &a[0..4]; // From 0th position to 4th(excluding)
    let d = &a[..]; // Slicing whole array
    let e = &a[1..3]; // [2, 3]
    let f = &a[1..]; // [2, 3, 4]
    let g = &a[..3]; // [1, 2, 3]

    println!("{:?} {:?}", a, d);
    // String

    let mut fullName = String::from("Nguyen");
    fullName.push_str(":)");
    println!("{:?}", fullName);
    // Điều kiện
    // if else
    // match
    // Vòng lặp
    // loop
    // for
    // while

    println!("Hello,");

    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let leng = org_arr.len();
    let mut i = 0;
    let mut check = false;
    while 1 == 1 {
        if (i > org_arr.len() - sub_arr.len() + 1) {
            break;
        }
        let tmp = &org_arr[i..i + sub_arr.len()];
        i += 1;
        println!("{:?}", tmp);
        if tmp == sub_arr {
            println!("{:?}", tmp);
            println!("{:?}", sub_arr);
            println!("{}", tmp == sub_arr);
            check = true;
            break;
        }
    }
    if check {
        println!("YES");
    } else {
        println!("NO");
    }

    let input = "adbcdaDd";
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input);

    println!("input {} ", user_input);
 
    Ok(())

}
