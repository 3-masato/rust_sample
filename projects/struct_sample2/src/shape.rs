use std::f32::consts::PI;

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

pub struct Circle {
    radius: u32,
}

impl Circle {
    pub fn new(radius: u32) -> Self {
        Circle { radius: radius }
    }

    pub fn area(&self) -> f32 {
        (self.radius.pow(2) as f32) * PI
    }

    pub fn circumference(&self) -> f32 {
        ((self.radius * 2) as f32) * PI
    }
}
