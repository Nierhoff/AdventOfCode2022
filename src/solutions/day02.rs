use crate::solutions::day02::RPS::{Paper, Rock, Scissors, Unkown};

pub struct Input {
    pub game: String,
    pub first: char,
    pub second: char,
}

pub enum RPS {
    Rock,
    Paper,
    Scissors,
    Unkown,
}

impl std::fmt::Display for RPS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RPS::Rock => write!(f, "Rock"),
            RPS::Paper => write!(f, "Paper"),
            RPS::Scissors => write!(f, "Scissors"),
            RPS::Unkown => write!(f, "Unknown"),
        }
    }
}

impl Input {
    fn new(play_str: &str) -> Input {
        let char_vec: Vec<char> = play_str.chars().collect();

        Input {
            game: play_str.to_ascii_uppercase(),
            first: char_vec.first().unwrap().to_ascii_uppercase(),
            second: char_vec.last().unwrap().to_ascii_uppercase(),
        }
    }
}

pub struct Play {
    pub their_play: RPS,
    pub our_play: RPS,
    // A=Rock, B=Paper, C=Scissors
}

impl Play {
    fn new_part2(input: Input) -> Play {
        let their_play: RPS = match input.first {
            'A' => RPS::Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => Unkown,
        };
        let our_play: RPS = match (input.first, input.second) {
            // X=Loss Y=Draw, Z=Win
            ('A', 'X') => Scissors, // loss
            ('A', 'Y') => Rock, // draw
            ('A', 'Z') => Paper, // win
            ('B', 'X') => Rock,
            ('B', 'Y') => Paper,
            ('B', 'Z') => Scissors,
            ('C', 'X') => Paper,
            ('C', 'Y') => Scissors,
            ('C', 'Z') => Rock,
            _ => Unkown,
        };

        Play {
            their_play: their_play,
            our_play: our_play,
        }
    }

    fn new_part1(input: Input) -> Play {
        let their_play: RPS = match input.first {
            'A' => RPS::Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => Unkown,
        };
        let our_play: RPS = match input.second {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => Unkown,
        };

        Play {
            their_play: their_play,
            our_play: our_play,
        }
    }

    fn get_winner_score(&self) -> u32 {
        let score: u32 = match (&self.their_play, &self.our_play) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 6,
            (Rock, Scissors) => 0,
            (Paper, Rock) => 0,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 6,
            (Scissors, Rock) => 6,
            (Scissors, Paper) => 0,
            (Scissors, Scissors) => 3,
            _ => 0,
        };
        return score;
    }

    fn get_play_value(&self) -> u32 {
        return match self.our_play {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
            _ => 0,
        };
    }

    fn get_score(&self) -> u32 {
        let score: u32 = &self.get_winner_score() + &self.get_play_value();
        println!(
            "game: {} vs {}, score: {}",
            &self.their_play, self.our_play, score
        );
        return score;
    }
}

pub fn part_one(input: &str) -> u32 {
    let a: u32 = input
        .lines()
        .filter(|x| x.len() > 0)
        .map(Input::new)
        .map(Play::new_part1)
        .map(|x| x.get_score())
        .sum();
    println!("{}", a);
    return a;
}

pub fn part_two(input: &str) -> u32 {
    let a: u32 = input
        .lines()
        .filter(|x| x.len() > 0)
        .map(Input::new)
        .map(Play::new_part2)
        .map(|x| x.get_score())
        .sum();
    return a;
}

#[cfg(test)]
mod tests {
    use advent_of_code_2022::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_one_real() {
        let input = read_file("inputs", 2);
        assert_eq!(part_one(&input), 12679);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 12);
    }

    #[test]
    fn test_part_two_real() {
        let input = read_file("inputs", 2);
        assert_eq!(part_two(&input), 14470);
    }
}
