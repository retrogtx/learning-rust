fn main() {
    stack_fn();
    update_string();
    heap_fn();
}

fn stack_fn(){
    let a: i32 = 10;
    let b: i32 = 20;
    let c = a + b; 
    println!("Stack funtion: The sum of {}, {} is {}", a, b, c);
}

fn heap_fn(){
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{}, {}", s1, s2);
    println!("Heap function: Combined String is '{}'", combined);
}

fn update_string(){
    let mut s = String::from("Initial String");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    s.push_str("Additional");
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    println!("After update: {}", s);
}
