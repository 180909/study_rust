fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // change(&s1);
    // we can't change the value of the reference to a string

    let mut str1 = String::from("hello");
    // println!("{}", str1);
    change_string(&mut str1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}