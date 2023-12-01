use std::fs;
use std::io::{BufRead, BufReader};
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(fs::File::open("day-01/data/input.txt")?);

    let mut total = 0;
    for line in r.lines() {
        let line = line?;
        let num = parse_line_with_spelled(&line)?.context("No number found")?;
        total += num;
    }

    println!("answer: {}", total);
    Ok(())
}

const SPELLED: [(u32, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];


fn find(line: &str) -> Option<u32> {
    // (index, value)
    let mut x = line.find(char::is_numeric)
        .map(|i| (i, line.chars().nth(i).unwrap()))
        .map(|(i, c)| (i, c.to_digit(10).unwrap()));

    for (value, word) in SPELLED {
        if let Some(i) = line.find(word) {
            match x {
                Some((ix, _)) => {
                    if i < ix {
                        x = Some((i, value));
                    }
                }
                None => {
                    x = Some((i, value));
                }
            }
        }
    }

    Some(x?.1)
}


fn rfind(line: &str) -> Option<u32> {
    // (index, value)
    let mut x = line.rfind(char::is_numeric)
        .map(|i| (i, line.chars().nth(i).unwrap()))
        .map(|(i, c)| (i, c.to_digit(10).unwrap()));

    for (value, word) in SPELLED {
        if let Some(i) = line.rfind(word) {
            match x {
                Some((ix, _)) => {
                    if i > ix {
                        x = Some((i, value));
                    }
                }
                None => {
                    x = Some((i, value));
                }
            }
        }
    }

    Some(x?.1)
}

fn parse_line_with_spelled(line: &str) -> anyhow::Result<Option<u32>> {
    let left = find(line);
    let right = rfind(line);

    match (left, right) {
        (Some(left), Some(right)) => {
            let num = (left * 10) + right;
            Ok(Some(num))
        }
        _ => Ok(None),
    }
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! { r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        "#};

        let lines: Vec<_> = input.lines().collect();

        assert_eq!(parse_line_with_spelled(lines[0]).unwrap(), Some(29));
        assert_eq!(parse_line_with_spelled(lines[1]).unwrap(), Some(83));
        assert_eq!(parse_line_with_spelled(lines[2]).unwrap(), Some(13));
        assert_eq!(parse_line_with_spelled(lines[3]).unwrap(), Some(24));
        assert_eq!(parse_line_with_spelled(lines[4]).unwrap(), Some(42));
        assert_eq!(parse_line_with_spelled(lines[5]).unwrap(), Some(14));
        assert_eq!(parse_line_with_spelled(lines[6]).unwrap(), Some(76));
    }
}