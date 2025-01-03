fn main() {
    println!("{}", fib(10));
    println!("{}", is_even(10));
    println!("{}", get_str_len("Hello, world!"));
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 1..num - 1 {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}

fn get_str_len(str: &str) -> usize {
    return str.len();
}
