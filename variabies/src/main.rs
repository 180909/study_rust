fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("Now, The value of x is {}", x);

    let num = 10;
    println!("The first value of num is {}", num);

    let num = num + 10;
    println!("The twice value of num is {}", num);

    let num = num * 5;
    println!("The third value of num is {}", num);

    let spaces = "     ";
    println!("The spaces value is {}...", spaces);

    let spaces = spaces.len();
    println!("Now, The spaces value is {}...", spaces);


    // + - * / %
    // +
    let sum = 10 + 5;
    let difference = 95.5 - 32.3;
    let product = 4 * 56;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("+ {}; - {}; * {}; / {}; % {}", sum, difference, product, quotient, remainder);

    // bool
    let t = true;
    let f: bool = false;
    println!("t {}, f {}", t, f);

    // char
    let min_cast = 'z';
    let max_cast = 'Z';
    println!("c {}, C {}", min_cast, max_cast);

    // tuple: 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x {}, y {}, z {}", x, y, z);
    println!("1 {}, 2 {}, 3 {}", tup.0, tup.1, tup.2);

    // array: 数组
    let arr = [1, 2, 3, 4, 5];
    println!("1 {}, 2 {}, 3{}, 4{}, 5{}", arr[0], arr[1], arr[2], arr[3], arr[4]);
}
