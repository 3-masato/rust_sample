// 構造体定義の直前に`#[derive(Debug)]`という注釈を追加
// `Debug` トレイトを導出する注釈を追加することで、構造体を出力できる
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 構造体にメソッドを加える
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 50, height: 60 };

    // これは動かない
    // println!("rect1 is {}", rect1);

    // 構造体を出力するには、`{}` ではなく `{:?}` を使う
    // println!("rect1 is {:?}", rect1);

    // `{:?}` を `{:#?}` とすることで、読みやすく整形されて出力される
    println!("rect1 is {:#?}", rect1);

    println!("The area of the rectangle is {} square pixels", rect1.area());

    let rect2 = Rectangle { width: 30, height: 20 };
    let rect3 = Rectangle { width: 40, height: 70 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(10);

    println!("square1 is {:#?}", square1);
}
