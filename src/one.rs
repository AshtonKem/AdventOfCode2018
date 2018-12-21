use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::Iterator;

fn main() {
    let input: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| parse(&s))
        .collect::<Vec<i32>>();
    let sum: i32 = input.iter().sum();

    println!("Answers: {}, {}", sum, find_repeat(input));
}

fn find_repeat(sequence: Vec<i32>) -> i32 {
    let mut sums = HashSet::new();
    let mut sum = 0;
    for num in sequence.iter().cycle() {
        if sums.contains(&sum) {
            return sum;
        }
        sums.insert(sum);
        sum += num;
    }
    0
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

    #[test]
    fn test_find_repeat() {
        assert_eq!(find_repeat(vec![1, -1]), 0);
        assert_eq!(find_repeat(vec![1, 2, -1]), 3);
    }
}
