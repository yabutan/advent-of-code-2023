use std::fs;
use std::io::{BufReader, Read};

use glam::I64Vec3;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete;
use nom::character::complete::{newline, space0, space1};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-05/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = part1(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let mut locations = Vec::new();
    let (_, (seeds, maps)) = parse_data(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;
    for seed in seeds {
        let mut value = seed;
        for map in &maps {
            value = map.convert(value);
        }
        locations.push(value);
    }

    println!("{:?}", locations);

    let min = locations.iter().min().unwrap();
    Ok(min.to_string())
}

fn parse_map_line(input: &str) -> IResult<&str, I64Vec3> {
    let (input, numbers) = separated_list1(space1, complete::i64)(input)?;
    if numbers.len() != 3 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )));
    }
    Ok((input, I64Vec3::from_slice(&numbers)))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, label) = take_while1(|c: char| c.is_alphabetic() || c == '-')(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, list) = separated_list1(newline, parse_map_line)(input)?;
    Ok((input, Map { label, list }))
}

#[derive(Debug)]
struct Map<'a> {
    label: &'a str,
    list: Vec<I64Vec3>,
}

fn parse_data(input: &str) -> IResult<&str, (Vec<i64>, Vec<Map>)> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = space0(input)?;
    let (input, seeds) = separated_list1(space1, complete::i64)(input)?;
    let (input, _) = tuple((newline, newline))(input)?;
    let (input, maps) = separated_list1(tuple((newline, newline)), parse_map)(input)?;

    Ok((input, (seeds, maps)))
}

impl Map<'_> {
    fn convert(&self, src: i64) -> i64 {
        for data in &self.list {
            if let Some(value) = convert(data, src) {
                return value;
            }
        }
        src
    }
}

fn convert(data: &I64Vec3, src: i64) -> Option<i64> {
    if (data.y..(data.y + data.z)).contains(&src) {
        let diff = data.x - data.y;
        return Some(src + diff);
    }
    None
}

#[cfg(test)]
mod tests {
    use glam::i64vec3;
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    "#};

    #[test]
    fn test_parse_map() {
        let (_, data) = parse_data(INPUT).unwrap();
        println!("{:#?}", data);
    }

    #[test]
    fn test_map_convert() {
        let (_, (_, data)) = parse_data(INPUT).unwrap();

        let converter = &data[0];
        assert_eq!(converter.label, "seed-to-soil");
        assert_eq!(converter.convert(0), 0);
        assert_eq!(converter.convert(1), 1);
        assert_eq!(converter.convert(48), 48);
        assert_eq!(converter.convert(49), 49);
        assert_eq!(converter.convert(50), 52);
        assert_eq!(converter.convert(51), 53);
        assert_eq!(converter.convert(96), 98);
        assert_eq!(converter.convert(97), 99);
        assert_eq!(converter.convert(98), 50);
        assert_eq!(converter.convert(99), 51);
    }

    #[test]
    fn test_convert() {
        let c = i64vec3(50, 98, 2);
        assert_eq!(convert(&c, 98), Some(50));
        assert_eq!(convert(&c, 99), Some(51));
        assert_eq!(convert(&c, 10), None);

        let c = i64vec3(52, 50, 48);
        assert_eq!(convert(&c, 53), Some(55));
        assert_eq!(convert(&c, 10), None);
    }

    #[test]
    fn test_par1() {
        let answer = part1(INPUT).unwrap();
        assert_eq!(answer, "35");
    }
}
