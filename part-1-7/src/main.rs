fn main(){
    let index = find_first(String::from("ayo"));
    
    match index{
        Some(value) => println!("Index of 'a': {}", value),
        None => println!("'a' not found")
    }
}

fn find_first(s: String) -> IndexOption<i32> {
    for (index, char) in s.chars().enumerate(){
        if char == 'a'{
            return IndexOption::Some(index as i32);
        }
    }
    IndexOption::None
}