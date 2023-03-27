Here is an example of declaring and initializing an array in Rust:

```rust
let a = [1, 2, 3, 4, 5]; // array of 5 integers
```
You can also specify the type of the array elements, like this:


```rust
let b: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 integers of type i32
```

You can also create an array by using the vec! macro:

```rust
let c = vec![1, 2, 3, 4, 5]; // array of 5 integers
```

You can also access elements of an array using the indexing syntax:

```rust
let d = [1, 2, 3, 4, 5];
println!("The third element is {}", d[2]);
```

You can iterate over an array using a for loop:

```rust
let e = [1, 2, 3, 4, 5];
for i in e.iter() {
    println!("{}", i);
}
```

You can also use the .len() method to get the length of an array

```rust
let f = [1, 2, 3, 4, 5];
println!("The length of the array is: {}", f.len());
```

Note that the indexing starts with 0 and the last index is len-1.
