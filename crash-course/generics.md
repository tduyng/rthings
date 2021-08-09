## Generic functions

Functions can be generic:

```rust
fn foobar<T>(arg: T) {
    // do something with `arg`
}
```

They can have multiple type parameters, 
which can then be used in the function's declaration and its body, 
instead of concrete types:

```rust
fn foobar<L, R>(left: L, right: R) {
    // do something with `left` and `right`
}
```

Type parameters usually have constraints, so you can actually do something with them.

The simplest constraints are just trait names:

```rust
fn print<T: Display>(value: T) {
    println!("value = {}", value);
}

fn print<T: Debug>(value: T) {
    println!("value = {:?}", value);
}
```

There's a longer syntax for type parameter constraints:
```rust
fn print<T>(value: T)
where
    T: Display,
{
    println!("value = {}", value);
}

```

Constraints can be more complicated: they can require a type parameter to implement multiple traits:
```rust
use std::fmt::Debug;

fn compare<T>(left: T, right: T)
where
    T: Debug + PartialEq,
{
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}

fn main() {
    compare("tea", "coffee");
    // prints: "tea" != "coffee"
}

```

Generic functions can be thought of as namespaces, 
containing an infinity of functions with different concrete types.

Same as with crates, and modules, and types, generic functions can be "explored" (navigated?) using `::`
```rust
fn main() {
    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"
}
```

This is lovingly called turbofish syntax, because ::<> looks like a fish.


Structs can be generic too:
```rust
struct Pair<T> {
    a: T,
    b: T,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1); // prints "Pair<i32>"
    print_type_name(&p2); // prints "Pair<bool>"
}
```

The standard library type Vec (~ a heap-allocated array), is generic:

```rust
fn main() {
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"
}
```

Speaking of Vec, it comes with a macro that gives more or less "vec literals":

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![true, false, true];
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"
}
```

All of `name!()`, `name![]` or `name!{}` invoke a macro. Macros just expand to regular code.

In fact, `println` is a macro:

```rust
fn main() {
    println!("{}", "Hello there!");
}
```

This expands to something that has the same effect as:

```rust
fn main() {
    use std::io::{self, Write};
    io::stdout().lock().write_all(b"Hello there!\n").unwrap();
}
```

`panic` is also a macro. It violently stops execution with an error message, and the file name / line number of the error, if enabled:

```rust
fn main() {
    panic!("This panics");
}
// output: thread 'main' panicked at 'This panics', src/main.rs:3:5
```
Some methods also `panic`. For example, the Option type can contain something, or it can contain nothing. 
If `.unwrap()` is called on it, and it contains nothing, it panics:

```rust
fn main() {
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    o2.unwrap(); // this panics!
// output: thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:378:21
}
```

Option is not a struct - it's an enum, with two variants.
```rust
enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        // enums variants can be used in patterns:
        match self {
            Self::Some(t) => t,
            Self::None => panic!(".unwrap() called on a None option"),
        }
    }
}

use self::Option::{None, Some};

fn main() {
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    o2.unwrap(); // this panics!
}

// output: thread 'main' panicked at '.unwrap() called on a None option', src/main.rs:11:27
```

Result is also an enum, it can either contain something, or an error:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
It also panics when unwrapped and containing an error.

