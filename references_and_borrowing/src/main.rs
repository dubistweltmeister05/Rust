fn main(){
    let m1=String::from("Hello!");
    let m2=String::from("Hello!");
    greet(&m1, &m2);
    let s = format!("{}, {}",m1,m2);
}

fn greet (S1:&str, S2:&str)  {
    println!("{}, {}",S1, S2);

}