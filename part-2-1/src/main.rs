fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    
    // or let mut vec = vec![1, 2, 3, 4, 5]; <- this is a macro

    println!("{:?}", even_filter(vec));
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut even_vec = Vec::new();
    for i in vec {
        if i % 2 == 0 {
            even_vec.push(i);
        }
    }
    even_vec
}
