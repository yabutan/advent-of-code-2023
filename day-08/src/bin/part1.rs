use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-08/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let mut step = 0;
    let mut current = Place("AAA");

    while current != Place("ZZZ") {
        let (left, right) = &data.places[&current];
        let nav = &data.instructions[step % data.instructions.len()];
        step += 1;
        println!("step: {}, current: {:?}, nav: {:?}", step, current, nav);

        current = match nav {
            Navigation::Left => left.clone(),
            Navigation::Right => right.clone(),
        };
    }

    Ok(step.to_string())
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Place<'a>(&'a str);

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

fn parse_instructions(input: &str) -> IResult<&str, Vec<Navigation>> {
    let nav = alt((
        value(Navigation::Left, complete::char('L')),
        value(Navigation::Right, complete::char('R')),
    ));

    many1(nav)(input)
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    fn parse_place(input: &str) -> IResult<&str, Place> {
        let (input, s) = complete::alpha1(input)?;
        Ok((input, Place(s)))
    }
    fn parse_line(input: &str) -> IResult<&str, (Place, (Place, Place))> {
        let (input, ret) = parse_place(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, left) = parse_place(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, right) = parse_place(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((input, (ret, (left, right))))
    }
    fn parse_places(input: &str) -> IResult<&str, HashMap<Place, (Place, Place)>> {
        let (input, maps) = separated_list1(newline, parse_line)(input)?;
        Ok((input, maps.into_iter().collect()))
    }

    let (input, instructions) = parse_instructions(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, places) = parse_places(input)?;

    Ok((
        input,
        InputData {
            instructions,
            places,
        },
    ))
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

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:#?}", data);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "2");

        let answer = process(INPUT2).unwrap();
        assert_eq!(answer, "6");
    }
}
