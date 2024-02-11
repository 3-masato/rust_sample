// 構造体定義の直前に`#[derive(Debug)]`という注釈を追加
// `Debug` トレイトを導出する注釈を追加することで、構造体を出力できる
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle { width: 10, height: 20 };

    // これは動かない
    // println!("rect1 is {}", rect1);

    // 構造体を出力するには、`{}` ではなく `{:?}` を使う
    // println!("rect1 is {:?}", rect1);

    // `{:?}` を `{:#?}` とすることで、読みやすく整形されて出力される
    println!("rect1 is {:#?}", rect1);

    println!("The area of the rectangle is {} square pixels", area(&rect1));
}
