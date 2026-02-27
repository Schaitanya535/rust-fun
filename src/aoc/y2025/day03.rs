fn best_n_digits(bank: &str, n: usize) -> i64 {
    let chars: Vec<char> = bank.chars().collect();
    let len = chars.len();
    let mut idx = 0;
    let mut result: i64 = 0;
    for i in 0..n {
        let remaining = len - (n - i);
        let (best_idx, best_digit) = chars[idx..=remaining]
            .iter()
            .enumerate()
            .max_by(|&(ai, ac), &(bi, bc)| ac.cmp(bc).then(bi.cmp(&ai)))
            .map(|(j, c)| (idx + j, c.to_digit(10).unwrap() as i64))
            .unwrap();
        result = result * 10 + best_digit;
        idx = best_idx + 1;
    }
    result
}

pub fn part2(input: &str) -> i64 {
    input.lines().map(|bank| best_n_digits(bank, 12)).sum()
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(|bank| best_n_digits(bank, 2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../../inputs/2025/day03_sample.txt");
    const ACTUAL: &str = include_str!("../../../inputs/2025/day03.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 357);
        assert_eq!(part1(ACTUAL), 17432);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE), 3121910778619);
        assert_eq!(part2(ACTUAL), 173065202451341);
    }

    // Uncomment once you have your real input and know the answer:
    // const ACTUAL: &str = include_str!("../../../../inputs/2025/day01.txt");
    // #[test]
    // fn test_part1_actual() {
    //     assert_eq!(part1(ACTUAL), "your_answer_here");
    // }
}
