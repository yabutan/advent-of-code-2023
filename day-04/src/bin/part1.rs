use std::fs;
use std::io::{BufRead, BufReader};

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{space0, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-04/data/input.txt")?);
    let answer = part1(&mut r)?;
    println!("answer: {}", answer);
    Ok(())
}

fn part1(r: &mut impl BufRead) -> anyhow::Result<String> {
    let mut total = 0;
    for line in r.lines() {
        let line = line?;
        let (_, card) = parse_card(&line).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let (_, point) = card.get_winning();
        total += point;
    }

    Ok(total.to_string())
}

#[derive(Debug, Eq, PartialEq)]
struct Card {
    number: u32,
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Card {
    fn get_winning(&self) -> (Vec<u32>, u32) {
        let winning_numbers = self
            .left
            .iter()
            .filter(|n| self.right.contains(n))
            .cloned()
            .collect::<Vec<_>>();

        let point = if winning_numbers.is_empty() {
            0
        } else {
            2u32.pow(winning_numbers.len() as u32 - 1)
        };

        (winning_numbers, point)
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, number) = delimited(
        tuple((tag("Card"), space0)),
        complete::u32,
        tuple((tag(":"), space0)),
    )(input)?;
    let (input, left) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = tuple((space0, tag("|"), space0))(input)?;
    let (input, right) = separated_list1(space1, complete::u32)(input)?;

    Ok((
        input,
        Card {
            number,
            left,
            right,
        },
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#};

    #[test]
    fn test_parse_card() {
        let lines = INPUT.lines().collect::<Vec<_>>();

        let (_, card) = parse_card(lines[0]).unwrap();
        assert_eq!(
            card,
            Card {
                number: 1,
                left: vec![41, 48, 83, 86, 17],
                right: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }

    #[test]
    fn test_get_winning() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        let cards = lines
            .into_iter()
            .map(|line| parse_card(line).unwrap().1)
            .collect::<Vec<_>>();

        let expects = [
            (vec![48, 83, 86, 17], 8),
            (vec![32, 61], 2),
            (vec![1, 21], 2),
            (vec![84], 1),
            (vec![], 0),
            (vec![], 0),
        ];

        for (i, card) in cards.iter().enumerate() {
            assert_eq!(card.get_winning(), expects[i]);
        }
    }

    #[test]
    fn test_part1_example() {
        let mut r = INPUT.as_bytes();
        assert_eq!(part1(&mut r).unwrap(), "13");
    }

    #[test]
    fn test_pow() {
        assert_eq!(2u32.pow(0), 1);
        assert_eq!(2u32.pow(1), 2);
        assert_eq!(2u32.pow(2), 4);
        assert_eq!(2u32.pow(3), 8);
    }
}
