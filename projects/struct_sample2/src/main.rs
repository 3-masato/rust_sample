mod shape;

fn main() {
    let rect1 = shape::Rectangle::new(35, 70);
    let square1 = shape::Rectangle::square(100);
    println!("rect1 area: {}", rect1.area());
    println!("square1 area: {}", square1.area());
}
