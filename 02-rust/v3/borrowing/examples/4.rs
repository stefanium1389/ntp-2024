fn main() {
    let mut s = String::from("hello, ");

    let p = &mut s; // TODO
    
    p.push_str("world");

    println!("Success!");
}