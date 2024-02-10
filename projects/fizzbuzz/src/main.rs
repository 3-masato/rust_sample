fn fizzbuzz_basic(x: u32) -> String {
    if x % 15 == 0 {
        return "FizzBuzz".into();
    } else if x % 3 == 0 {
        return "Fizz".into();
    } else if x % 5 == 0 {
        return "Buzz".into();
    } else {
        return x.to_string();
    }
}

fn fizzbuzz_match(x: u32) -> String {
    return match (x % 3, x % 5) {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        (_, _) => x.to_string(),
    };
}

fn main() {
    for x in 1..=100 {
        println!("{}: {}, {}", x, fizzbuzz_basic(x), fizzbuzz_match(x));
    }
}
