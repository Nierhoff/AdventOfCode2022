pub fn part_one(input: &str) -> u32 {
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use advent_of_code_2022::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 9);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 9);
        assert_eq!(part_two(&input), 0);
    }
}
