use std::fs;
use std::io::{BufReader, Read};

use itertools::Itertools;

use day_03::parse_numbers;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-03/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = part2(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let numbers = parse_numbers(input)?;

    let grouping = numbers
        .into_iter()
        .into_group_map_by(|number| number.gear_pos(&lines));

    let mut total = 0;
    for (gear_pos, numbers) in grouping {
        if gear_pos.is_none() || numbers.len() == 1 {
            // ギアで繋がりないものは除外する。
            continue;
        }

        total += numbers
            .into_iter()
            .map(|n| n.value_as_u64())
            .product::<u64>();
    }

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
    fn test_part2_example() {
        assert_eq!(part2(INPUT).unwrap(), "467835");
    }
}
