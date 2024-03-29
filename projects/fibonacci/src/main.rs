use std::collections::HashMap;
use std::io;

fn fibonacci(x: u64) -> u64 {
    let mut memo: HashMap<u64, u64> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);

    fn fib(x: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        return match memo.get(&x) {
            None => {
                let n = fib(x - 2, memo) + fib(x - 1, memo);
                memo.insert(x, n);
                n
            }
            _ => memo[&x],
        };
    }

    fib(x, &mut memo)
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
