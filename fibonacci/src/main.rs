use std::io;
fn main() {
    let mut x = String::new();
    println!("Enter the index number n");
    io::stdin()
        .read_line(&mut x)
        .expect("Failure to read input");

    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let ret = fibonacci(x);
    println!("The {x} the fibonacci number is {ret}");
}
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut fib: u64 = 0;

    for _ in 2..=n {
        fib = a + b;
        a = b;
        b = fib;
    }

    fib
}
