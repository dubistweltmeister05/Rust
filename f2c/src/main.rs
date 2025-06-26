use std::io;
fn main() {
    let mut x = String::new();
    println!("Enter the temperature on Fahrenheit");
    io::stdin()
        .read_line(&mut x)
        .expect("Failure to read input");

    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let out: f64 = (x - 32.0) * (5.0 / 9.0);

    println!("The temperature is {out} celsius");
}
