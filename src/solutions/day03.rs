use std::collections::HashSet;
use std::str::Chars;

pub struct Input {
    pub game: String,
    pub first: Vec<char>,
    pub second: Vec<char>,
}

impl Input {
    fn new(play_str: &str) -> Input {
        let char_vec: Chars = play_str.chars().ve;
        let size = char_vec.le;
        let middle = size / 2;
        Input {
            game: play_str.to_string(),
            first: Vec::from(char_vec[1..4]),
            second: Vec::from(char_vec[1..4]),
        }
    }

    fn find_matching_char(&self) -> char {
        let mut a = HashSet::from_iter(self.first.iter());
        let mut b = HashSet::from_iter(self.second.iter());
        let res: HashSet<char> = a.intersection(&b).collect();
        let c = Vec::from_iter(res.iter()).first().unwrap();
        return **c;
    }
}

fn to_value(input: char) -> u32 {
    return input.to_digit(26).unwrap();
}

pub fn part_one(input: &str) -> u32 {
    let a: u32 = input
        .lines()
        .filter(|x| x.len() > 0)
        .map(Input::new)
        .map(to_value)
        // .map(Input::find_matching_char)
        .size()
        .trailing_zeros();
    println!("{}", a);
    return a;
}

pub fn part_two(input: &str) -> u32 {
    let a: u32 = input
        .lines()
        .filter(|x| x.len() > 0)
        .map(Input::new)
        .map(to_value)
        .size()
        .trailing_zeros();
    return a;
}

#[cfg(test)]
mod tests {
    use advent_of_code_2022::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_one_real() {
        let input = read_file("inputs", 3);
        assert_eq!(part_one(&input), 12679);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), 12);
    }

    #[test]
    fn test_part_two_real() {
        let input = read_file("inputs", 3);
        assert_eq!(part_two(&input), 14470);
    }
}
