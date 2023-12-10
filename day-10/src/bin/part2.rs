use std::fs;
use std::io::{BufReader, Read};

use day_10::{dump_map, is_outside, parse_input, search_area, search_path};

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

    let mut count_of_tiles = 0;
    for path in &search_path(&data) {
        println!("----------");

        // 進行方向に対して、右側のエリアを取得
        let area = search_area(&data, path);
        dump_map(&data, &area, path);

        // 外側と判断すればカウントしない。
        if is_outside(&data, &area) {
            continue;
        }

        count_of_tiles = area.len();
    }

    Ok(count_of_tiles.to_string())
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

    const INPUT3: &str = indoc! {r#"
    ...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........
    "#};

    const INPUT4: &str = indoc! {r#"
    .F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...
    "#};

    const INPUT5: &str = indoc! {r#"
    FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "1");

        let answer = process(INPUT2).unwrap();
        assert_eq!(answer, "1");

        let answer = process(INPUT3).unwrap();
        assert_eq!(answer, "4");

        let answer = process(INPUT4).unwrap();
        assert_eq!(answer, "8");

        let answer = process(INPUT5).unwrap();
        assert_eq!(answer, "10");
    }
}
