use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use day_12::{arrangements, parse_input};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-12/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("parse_input: {:?}", e))?;

    let mut total = 0;
    let mut memo = HashMap::new();
    for (i, criterion) in data.criteria.iter().enumerate() {
        print!("({}/{}) {:?}", i, data.criteria.len(), criterion);
        let len = arrangements(criterion, &mut memo);
        println!(" -> {}", len);

        total += len;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "21");
    }
}
