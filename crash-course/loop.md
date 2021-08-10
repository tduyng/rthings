## Some notes about loop in Rust

Anything that is iterable can be used in a for in loop.

We've just seen a range being used, but it also works with a Vec:

```rust
fn main() {
    for i in vec![52, 49, 21] {
        println!("I like the number {}", i);
    }
}
```

Or a slice:
```rust
fn main() {
    for i in &[52, 49, 21] {
        println!("I like the number {}", i);
    }
}

// output:
// I like the number 52
// I like the number 49
// I like the number 21
```

Or an actual iterator:

```rust
fn main() {
    // note: `&str` also has a `.bytes()` iterator.
    // Rust's `char` type is a "Unicode scalar value"
    for c in "rust".chars() {
        println!("Give me a {}", c);
    }
}

// output:
// Give me a r
// Give me a u
// Give me a s
// Give me a t

```

Even if the iterator items are filtered and mapped and flattened:

```rust
fn main() {
    for c in "SuRPRISE INbOUND"
        .chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase())
    {
        print!("{}", c);
    }
    println!();
}

// output: UB
```

You can return a closure from a function:

```rust
fn make_tester(answer: String) -> impl Fn(&str) -> bool {
    move |challenge| {
        challenge == answer
    }
}

fn main() {
    // you can use `.into()` to perform conversions
    // between various types, here `&'static str` and `String`
    let test = make_tester("hunter2".into());
    println!("{}", test("******"));
    println!("{}", test("hunter2"));
}
````

You can even move a reference to some of a function's arguments, into a closure it returns:

```rust
fn make_tester<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| {
        challenge == answer
    }
}

fn main() {
    let test = make_tester("hunter2");
    println!("{}", test("*******"));
    println!("{}", test("hunter2"));
}

// output:
// false
// true
```

Or, with elided lifetimes:

```rust
fn make_tester(answer: &str) -> impl Fn(&str) -> bool + '_ {
    move |challenge| {
        challenge == answer
    }
}
```