pub fn part1(input: &str) -> String {
    let _lines: Vec<&str> = input.lines().collect();
    "not implemented".to_string()
}

pub fn part2(input: &str) -> String {
    let _lines: Vec<&str> = input.lines().collect();
    "not implemented".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../../inputs/2025/day01_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE), "not implemented");
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE), "not implemented");
    }

    // Uncomment once you have your real input and know the answer:
    // const ACTUAL: &str = include_str!("../../../../inputs/2025/day01.txt");
    // #[test]
    // fn test_part1_actual() {
    //     assert_eq!(part1(ACTUAL), "your_answer_here");
    // }
}
