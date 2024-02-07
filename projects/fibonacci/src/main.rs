fn fibonacci(x: u64) -> u64 {
    return match x {
        0 => 0,
        1 => 1,
        _ => fibonacci(x - 2) + fibonacci(x - 1),
    };
}

fn main() {
    let num = fibonacci(50);
    println!("fibonacci: {}", num);
}
