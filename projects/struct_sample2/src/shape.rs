pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width: width, height: height }
    }

    pub fn square(size: u32) -> Self {
        Rectangle { width: size, height: size }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
