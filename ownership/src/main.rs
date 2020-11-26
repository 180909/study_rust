fn main() {
    println!("Hello, world!");
    // create string data
    let mut s = String::from("hello");
    println!("The value of s is: {}", s);

    // appends a literal to string
    s.push_str(", world!");
    println!("The value of s is: {}", s);

    let s2 = s;
    // println!("The value of s2 is: {}", s);
    println!("The value of s2 is: {}", s2);

    string_clone();

    let x = 5;
    let y = x;
    println!("The value of x is: {}\nThe value of y is: {}", x, y);


    takes_ownership(s2);
    makes_copy(x);
    // if your use fn to send String like s2, then, s2 will be drop
    // println!("{}", s2);
    // but other integer or boolean or float like x, they doesn't drop
    println!("{}", x);

    // return string
    let str1 = gives_ownership();
    println!("The value of gives_ownership is {}", str1);
    let str2 = String::from("hello, world");
    let str3 = takes_and_gives_back(str2);
    println!("The value of takes_and_gives_back is {}", str3);

    // return tuple
    let str4 = String::from("world");
    let (str5, len) = calculate_length(str4);
    println!("The length of '{}' is {}", str5, len);
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(a_string: String) -> (String, usize) {
    let len = a_string.len();
    (a_string, len)
}