use itertools::Itertools;

fn str_to_u32(input: &str) -> u32 {
    return input.parse::<u32>().unwrap();
}

fn to_carry(input: &str) -> u32 {
    return input.lines().map(str_to_u32).sum();
}

pub fn part_one(input: &str) -> u32 {
    return input.split("\n\n").map(to_carry).max().unwrap();
}

pub fn part_two(input: &str) -> u32 {
    let elves: Vec<u32> = input.split("\n\n").map(to_carry).sorted().collect();
    return elves[elves.len() - 3..elves.len()].iter().sum()
}

#[cfg(test)]
mod tests {
    use advent_of_code_2022::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 24000);
    }

    #[test]
    fn test_part_one_real() {
        let input = read_file("inputs", 1);
        assert_eq!(part_one(&input), 69626);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 45000);
    }

    #[test]
    fn test_part_two_real() {
        let input = read_file("inputs", 1);
        assert_eq!(part_two(&input), 206780);
    }
}
