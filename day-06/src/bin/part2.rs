use std::fs;
use std::io::{BufReader, Read};

use day_06::{merge_records, parse_input};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-06/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = part2(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let (_, records) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let record = merge_records(&records);
    let value = record.count_win_ways();
    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    Time:      7  15   30
    Distance:  9  40  200
    "#};

    #[test]
    fn test_part2() {
        let answer = part2(INPUT).unwrap();
        assert_eq!(answer, "71503");
    }
}
