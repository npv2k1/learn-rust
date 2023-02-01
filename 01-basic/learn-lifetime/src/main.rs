fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("short");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);

    // destroy string1 and string2
 let s1;
    {
       s1 = string1.as_str();
        println!("string1 is {}", s1);
    }
    println!("string1 is {}", s1);
}
