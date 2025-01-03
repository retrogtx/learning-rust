struct User {
    first_name: String,
    last_name: String,
    email: String,
    age: i32
}

struct Rect{
    width: u32,
    height: u32
}

impl Rect{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main(){
    let user = User {
        first_name: String::from("Amrit"),
        last_name: String::from("Rai"),
        email: String::from("iamritrai27@gmail.com"),
        age: 20
    };

    println!("{}", user.first_name);

    let rect = Rect{
        width: 10,
        height: 20
    };

    println!("{}", rect.area());
}
