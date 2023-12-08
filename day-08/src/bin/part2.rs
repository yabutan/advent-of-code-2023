use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};
use std::ops::{Div, Mul, Rem};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-08/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn find_step(data: &InputData, begin_place: &Place) -> usize {
    let mut step = 0;
    let mut current = begin_place;
    while !current.ends_with('Z') {
        let nav = &data.instructions[step % data.instructions.len()];
        step += 1;

        let (left, right) = &data.places[current];
        current = match nav {
            Navigation::Left => left,
            Navigation::Right => right,
        };
    }
    step
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let numbers = data
        .places
        .keys()
        .filter(|p| p.ends_with('A'))
        .map(|p| find_step(&data, p))
        .collect::<Vec<_>>();

    let mut v = numbers[0];
    for i in 1..numbers.len() {
        v = lcm(v, numbers[i]);
    }
    println!("lcm: {}", v);

    Ok(v.to_string())
}

type Place<'a> = &'a str;

#[derive(Debug, Copy, Clone)]
enum Navigation {
    Left,
    Right,
}

#[derive(Debug)]
struct InputData<'a> {
    instructions: Vec<Navigation>,
    places: HashMap<Place<'a>, (Place<'a>, Place<'a>)>,
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    fn navs(input: &str) -> IResult<&str, Vec<Navigation>> {
        let nav = alt((
            value(Navigation::Left, complete::char('L')),
            value(Navigation::Right, complete::char('R')),
        ));
        many1(nav)(input)
    }
    fn place(input: &str) -> IResult<&str, Place> {
        complete::alphanumeric1(input)
    }
    fn line(input: &str) -> IResult<&str, (Place, (Place, Place))> {
        let (input, from) = place(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, (left, right)) = separated_pair(place, tag(", "), place)(input)?;
        let (input, _) = tag(")")(input)?;
        Ok((input, (from, (left, right))))
    }

    let (input, instructions) = navs(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, places) = separated_list1(newline, line)(input)?;

    Ok((
        input,
        InputData {
            instructions,
            places: places.into_iter().collect(),
        },
    ))
}

trait Num:
    Copy + Eq + Default + Div<Self, Output = Self> + Mul<Self, Output = Self> + Rem<Self, Output = Self>
{
}
impl Num for usize {}
impl Num for u32 {}

fn gcd<T: Num>(mut a: T, mut b: T) -> T {
    while b != Default::default() {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm<T: Num>(a: T, b: T) -> T {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    "#};

    const INPUT2: &str = indoc! {r#"
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    "#};

    const INPUT3: &str = indoc! {r#"
    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    "#};

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:#?}", data);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT3).unwrap();
        assert_eq!(answer, "6");
    }

    #[test]
    fn test_gdc() {
        let numbers = [8u32, 10, 15];
        let mut v = numbers[0];
        for i in 1..numbers.len() {
            v = lcm(v, numbers[i]);
        }

        assert_eq!(v, 120);
    }
}
