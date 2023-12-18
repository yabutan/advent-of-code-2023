use std::fs;
use std::io::{BufReader, Read};

use day_18::{calc_area, make_vertices, parse_input};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-18/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let vertex = make_vertices(&data);
    let count = calc_area(&data, &vertex);

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "62");
    }
}
