use std::fs;
use std::io::{BufReader, Read};

use day_17::{parse_input, Searcher};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-17/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let s = Searcher {
        data: &data,
        consecutive: 4..=10,
    };
    let heat_loss = s.search();

    Ok(heat_loss.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "94");
    }
}
