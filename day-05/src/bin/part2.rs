use std::cmp::{max, min};
use std::fs;
use std::io::{BufReader, Read};

use crate::ConvertResult::{Converted, Through};
use glam::{I64Vec2, I64Vec3};
use itertools::Itertools;
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

    let answer = part2(&input)?;
    println!("answer: {}", answer);
    Ok(())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let (_, (seeds, maps)) = parse_data(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let mut values = seeds.chunks(2).map(I64Vec2::from_slice).collect::<Vec<_>>();

    for map in &maps {
        println!("{}", map.label);
        values = values
            .into_iter()
            .flat_map(|v| map.convert_range(v))
            .collect::<Vec<_>>();
    }

    let locations = values.iter().map(|data| data.x).collect_vec();
    println!("{:?}", locations);

    let min = locations.iter().min().expect("min");
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
    fn convert_range(&self, src: I64Vec2) -> Vec<I64Vec2> {
        let mut results = Vec::new();
        self.convert_range_loop(&mut results, 0, src);
        results
    }
    fn convert_range_loop(&self, result_store: &mut Vec<I64Vec2>, i: usize, src: I64Vec2) {
        if i >= self.list.len() {
            result_store.push(src);
            return;
        }

        let c = &self.list[i];
        let results = convert_range(c, src);

        for res in results {
            match res {
                Converted(v) => {
                    result_store.push(v);
                }
                Through(v) => {
                    self.convert_range_loop(result_store, i + 1, v);
                }
            }
        }
    }
}

fn convert(data: &I64Vec3, src: i64) -> Option<i64> {
    if (data.y..(data.y + data.z)).contains(&src) {
        let diff = data.x - data.y;
        return Some(src + diff);
    }
    None
}

#[derive(Debug, Eq, PartialEq)]
enum ConvertResult<T> {
    Converted(T),
    Through(T),
}

fn convert_range(data: &I64Vec3, src: I64Vec2) -> Vec<ConvertResult<I64Vec2>> {
    let end = src.x + src.y - 1; // length ゼロは考慮しない。
    let data_end = data.y + data.z - 1; // length ゼロは考慮しない。

    if end < data.y {
        return vec![ConvertResult::Through(src)];
    }
    if data_end < src.x {
        return vec![ConvertResult::Through(src)];
    }
    if data.y <= src.x && end <= data_end {
        let new_x = convert(data, src.x).expect("converted");
        return vec![ConvertResult::Converted(I64Vec2::new(new_x, src.y))];
    }

    let mut results = Vec::new();
    if src.x < data.y {
        let new_x = src.x;
        let new_y = data.y - src.x;
        results.push(ConvertResult::Through(I64Vec2::new(new_x, new_y)));
    }

    {
        let new_x = max(src.x, data.y);
        let new_end = min(end, data_end);
        results.push(ConvertResult::Converted(I64Vec2::new(
            convert(data, new_x).expect("converted"),
            new_end - new_x + 1,
        )));
    }

    if data_end < end {
        let new_x = data_end + 1;
        let new_y = end - data_end;
        results.push(ConvertResult::Through(I64Vec2::new(new_x, new_y)));
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::ConvertResult::{Converted, Through};
    use glam::{i64vec2, i64vec3};
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
    fn test_convert_range() {
        let c = i64vec3(200, 100, 10);

        assert_eq!(
            convert_range(&c, i64vec2(99, 1)),
            vec![Through(i64vec2(99, 1))]
        );

        assert_eq!(
            convert_range(&c, i64vec2(99, 5)),
            vec![Through(i64vec2(99, 1)), Converted(i64vec2(200, 4)),]
        );

        assert_eq!(
            convert_range(&c, i64vec2(105, 3)),
            vec![Converted(i64vec2(205, 3)),]
        );

        assert_eq!(
            convert_range(&c, i64vec2(105, 10)),
            vec![Converted(i64vec2(205, 5)), Through(i64vec2(110, 5)),]
        );

        assert_eq!(
            convert_range(&c, i64vec2(110, 10)),
            vec![Through(i64vec2(110, 10))]
        );

        assert_eq!(
            convert_range(&c, i64vec2(95, 20)),
            vec![
                Through(i64vec2(95, 5)),
                Converted(i64vec2(200, 10)),
                Through(i64vec2(110, 5)),
            ]
        );
    }

    #[test]
    fn test_part2() {
        let answer = part2(INPUT).unwrap();
        assert_eq!(answer, "46");
    }
}
