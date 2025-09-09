fn main(){
    let m1=String::from("Hello!");
    let m2=String::from("Hello!");
    greet(&m1, &m2);
    let _s = format!("{}, {}",m1,m2);
}

fn greet (s1:&str, s2:&str)  {
    println!("{}, {}",s1, s2);

}