use std::io;
use std::io::prelude::*;

fn main() {
    let x: i32 = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| parse(&s))
        .sum();

    println!("Answer: {}", x);
}

fn parse(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("+1"), 1);
        assert_eq!(parse("-1"), -1);
        assert_eq!(parse("+3"), 3);
        assert_eq!(parse("-2"), -2);
        assert_eq!(parse("0"), 0);
    }
}
