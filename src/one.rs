use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let answer = sum(input);
    println!("Answer: {}", answer);
}

fn parse(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

fn split(input: &str) -> Vec<&str> {
    if input == "" {
        vec![]
    } else {
        input.split(", ").collect::<Vec<&str>>()
    }
}

fn sum(input: &str) -> i32 {
    split(input).iter().map(|&s| parse(s)).sum()
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
    fn test_split() {
        let empty: Vec<&str> = vec![];
        assert_eq!(split(""), empty);
        assert_eq!(split("one"), vec!["one"]);
        assert_eq!(split("one, two, three"), vec!["one", "two", "three"]);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(""), 0);
        assert_eq!(sum("+1"), 1);
        assert_eq!(sum("+1, -3"), -2);
    }
}
