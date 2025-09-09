fn main (){
    let mut x: Box<i32> = Box::new(1);
    let a :i32 = *x;

    let r1: &Box<i32> = &x;
    let b: i32 = **r1;

    let r2: &i32 = &*x;
    let c :i32 = *r2;

    println!("a: {}, b: {}, c: {}", a, b, c);
}