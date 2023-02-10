# Rust ownership
Rust has a unique ownership model that is designed to prevent data races and other forms of undefined behavior.

Here's an example of how ownership works in Rust:

```rust
let s = String::from("hello");
let t = s;

println!("{}, {}", s, t);
```


In this example, the variable s is created and assigned the value "hello". Then, the variable t is created and assigned the value of s.

After this, you will get an error if you try to use s again, because Rust considers s to be "moved" into t. This is because Rust assumes that the original variable is no longer needed, and it wants to ensure that the data it references is not accidentally used after it has been moved.

In summary, in Rust variables have the ownership of the data and when the ownership is transferred from one variable to another, the previous owner is no longer valid and cannot be used further.


In Rust, the & and * symbols are used in the context of references and pointers.

A reference is a way to refer to some value without taking ownership of it. It is created using the & symbol. For example:

```rust
let s = String::from("hello");
let r = &s;
```
In this example, r is a reference to the value of s. It points to the same memory location as s, but it does not own the data. The variable s is still valid and can be used as usual.

A pointer, on the other hand, is a way to refer to some value by its memory address. It is created using the * symbol. For example:

```rust
let s = String::from("hello");
let p = &s as *const String;
```
In this example, p is a pointer to the value of s. It contains the memory address of s, rather than the value itself. Pointers can be dereferenced using the * symbol to access the value they point to.

It's important to notice that when using pointers, you must ensure that the memory they point to is valid, otherwise you'll get a runtime error. Rust uses a pointer type Option<Box<T>> to handle null or dangling pointer.

In summary, & is used for creating references and * is used for creating pointers. Both references and pointers allow you to refer to a value without taking ownership of it, but they have different behaviors and uses.