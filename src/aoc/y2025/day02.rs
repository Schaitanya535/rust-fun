use crate::utils;

pub fn part1(input: &str) -> i64 {
    let ranges: Vec<(i64, i64)> = input.lines().flat_map(utils::read_ranges).collect();

    fn is_invalid(num: i64) -> bool {
        let s = num.to_string();
        let mid = s.len() / 2;
        s[..mid] == s[mid..]
    }

    return ranges
        .into_iter()
        .map(|(a, b)| (a..=b).filter(|&num| is_invalid(num)).sum::<i64>())
        .sum();
}

pub fn part2(input: &str) -> String {
    let _lines: Vec<&str> = input.lines().collect();
    "not implemented".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../../inputs/2025/day02_sample.txt");
    const ACTUAL: &str = include_str!("../../../inputs/2025/day02.txt");

    #[test]
    fn test_part1() {
        dbg!(part1(SAMPLE));
        dbg!(part1(ACTUAL));
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
