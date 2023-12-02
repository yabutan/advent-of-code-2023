use std::collections::HashMap;

use nom::character::complete::space1;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::map,
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cube {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Eq, PartialEq)]
pub struct GameInfo {
    pub number: u32,
    pub sets: Vec<Vec<(u32, Cube)>>,
}

pub fn parse_game(input: &str) -> IResult<&str, GameInfo> {
    let (input, number) = parse_game_header(input)?;
    let (input, _) = space1(input)?;
    let (input, sets) = separated_list1(tuple((tag(";"), space0)), parse_cube_set)(input)?;

    Ok((input, GameInfo { number, sets }))
}

pub fn game_possible(game_info: &GameInfo, max_map: &HashMap<Cube, u32>) -> bool {
    for set in &game_info.sets {
        for (num_of_cubes, cube_color) in set {
            let contains = max_map.get(cube_color).unwrap();
            if num_of_cubes > contains {
                return false;
            }
        }
    }
    true
}

fn parse_cube(input: &str) -> IResult<&str, (u32, Cube)> {
    let (input, num) = map(digit1, |s: &str| s.parse().unwrap())(input)?;

    let (input, _) = space1(input)?;

    let (input, cube) = alt((
        value(Cube::Red, tag("red")),
        value(Cube::Green, tag("green")),
        value(Cube::Blue, tag("blue")),
    ))(input)?;

    Ok((input, (num, cube)))
}

fn parse_cube_set(input: &str) -> IResult<&str, Vec<(u32, Cube)>> {
    separated_list1(tuple((tag(","), space0)), parse_cube)(input)
}

fn parse_game_header(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Game")(input)?;
    let (input, _) = space1(input)?;
    let (input, number) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;
    let (input, _) = tag(":")(input)?;
    Ok((input, number))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_parse_cube() {
        assert_eq!(parse_cube("6 red").unwrap(), ("", (6, Cube::Red)));
        assert_eq!(parse_cube("2 green").unwrap(), ("", (2, Cube::Green)));
        assert_eq!(parse_cube("3 blue").unwrap(), ("", (3, Cube::Blue)));
    }

    #[test]
    fn test_parse_cute_set() {
        assert_eq!(
            parse_cube_set("3 blue, 4 red").unwrap(),
            ("", vec![(3, Cube::Blue), (4, Cube::Red)])
        );
        assert_eq!(
            parse_cube_set("1 red, 2 green, 6 blue").unwrap(),
            ("", vec![(1, Cube::Red), (2, Cube::Green), (6, Cube::Blue),])
        );
        assert_eq!(
            parse_cube_set("2 green").unwrap(),
            ("", vec![(2, Cube::Green),])
        );
    }

    #[test]
    fn test_parse_game() {
        let input = indoc! { r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#};

        let lines: Vec<_> = input.lines().collect();

        assert_eq!(
            parse_game(lines[0]).unwrap(),
            (
                "",
                GameInfo {
                    number: 1,
                    sets: vec![
                        vec![(3, Cube::Blue), (4, Cube::Red)],
                        vec![(1, Cube::Red), (2, Cube::Green), (6, Cube::Blue)],
                        vec![(2, Cube::Green)],
                    ],
                }
            )
        );

        assert_eq!(
            parse_game(lines[1]).unwrap(),
            (
                "",
                GameInfo {
                    number: 2,
                    sets: vec![
                        vec![(1, Cube::Blue), (2, Cube::Green)],
                        vec![(3, Cube::Green), (4, Cube::Blue), (1, Cube::Red)],
                        vec![(1, Cube::Green), (1, Cube::Blue)],
                    ],
                }
            )
        );
    }
}
