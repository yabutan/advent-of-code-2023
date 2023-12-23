use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufReader, Read};

use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::multi::many0;
use nom::IResult;
use nom_locate::LocatedSpan;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-21/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input, 26501365)?;
    println!("answer: {}", answer);

    Ok(())
}

fn find(data: &InputData, max: usize) -> i64 {
    let mut tiles: HashMap<_, Int> = HashMap::new();
    let mut visited = HashSet::new();

    let mut marks = vec![data.start];
    for distance in 1..=max {
        marks = next_step(data, &marks, &mut visited, &mut tiles, distance);
    }

    tiles
        .into_iter()
        .filter(|(distance, _)| distance % 2 == max % 2)
        .map(|(_, count)| count as i64)
        .sum()
}

fn quad(y: &[i64], n: i64) -> i64 {
    let a = (y[2] - (2 * y[1]) + y[0]) / 2;
    let b = y[1] - y[0] - a;
    let c = y[0];

    (a * n.pow(2)) + (b * n) + c
}

fn process(input: &str, max: usize) -> anyhow::Result<String> {
    let (_, data) = parse_input(LocatedSpan::new(input)).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let size = data.size.x as usize;
    let edge = size / 2;
    let y = [
        find(&data, edge),
        find(&data, edge + size),
        find(&data, edge + size * 2),
    ];
    let answer = quad(&y, ((max - edge) / size) as i64);

    Ok(format!("{}", answer))
}

fn next_step(
    data: &InputData,
    marks: &[IVec2],
    visited: &mut HashSet<IVec2>,
    tiles: &mut HashMap<usize, Int>,
    distance: usize,
) -> Vec<IVec2> {
    let mut next = Vec::new();

    for pos in marks {
        for p in neighbors_infinity(data, pos) {
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);

            if is_rock(data, &p) {
                continue;
            }

            tiles.entry(distance).and_modify(|v| *v += 1).or_insert(1);
            next.push(p);
        }
    }
    next
}

fn neighbors(data: &InputData, pos: &IVec2) -> Vec<IVec2> {
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .into_iter()
        .map(|d| *pos + d)
        .filter(|p| 0 <= p.x && p.x < data.size.x && 0 <= p.y && p.y < data.size.y)
        .collect()
}

fn neighbors_infinity(_data: &InputData, pos: &IVec2) -> Vec<IVec2> {
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .into_iter()
        .map(|d| *pos + d)
        .collect()
}

fn is_rock(data: &InputData, pos: &IVec2) -> bool {
    let pos = IVec2::new(pos.x.rem_euclid(data.size.x), pos.y.rem_euclid(data.size.y));
    data.rocks.contains(&pos)
}

type Int = i32;

#[derive(Debug)]
struct InputData {
    start: IVec2,
    rocks: HashSet<IVec2>,
    size: IVec2,
}

fn parse_input(input: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, InputData> {
    let size = IVec2::new(
        input.lines().next().unwrap().len() as Int,
        input.lines().count() as Int,
    );

    let mut start_pos = IVec2::ZERO;
    let mut rocks = HashSet::new();

    let mut input_mut = input;
    loop {
        let (input, _) = take_while(|c| c == '.' || c == '\n')(input_mut)?;
        let (input, list): (_, Vec<_>) = many0(alt((tag("#"), tag("S"))))(input)?;
        input_mut = input;
        if list.is_empty() {
            break;
        }

        for s in list {
            let pos = IVec2::new(s.get_column() as Int - 1, s.location_line() as Int - 1);
            match *s.fragment() {
                "S" => {
                    start_pos = pos;
                }
                _ => {
                    rocks.insert(pos);
                }
            }
        }
    }

    Ok((
        input_mut,
        InputData {
            start: start_pos,
            rocks,
            size,
        },
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........
    "#};

    #[test]
    fn test_is_rock() {
        let (_, data) = parse_input(LocatedSpan::new(INPUT)).unwrap();

        assert!(is_rock(&data, &IVec2::new(1, 2)));
        assert!(!is_rock(&data, &IVec2::new(-1, 1)));
        assert!(is_rock(&data, &IVec2::new(-2, 1)));
        assert!(!is_rock(&data, &IVec2::new(-12, 1)));
        assert!(is_rock(&data, &IVec2::new(-13, 1)));
        assert!(is_rock(&data, &IVec2::new(1, -2)));
    }

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(LocatedSpan::new(INPUT)).unwrap();

        assert_eq!(data.start, IVec2::new(5, 5));

        assert!(data.rocks.contains(&IVec2::new(5, 1)));
        assert!(data.rocks.contains(&IVec2::new(6, 1)));
        assert!(data.rocks.contains(&IVec2::new(7, 1)));
        assert!(data.rocks.contains(&IVec2::new(9, 1)));

        assert!(data.rocks.contains(&IVec2::new(1, 9)));
        assert!(data.rocks.contains(&IVec2::new(2, 9)));
        assert!(data.rocks.contains(&IVec2::new(5, 9)));
        assert!(data.rocks.contains(&IVec2::new(6, 9)));
        assert!(data.rocks.contains(&IVec2::new(8, 9)));
        assert!(data.rocks.contains(&IVec2::new(9, 9)));
    }

    #[test]
    fn test_process() {
        // assert_eq!(process(INPUT, 6).unwrap(), "16");
        // assert_eq!(process(INPUT, 10).unwrap(), "50");
        // assert_eq!(process(INPUT, 50).unwrap(), "1594");
        // assert_eq!(process(INPUT, 100).unwrap(), "6536");
        // assert_eq!(process(INPUT, 500).unwrap(), "167004");
        // assert_eq!(process(INPUT, 1000).unwrap(), "668697");
        // assert_eq!(process(INPUT, 5000).unwrap(), "16733044");
    }

    #[test]
    fn test_process2() {
        //assert_eq!(process(INPUT, 500).unwrap(), "167004");
        //assert_eq!(process(INPUT, 1000).unwrap(), "668697");
        // assert_eq!(process(INPUT, 5000).unwrap(), "16733044");
    }
}
