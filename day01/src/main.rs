use aho_corasick::AhoCorasick;
use std::include_str;

/// Returns the sum of the concatenated value of the first and last number found in `input_text`
///
/// Uses Aho-Corasick to find overlapping matches, and the pattern ID mod 9 + 1 to get the integer value
/// of the match. A bit hacky, and requires the input vec to be ordered i.e. `["1", ..., "9", "one", ..., "nine"]`
/// # Arguments
/// * `input_text` - A string from include_str
/// * `pattern` - The Aho-Corasick pattern to match for.
fn sum_first_last(input_text: &'static str, pattern: &Vec<&str>) -> i32 {
    let ac = AhoCorasick::new(pattern).unwrap();
    input_text
        .lines()
        .map(|line| {
            ac.find_overlapping_iter(line)
                .map(|mat| mat.pattern().as_usize() % 9 + 1) // mod 9 needed for part b
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
    let mut pattern = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    println!("part a: {:?}", sum_first_last(input_text, &pattern));
    pattern.append(&mut vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]);
    println!("part b: {:?}", sum_first_last(input_text, &pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let a = include_str!("day01a_test");
        let a_pat = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        assert_eq!(142, sum_first_last(a, &a_pat));
    }
    #[test]
    fn test_b() {
        let b = include_str!("day01b_test");
        let b_pat = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];
        assert_eq!(281, sum_first_last(b, &b_pat))
    }
}
