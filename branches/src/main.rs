fn main() {
    println!("Hello, world!");
    if_branches();
    else_if_branches();
    if_let_branches();
    // loop_branches();
    while_branches();
    for_branches();
}

fn if_branches() {
    let number = 7;
    println!("The value of number is {}", number);
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn else_if_branches() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 2, 3");
    }
}

fn if_let_branches() {
    let condition = true;
    let number = if condition {
        5
    } else {
        10
    };
    println!("The value of number is {}", number);
}

fn loop_branches() {
    loop {
        println!("again!");
    }
}

fn while_branches() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn for_branches() {
    let a = [1,2,3,4,5];
    for element in a.iter() {
        println!("the value is {}", element);
    }

    for number in (1..4).rev() {
        println!{"{}", number};
    }
}