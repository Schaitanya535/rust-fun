use crate::utils;

pub fn part1(input: &str) -> i64 {
    let grid = utils::read_char_grid(input);

    let mut count = 0;
    fn check_adjacent(grid: &[Vec<char>], i: usize, j: usize) -> bool {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        utils::DIRS_8
            .iter()
            .filter(|(dr, dc)| {
                let ni = i as i32 + dr;
                let nj = j as i32 + dc;
                ni >= 0
                    && ni < rows
                    && nj >= 0
                    && nj < cols
                    && grid[ni as usize][nj as usize] == '@'
            })
            .count()
            < 4
    }

    let rows = grid.len();
    let col = grid[0].len();

    for i in 0..rows {
        for j in 0..col {
            if grid[i][j] == '@' && check_adjacent(&grid, i, j) {
                count = count + 1
            }
        }
    }
    count
}

pub fn part2(input: &str) -> i64 {
    let mut grid = utils::read_char_grid(input);

    let mut final_count = 0;
    fn check_adjacent(grid: &[Vec<char>], i: usize, j: usize) -> bool {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        utils::DIRS_8
            .iter()
            .filter(|(dr, dc)| {
                let ni = i as i32 + dr;
                let nj = j as i32 + dc;
                ni >= 0
                    && ni < rows
                    && nj >= 0
                    && nj < cols
                    && grid[ni as usize][nj as usize] == '@'
            })
            .count()
            < 4
    }

    let rows = grid.len();
    let col = grid[0].len();

    loop {
        let mut count = 0;
        for i in 0..rows {
            for j in 0..col {
                if grid[i][j] == '@' && check_adjacent(&grid, i, j) {
                    grid[i][j] = '.';
                    count = count + 1;
                }
            }
        }
        final_count += count;
        if count == 0 {
            break;
        }
    }
    final_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../../inputs/2025/day04_sample.txt");
    const ACTUAL: &str = include_str!("../../../inputs/2025/day04.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13);
        assert_eq!(part1(ACTUAL), 1502);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE), 43);
        assert_eq!(part2(ACTUAL), 9083);
    }

    // Uncomment once you have your real input and know the answer:
    // const ACTUAL: &str = include_str!("../../../../inputs/2025/day01.txt");
    // #[test]
    // fn test_part1_actual() {
    //     assert_eq!(part1(ACTUAL), "your_answer_here");
    // }
}
