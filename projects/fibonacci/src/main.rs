use std::io;

fn fibonacci(x: u64) -> u64 {
    return match x {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci(x - 2) + fibonacci(x - 1),
    };
}

fn main() {
    println!("Calculate the Nth Fibonacci number: ");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let num: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("The {}th Fibonacci number is {}", num, fibonacci(num));
        break;
    }
}
