use std::fs;
use std::io::{BufReader, Read};

use day_03::parse_numbers;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-03/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = part1(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let numbers = parse_numbers(input)?;

    let total = numbers
        .iter()
        .filter_map(|n| {
            if n.is_adjacent_symbol(&lines) {
                Some(n.value_as_u64())
            } else {
                None
            }
        })
        .sum::<u64>();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(INPUT).unwrap(), "4361");
    }
}
