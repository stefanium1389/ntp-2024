// Ispravi grešku u kodu.
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}", r1);

    println!("Success!");
}