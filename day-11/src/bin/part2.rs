use std::fs;
use std::io::{BufReader, Read};

use day_11::{make_expanded_stars, make_pairs, measure_length, parse_input, Int};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-11/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input, 1000000)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str, empty_size: Int) -> anyhow::Result<String> {
    let data = parse_input(input);
    let stars = make_expanded_stars(&data, empty_size);
    let pairs = make_pairs(stars.len());

    let mut total = 0;
    for (i, j) in pairs {
        let distance = measure_length(&stars[i].pos, &stars[j].pos);
        println!("{} -> {}: {}", stars[i].id, stars[j].id, distance);
        total += distance;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT, 2).unwrap();
        assert_eq!(answer, "374");

        let answer = process(INPUT, 10).unwrap();
        assert_eq!(answer, "1030");

        let answer = process(INPUT, 100).unwrap();
        assert_eq!(answer, "8410");
    }
}
