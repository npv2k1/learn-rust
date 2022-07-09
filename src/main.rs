fn variable(){
  let x:i32 = 10;
  let y:i32 = 20;

  let z = x+y;

//   let mut arr  = [1,2,4];
//   let z1 = &arr;
//   arr[0]=999;
  println!("x+y = {}",z);
//   println!("&z = {}",arr[0]);
//   println!("&z = {}",z1[0]);
}
fn main() {
    let name = "Nguyen";
    println!("Hello :, {}",name);
    variable()    
}

