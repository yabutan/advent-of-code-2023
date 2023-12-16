use std::fs;
use std::io::{BufReader, Read};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-15/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let sum = input.trim().split(',').map(to_hash).sum::<Int>();

    Ok(sum.to_string())
}

type Int = i32;

fn to_hash(input: &str) -> Int {
    let mut value = 0;
    for c in input.chars() {
        value += c as Int;
        value *= 17;
        value %= 256;
    }

    value
}
#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "#};

    #[test]
    fn test_hash() {
        let input = "HASH";
        assert_eq!(to_hash(input), 52);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "1320");
    }
}
