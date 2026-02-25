/// Split input into lines, trimming trailing whitespace.
pub fn lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Parse a line like "1234-1239, 123-234, 51534-54565" into [(1234,1239), (123,234), ...]
pub fn read_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (a, b) = part.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

/// Parse each line into a type that implements FromStr.
pub fn parse_lines<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    input.lines().map(|l| l.parse().unwrap()).collect()
}
