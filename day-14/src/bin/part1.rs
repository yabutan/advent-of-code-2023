use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use glam::IVec2;
use itertools::Itertools;
use nom::bytes::complete::is_a;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-14/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let mut rocks = get_rocks(&data);
    for x in 0..data.size.x {
        let mut columns = get_vertical_line(&mut rocks, x);
        move_to_north(&mut columns);
    }

    let total = rocks
        .iter()
        .filter(|o| o.shape == RockShape::Round)
        .map(|o| data.size.y - o.pos.y)
        .sum::<i32>();

    Ok(total.to_string())
}

#[derive(Debug, Eq, PartialEq)]
enum RockShape {
    Round,
    Cube,
}

#[derive(Debug)]
struct Rock {
    id: u32,
    pos: IVec2,
    shape: RockShape,
}

fn get_rocks(data: &InputData) -> Vec<Rock> {
    let mut rocks = Vec::new();
    for (y, line) in data.platform.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            let id = rocks.len() as u32 + 1;
            let pos = IVec2::new(x as i32, y as i32);
            let shape = match c {
                'O' => RockShape::Round,
                '#' => RockShape::Cube,
                _ => unreachable!(),
            };

            rocks.push(Rock { id, pos, shape });
        }
    }

    rocks
}

fn get_vertical_line(rocks: &mut [Rock], x: i32) -> Vec<&mut Rock> {
    rocks
        .iter_mut()
        .filter(|o| o.pos.x == x)
        .sorted_by_key(|o| o.pos.y)
        .collect()
}

fn get_horizontal_line(rocks: &mut [Rock], y: i32) -> Vec<&mut Rock> {
    rocks
        .iter_mut()
        .filter(|o| o.pos.y == y)
        .sorted_by_key(|o| o.pos.x)
        .collect()
}

fn move_to_north(vertical_line: &mut [&mut Rock]) {
    let mut top = 0;
    for rock in vertical_line {
        match rock.shape {
            RockShape::Round => {
                if top <= rock.pos.y {
                    rock.pos.y = top;
                    top += 1;
                }
            }
            RockShape::Cube => {
                top = rock.pos.y + 1;
            }
        }
    }
}

fn dump_rocks(data: &InputData, rocks: &[Rock]) -> Vec<String> {
    let rocks = rocks
        .iter()
        .map(|o| {
            let c = match o.shape {
                RockShape::Round => 'O',
                RockShape::Cube => '#',
            };

            (o.pos, c)
        })
        .collect::<HashMap<IVec2, char>>();

    let mut lines = Vec::new();
    for y in 0..data.size.y {
        let mut line = String::new();
        for x in 0..data.size.x {
            let c = rocks.get(&IVec2::new(x, y)).unwrap_or(&'.');
            line.push(*c);
        }
        lines.push(line);
    }

    lines
}

#[derive(Debug)]
struct InputData<'a> {
    platform: Vec<&'a str>,
    size: IVec2,
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, lines) = separated_list1(newline, is_a(".O#"))(input)?;
    let size = IVec2::new(lines[0].len() as i32, lines.len() as i32);

    Ok((
        input,
        InputData {
            platform: lines,
            size,
        },
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    "#};

    #[test]
    fn test_parse_input() {
        let expect = indoc! {r#"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
        "#};

        let (_, data) = parse_input(INPUT).unwrap();

        let mut rocks = get_rocks(&data);
        for x in 0..data.size.x {
            let mut columns = get_vertical_line(&mut rocks, x);
            move_to_north(&mut columns);
        }

        let total = rocks
            .iter()
            .filter(|o| o.shape == RockShape::Round)
            .map(|o| data.size.y - o.pos.y)
            .sum::<i32>();

        let output = dump_rocks(&data, &rocks);

        assert_eq!(total, 136);
        assert_eq!(output.join("\n") + "\n", expect);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "136");
    }
}
