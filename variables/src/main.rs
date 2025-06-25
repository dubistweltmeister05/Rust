use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Enter an index please");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Couldn't read the line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The number at index {index} is {element}");
}
