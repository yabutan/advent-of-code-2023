use anyhow::Context;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(fs::File::open("day-01/data/input.txt")?);

    let mut total = 0;
    for line in r.lines() {
        let line = line?;
        let num = parse_line(&line)?.context("No number found")?;
        total += num;
    }

    println!("answer: {}", total);
    Ok(())
}

fn parse_line(line: &str) -> anyhow::Result<Option<u32>> {
    let left = line.chars().find(|x| x.is_numeric());
    let right = line.chars().rev().find(|x| x.is_numeric());

    match (left, right) {
        (Some(left), Some(right)) => {
            let num: u32 = format!("{}{}", left, right).parse()?;
            Ok(Some(num))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! { r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#};

        let lines: Vec<_> = input.lines().collect();

        assert_eq!(parse_line(lines[0]).unwrap(), Some(12));
        assert_eq!(parse_line(lines[1]).unwrap(), Some(38));
        assert_eq!(parse_line(lines[2]).unwrap(), Some(15));
        assert_eq!(parse_line(lines[3]).unwrap(), Some(77));
    }
}
