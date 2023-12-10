use std::fs;
use std::io::{BufReader, Read};

use day_10::{parse_input, search_path};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-10/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let data = parse_input(input)?;

    let paths = search_path(&data);
    for p in &paths {
        println!("path.len: {:?}", p.len());
    }

    // 一番長くでるーぷしている物のステップ数を取得する。
    let far = paths.iter().map(|p| p.len()).max().expect("no max") / 2;
    Ok(far.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    .....
    .S-7.
    .|.|.
    .L-J.
    .....
    "#};

    const INPUT2: &str = indoc! {r#"
    ..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "4");

        let answer = process(INPUT2).unwrap();
        assert_eq!(answer, "8");
    }
}
