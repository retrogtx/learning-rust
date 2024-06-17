fn main() {
    let greeting: String = String::from("Hello World!");
    println!("{}", greeting);

    let char1: Option<char> = greeting.chars().nth(0);
    println!("char1: {}", char1.unwrap());
}