# Match

In Rust, the match keyword is used to perform a switch-like function. It is used to match a value against a series of patterns and execute code based on the first pattern that matches. The syntax is as follows:

```rust
match variable {
    pattern1 => code1,
    pattern2 => code2,
    ...
    _ => code_default,
}
```

The variable is the value that you want to match against the patterns. Each pattern is followed by an arrow (=>) and the code that should be executed if the pattern matches. The underscore \_ pattern acts as a default case, similar to the default case in a traditional switch statement.

For example:

```rust
let num = 5;
match num {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

This will print "five" because 5 matches the fifth pattern.
It is important to know that Rust's match statement is exhaustive, meaning that it will always match one of the patterns defined and will not fall through like a C-style switch statement.
