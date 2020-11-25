fn main() {
    println!("Hello, world!");

    another_function();

    another_function_tow(5);

    another_function_three(5, 10);

    macro_call();

    let x = go_back();
    println!("The value of go_back is {}", x);
}

fn another_function() {
    println!("Another function");
}

fn another_function_tow(x: i32) {
    println!("The value of x is {}", x);
}

fn go_back() -> i32 {
    5
}

fn another_function_three(x: i32, y:i32) {
    println!("The value of x is {}; The value of y is {}; They add up to {}", x, y, x + y);
}

fn macro_call() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
}