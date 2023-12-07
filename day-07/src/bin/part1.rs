use std::fs;
use std::io::{BufReader, Read};

use itertools::Itertools;

use day_07::{parse_input, Hand, NormalRule};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-07/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, lines) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let hands = lines
        .iter()
        .map(|line| (line, Hand::<NormalRule>::new(line.cards)))
        .sorted_by(|(_, a), (_, b)| a.cmp(b))
        .collect::<Vec<_>>();

    let mut total = 0;
    for (i, (line, hand)) in hands.iter().enumerate() {
        let rank = (i + 1) as u32;

        total += line.bid * rank;
        println!("{}: {:?} {:?}", i, line, hand);
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "6440");
    }
}
