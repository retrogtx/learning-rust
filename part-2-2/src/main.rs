use std::collections::HashMap;

fn main(){
    let users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("amrit"), 20);
    println!("{:?}", users);

    let age = users.get("amrit");
    
    match age {
        Some(age) => println!("amrit is {} years old", age),
        None => println!("amrit is not in the hashmap"),
    }
}
