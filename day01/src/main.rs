use aho_corasick::AhoCorasick;
use std::include_str;

fn day01a(input_text: &'static str) -> i32 {
    input_text
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c: char| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|list| {
            format!("{}{}", list.first().unwrap(), list.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn day01b(input_text: &'static str) -> i32 {
    let patterns = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    input_text
        .lines()
        .map(|l| {
            ac.find_overlapping_iter(l)
                .map(|mat| mat.pattern().as_usize() % 9 + 1)
                .collect::<Vec<_>>()
        })
        .map(|list| {
            format!("{}{}", list.first().unwrap(), list.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn main() {
    let input_text = include_str!("./day01");
    println!("{:?}", day01a(input_text));
    println!("{:?}", day01b(input_text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let a = include_str!("day01a_test");
        assert_eq!(142, day01b(a));
    }
    #[test]
    fn test_b() {
        let b = include_str!("day01b_test");
        assert_eq!(281, day01b(b))
    }
}
