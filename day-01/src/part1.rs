use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines: Vec<_> = input.lines().collect();
    let sum = lines
        .iter()
        .filter(|line| line.chars().any(char::is_alphanumeric))
        .fold(0, |acc, &line| {
            let mut digits = Vec::new();
            line.chars().for_each(|c| {
                if let Some(n) = c.to_digit(10) {
                    digits.push(n)
                }
            });
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
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
