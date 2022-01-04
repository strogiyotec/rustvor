#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn from(width: u32, height: u32) -> Rectangle {
        return Rectangle { width, height };
    }
}

fn main() {
    let rectangle = Rectangle::from(30, 40);
    println!("{:?}", rectangle);
    println!("{}", rectangle.area());
}
