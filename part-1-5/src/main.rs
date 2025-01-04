enum Direction{
    North, 
    South,
    East,
    West
}

enum Shape{
    Rectangle(f32, f32),
    Circle(f32),
    Square(f32)
}
fn main(){
    let _dir = Direction::North;
    print_area(Shape::Rectangle(10.0, 20.0));
}

fn print_area(shape: Shape){
    match shape{
        Shape::Rectangle(width, height) => println!("Rectangle: {}", width * height), 
        Shape::Circle(radius) => println!("Circle: {}", radius * radius * 3.14),
        Shape::Square(side) => println!("Square: {}", side * side)
    }
}