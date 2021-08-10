## Closure in Rust

Closures are just functions of type Fn, FnMut or FnOnce with some captured context.

Their parameters are a comma-separated list of names within a pair of pipes (|). 
They don't need curly braces, unless you want to have multiple statements.

```rust
fn for_each_planet<F>(f: F)  where F: Fn(&'static str) {
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn main(){
  for_each_planet(|planet| println!("Hello, {}", planet))
}
```

The borrow rules apply to them too:

```rust
fn for_each_planet<F>(f: F) when F: Fn(&'static str){
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn main(){
  let greeting = String::from("Good to see you");
  for_each_planet(|planet| println!("{}, {}", greeting, planet))
}
```

For example, this would not work:
```rust
fn for_each_planet<F>(f: F)
    where F: Fn(&'static str) + 'static // `F` must now have "'static" lifetime
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn main() {
    let greeting = String::from("Good to see you");
    for_each_planet(|planet| println!("{}, {}", greeting, planet));
    // error: closure may outlive the current function, but it borrows
    // `greeting`, which is owned by the current function
}
```


But this would:

```rust
fn main() {
    let greeting = String::from("You're doing great");
    for_each_planet(move |planet| println!("{}, {}", greeting, planet));
    // `greeting` is no longer borrowed, it is *moved* into
    // the closure.
}
```

An FnMut needs to be mutably borrowed to be called, so it can only be called once at a time.

This is legal:

```rust
fn foobar<F>(f: F)
    where F: Fn(i32) -> i32
{
    println!("{}", f(f(2))); 
}
 
fn main() {
    foobar(|x| x * 2);
}
// output: 8
```

This isn't:
```rust
fn foobar<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    println!("{}", f(f(2))); 
    // error: cannot borrow `f` as mutable more than once at a time
}
 
fn main() {
    foobar(|x| x * 2);
}

```

This is legal again
```rust
fn foobar<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    let tmp = f(2);
    println!("{}", f(tmp)); 
}
 
fn main() {
    foobar(|x| x * 2);
}

// output: 8
```

`FnMut` exists because some closures mutably borrow local variables:

```rust
fn foobar<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    let tmp = f(2);
    println!("{}", f(tmp)); 
}
 
fn main() {
    let mut acc = 2;
    foobar(|x| {
        acc += 1;
        x * acc
    });
}

// output: 24
```

Those closures cannot be passed to functions expecting Fn:
```rust
fn foobar<F>(f: F)
    where F: Fn(i32) -> i32
{
    println!("{}", f(f(2))); 
}
 
fn main() {
    let mut acc = 2;
    foobar(|x| {
        acc += 1;
        // error: cannot assign to `acc`, as it is a
        // captured variable in a `Fn` closure.
        // the compiler suggests "changing foobar
        // to accept closures that implement `FnMut`"
        x * acc
    });
}
```

`FnOnce` closures can only be called once. 
They exist because some closure move out variables that have been moved when captured:

```rust
fn foobar<F>(f: F)
    where F: FnOnce() -> String
{
    println!("{}", f()); 
}
 
fn main() {
    let s = String::from("alright");
    foobar(move || s);
    // `s` was moved into our closure, and our
    // closures moves it to the caller by returning
    // it. Remember that `String` is not `Copy`.
}
```

This is enforced naturally, as `FnOnce` closures need to be moved in order to be called.

So, for example, this is illegal:

```rust
fn foobar<F>(f: F)
    where F: FnOnce() -> String
{
    println!("{}", f()); 
    println!("{}", f()); 
    // error: use of moved value: `f`
}
```

And, if you need convincing that our closure does move s, this is illegal too:

```rust
fn main() {
    let s = String::from("alright");
    foobar(move || s);
    foobar(move || s);
    // use of moved value: `s`
}
```

But this is fine:

```rust
fn main() {
    let s = String::from("alright");
    foobar(|| s.clone());
    foobar(|| s.clone());
}
```

Here's a closure with two arguments:
```rust
fn foobar<F>(x: i32, y: i32, is_greater: F)
    where F: Fn(i32, i32) -> bool
{
    let (greater, smaller) = if is_greater(x, y) {
        (x, y)
    } else {
        (y, x)
    };
    println!("{} is greater than {}", greater, smaller);
}
 
fn main() {
    foobar(32, 64, |x, y| x > y);
}
```

Here's a closure ignoring both its arguments:

```rust
fn main() {
    foobar(32, 64, |_, _| panic!("Comparing is futile!"));
}

```

Here's a slightly worrying closure:

```rust
fn countdown<F>(count: usize, tick: F)
    where F: Fn(usize)
{
    for i in (1..=count).rev() {
        tick(i);
    }
}
 
fn main() {
    countdown(3, |i| println!("tick {}...", i));
}

// output:
// tick 3...
// tick 2...
// tick 1...
```
And here's a toilet closure:

```rust
fn main() {
    countdown(3, |_| ());
}
```

Called thusly because |_| () looks like a toilet.

