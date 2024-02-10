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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz() {
        assert_eq!(fizzbuzz_basic(3), "Fizz");
        assert_eq!(fizzbuzz_basic(6), "Fizz");

        assert_eq!(fizzbuzz_match(3), "Fizz");
        assert_eq!(fizzbuzz_match(6), "Fizz");
    }

    #[test]
    fn test_buzz() {
        assert_eq!(fizzbuzz_basic(5), "Buzz");
        assert_eq!(fizzbuzz_basic(10), "Buzz");

        assert_eq!(fizzbuzz_match(5), "Buzz");
        assert_eq!(fizzbuzz_match(10), "Buzz");
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz_basic(15), "FizzBuzz");
        assert_eq!(fizzbuzz_basic(30), "FizzBuzz");

        assert_eq!(fizzbuzz_match(15), "FizzBuzz");
        assert_eq!(fizzbuzz_match(30), "FizzBuzz");
    }

    #[test]
    fn test_number() {
        assert_eq!(fizzbuzz_basic(1), "1");
        assert_eq!(fizzbuzz_basic(2), "2");
        assert_eq!(fizzbuzz_basic(4), "4");

        assert_eq!(fizzbuzz_match(1), "1");
        assert_eq!(fizzbuzz_match(2), "2");
        assert_eq!(fizzbuzz_match(4), "4");
    }
}
