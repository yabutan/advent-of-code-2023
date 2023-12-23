use std::collections::HashSet;
use std::fs;
use std::io::{BufReader, Read};

use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::IResult;
use nom::multi::many0;
use nom_locate::LocatedSpan;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-21/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input, 64)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str, max: usize) -> anyhow::Result<String> {
    let (_, data) = parse_input(LocatedSpan::new(input)).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let mut state = StepState {
        marks: HashSet::from([data.start]),
    };
    for _ in 0..max {
        state = next_step(&data, &state);
    }

    println!("{:?}", state.marks);
    Ok(format!("{}", state.marks.len()))
}

fn neighbors(data: &InputData, pos: &IVec2) -> Vec<IVec2> {
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .into_iter()
        .map(|d| *pos + d)
        .filter(|p| 0 <= p.x && p.x < data.size.x && 0 <= p.y && p.y < data.size.y)
        .collect()
}

fn next_step(data: &InputData, state: &StepState) -> StepState {
    let mut marks = HashSet::new();
    for pos in &state.marks {
        for p in neighbors(data, pos) {
            if data.rocks.contains(&p) {
                continue;
            }
            marks.insert(p);
        }
    }
    StepState { marks }
}

type Int = i32;

#[derive(Debug)]
struct InputData {
    start: IVec2,
    rocks: HashSet<IVec2>,
    size: IVec2,
}

struct StepState {
    marks: HashSet<IVec2>,
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
        let answer = process(INPUT, 6).unwrap();
        assert_eq!(answer, "16");
    }
}
