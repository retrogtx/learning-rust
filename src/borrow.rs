fn main() {
    let mut str: String = String::from("hello");
    update_str(&mut str);
}

fn update_str(str: &mut String){
    str.push_str("World");
}

