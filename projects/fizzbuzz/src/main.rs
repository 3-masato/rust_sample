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

fn main() {
    for x in 1..=100 {
        println!("{}: {}", x, fizzbuzz_basic(x));
    }
}
