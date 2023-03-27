In Rust, a "slice" is a reference to a contiguous section of an array. It's represented by the &[T] type, where T is the type of the element in the array.

Here's an example of creating a slice from an array:

```rust
let a = [1, 2, 3, 4, 5];
let b = &a[1..4]; // b is a slice of the elements at index 1, 2, and 3
```
You can also create a slice from a vector using the &vec[start..end] syntax:

```rust
let c = vec![1, 2, 3, 4, 5];
let d = &c[1..4]; // d is a slice of the elements at index 1, 2, and 3
```
You can iterate over a slice using a for loop:

```rust
let e = &[1, 2, 3, 4, 5][1..4];
for i in e.iter() {
    println!("{}", i);
}
```
You can also use the .len() method to get the length of a slice:

```rust
let f = &[1, 2, 3, 4, 5][1..4];
println!("The length of the slice is: {}", f.len());
```
Slices are useful when you need to pass a portion of an array or vector to a function without creating a new copy of the data. The original array or vector retains ownership of the data and the slice is a "view" into that data.