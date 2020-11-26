fn main() {
    let s = String::from("hello world");
    let a = first_word(&s);
    println!("a {}", a);

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello {}, world {}", hello, world);

    let slice = &s[0..2];
    println!("0..2: {}", slice);

    let slice = &s[..2];
    println!("..2: {}", slice);

    let slice = &s[6..s.len()];
    println!("6..len(): {}", slice);

    let slice = &s[6..];
    println!("6..: {}", slice);

    let slice = &s[0..s.len()];
    println!("0..len(): {}", slice);

    let slice = &s[..];
    println!("..: {}", slice);

    let first = first_word_with_slice(&s);
    println!("The first word with '{}' is: {}", s, first);

    let last = last_word_with_slice(&s);
    println!("The second word with '{}' is: {}", s, last);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn last_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().rev().enumerate() {
        if item == b' ' {
            return &s[i..]
        }
    }
    &s[..]
}