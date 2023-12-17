use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::{BufReader, Read};

use glam::{ivec2, IVec2, Vec2Swizzles};
use itertools::Itertools;
use nom::bytes::complete::is_a;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::IResult;

type Int = i32;

#[derive(Debug)]
struct InputData<'a> {
    symbols: BTreeMap<(Int, Int), Symbol<'a>>,
    size: IVec2,
}

#[derive(Debug)]
enum Symbol<'a> {
    Mirror(&'a str),
    Splitter(&'a str),
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct FloorState {
    // (x, y, dx, dy)
    paths: HashSet<(Int, Int, Int, Int)>,
}

impl FloorState {
    fn new() -> Self {
        Self {
            paths: HashSet::new(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-16/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let mut start_list = Vec::new();
    for x in 0..data.size.x {
        start_list.push((ivec2(x, -1), ivec2(0, 1)));
        start_list.push((ivec2(x, data.size.y), ivec2(0, -1)));
    }
    for y in 0..data.size.y {
        start_list.push((ivec2(-1, y), ivec2(1, 0)));
        start_list.push((ivec2(data.size.x, y), ivec2(-1, 0)));
    }

    let best_energized = start_list
        .iter()
        .map(|(pos, dir)| {
            let mut state = FloorState::new();
            proceed_beam(&data, &mut state, *pos, *dir);

            let energized = state
                .paths
                .iter()
                .map(|(x, y, _, _)| (x, y))
                .unique()
                .count();

            energized
        })
        .max()
        .expect("no option");

    Ok(best_energized.to_string())
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, lines): (_, Vec<&str>) = separated_list1(newline, is_a("|-/\\."))(input)?;

    let size = IVec2::new(lines[0].len() as Int, lines.len() as Int);

    let mut symbols = BTreeMap::new();
    for (y, line) in lines.iter().enumerate() {
        for x in 0..line.len() {
            let s = &line[x..x + 1];
            let symbol = match s {
                "." => continue,
                "|" | "-" => Symbol::Splitter(s),
                "/" | "\\" => Symbol::Mirror(s),
                _ => unreachable!("invalid symbol: {}", s),
            };
            symbols.insert((x as Int, y as Int), symbol);
        }
    }

    Ok((input, InputData { symbols, size }))
}

fn proceed_beam(data: &InputData, state: &mut FloorState, mut pos: IVec2, direction: IVec2) {
    loop {
        pos += direction;

        if pos.x < 0 || pos.x >= data.size.x || pos.y < 0 || pos.y >= data.size.y {
            // はみ出したら終了
            return;
        }

        // pathsに記録
        if !state.paths.insert((pos.x, pos.y, direction.x, direction.y)) {
            // すでに通ったことがあるなら、これ以上探索しない。
            return;
        }

        let Some(symbol) = data.symbols.get(&(pos.x, pos.y)) else {
            // 何もなければそのまま直進
            continue;
        };

        match symbol {
            Symbol::Mirror("/") => {
                proceed_beam(data, state, pos, -direction.yx());
                return;
            }
            Symbol::Mirror("\\") => {
                proceed_beam(data, state, pos, direction.yx());
                return;
            }
            Symbol::Splitter("|") if direction.x != 0 => {
                proceed_beam(data, state, pos, IVec2::new(0, -1));
                proceed_beam(data, state, pos, IVec2::new(0, 1));
                return;
            }
            Symbol::Splitter("-") if direction.y != 0 => {
                proceed_beam(data, state, pos, IVec2::new(-1, 0));
                proceed_beam(data, state, pos, IVec2::new(1, 0));
                return;
            }
            _ => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::ivec2;
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
    "#};

    #[test]
    fn test_direction_mirror() {
        let right = ivec2(1, 0);
        let up = ivec2(0, -1);
        let down = ivec2(0, 1);
        let left = ivec2(-1, 0);

        // '/'
        assert_eq!(-right.yx(), up);
        assert_eq!(-up.yx(), right);
        assert_eq!(-down.yx(), left);
        assert_eq!(-left.yx(), down);

        // '\\'
        assert_eq!(right.yx(), down);
        assert_eq!(up.yx(), left);
        assert_eq!(down.yx(), right);
        assert_eq!(left.yx(), up);
    }

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();

        println!("{:?}", data);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "51");
    }
}
