fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    multiple_loop();
    loop_while();
    loop_collection();
    loop_in();
}

fn multiple_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn loop_while() {
    let mut number = 3;
    while number != 0 {
        println!("number: {number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_collection() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn loop_in(){
    let a = [1,2,3,4,5];
    for e in a {
        println!("The value of arr is: {}", e);
    }
}