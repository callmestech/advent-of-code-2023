use std::collections::HashMap;

use crate::custom_error::AocError;
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let digits_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let sum = lines
        .iter()
        .filter(|line| line.chars().any(char::is_alphanumeric))
        .fold(0, |acc, &line| {
            let mut remaining = line;
            let mut digits = Vec::new();
            while let Some(m) = regex.find(remaining) {
                let found_str = m.as_str();
                let found = digits_map
                    .get(found_str)
                    .copied()
                    .or(found_str.parse::<u32>().ok())
                    .unwrap_or_default();
                digits.push(found);
                remaining = &remaining[m.start() + 1..]
            }
            let first = digits.first().unwrap_or(&0);
            let last = digits.last().unwrap_or(first);
            acc + (*first * 10 + *last)
        });
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
