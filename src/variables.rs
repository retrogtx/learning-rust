fn main() {
    let mut x: i8 = 1;
    
    for _i in 1..10 {
        x = x + 1;
        println!("{}", x);
    }

    let name = "Amrit";
    let age: u8 = 20;
    let height: f32 = 5.11;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {}", height);
}
