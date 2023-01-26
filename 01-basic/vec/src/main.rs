fn sample_vec(){
    let mut arr = Vec::new();
    for i in 1..10 {
        arr.push(i);
    }
    println!("{:?}", arr);
}

fn main() {
    sample_vec();   
}
