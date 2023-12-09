use std::fs;
use std::io::{BufReader, Read};

use day_09::{get_prev_prediction, get_sequences, parse_input};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-09/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let mut total = 0;
    for line in &data.lines {
        let sequences = get_sequences(line);
        let predict = get_prev_prediction(&sequences);
        println!("line:{:?} predict:{}", line, predict);
        total += predict;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "2");
    }
}
