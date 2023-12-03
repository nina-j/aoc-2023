/// JESUS
/// CHRIST
fn sum_valid_games(input_text: &str) -> i32 {
    let result: Vec<_> = input_text
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();
    let mut sum_games = 0;
    for game in result {
        let game_id = game
            .iter()
            .nth(1)
            .unwrap()
            .replace(":", "")
            .parse::<i32>()
            .unwrap();
        let mut valid = true;
        for c in game.chunks(2) {
            let first = c.first().unwrap();
            let last = c.last().unwrap();
            if last.contains("red") && first.parse::<i32>().unwrap() > 12 {
                valid = false;
            }
            if last.contains("green") && first.parse::<i32>().unwrap() > 13 {
                valid = false;
            }
            if last.contains("blue") && first.parse::<i32>().unwrap() > 14 {
                valid = false;
            }
        }
        if valid {
            sum_games += game_id
        }
    }
    sum_games
}

/// FFS
fn sum_powers(input_text: &str) -> i32 {
    let result: Vec<_> = input_text
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();
    let mut sum_games = 0;
    for game in result {
        let mut reds: Vec<_> = vec![];
        let mut greens: Vec<_> = vec![];
        let mut blues: Vec<_> = vec![];
        for c in game.chunks(2) {
            let first = c.first().unwrap();
            let last = c.last().unwrap();
            if last.contains("red") {
                reds.push(first.parse::<i32>().unwrap())
            }
            if last.contains("green") {
                greens.push(first.parse::<i32>().unwrap())
            }
            if last.contains("blue") {
                blues.push(first.parse::<i32>().unwrap())
            }
        }
        sum_games +=
            reds.iter().max().unwrap() * greens.iter().max().unwrap() * blues.iter().max().unwrap()
    }
    sum_games
}
fn main() {
    let input_text = include_str!("day02");
    println!("part a: {}", sum_valid_games(input_text));
    println!("part b: {}", sum_powers(input_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let a = include_str!("day02_test");
        assert_eq!(8, sum_valid_games(a))
    }
    #[test]
    fn test_b() {
        let b = include_str!("day02_test");
        assert_eq!(2286, sum_powers(b))
    }
}
