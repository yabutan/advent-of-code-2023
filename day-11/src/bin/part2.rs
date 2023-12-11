use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use glam::I64Vec2;
use itertools::Itertools;

type Int = i64;
type Vec2 = I64Vec2;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-11/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input, 1000000)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str, empty_size: Int) -> anyhow::Result<String> {
    let data = parse_input(input);
    let stars = make_expanded_stars(&data, empty_size);
    let pairs = make_pairs(stars.len());

    let mut total = 0;
    for (i, j) in pairs {
        let distance = measure_length(&stars[i].pos, &stars[j].pos);
        println!("{} -> {}: {}", stars[i].id, stars[j].id, distance);
        total += distance;
    }

    Ok(total.to_string())
}

fn make_pairs(num: usize) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..num {
        for j in (i + 1)..num {
            pairs.push((i, j));
        }
    }
    pairs
}

#[derive(Debug, Clone)]
struct Star {
    id: i32,
    pos: Vec2,
}

#[derive(Debug)]
struct InputData {
    stars: Vec<Star>,
}

fn parse_input(input: &str) -> InputData {
    let mut stars = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                stars.push(Star {
                    id: stars.len() as i32 + 1,
                    pos: Vec2::new(x as i64, y as i64),
                });
            };
        }
    }

    InputData { stars }
}

fn find_next_margin_y(current_y: Int, stars: &[Star]) -> Option<Int> {
    stars
        .iter()
        .map(|o| o.pos.y)
        .filter(|y| y > &current_y)
        .min()
}
fn find_next_margin_x(current_x: Int, stars: &[Star]) -> Option<Int> {
    stars
        .iter()
        .map(|o| o.pos.x)
        .filter(|x| x > &current_x)
        .min()
}

fn make_expanded_stars(data: &InputData, empty_size: Int) -> Vec<Star> {
    let y_migrates = {
        let group = data.stars.iter().into_group_map_by(|star| star.pos.y);
        let mut migrates = HashMap::new();
        let mut prev = -1;
        for y in group.keys().sorted() {
            let margin = y - prev - 1;

            if margin > 0 {
                let margin = (empty_size * margin) - 1;
                for star in &data.stars {
                    if star.pos.y >= *y {
                        migrates
                            .entry(star.id)
                            .and_modify(|e| *e += margin)
                            .or_insert(margin);
                    }
                }
            }

            prev = *y;
        }

        migrates
    };

    let x_migrates = {
        let group = data.stars.iter().into_group_map_by(|star| star.pos.x);
        let mut migrates = HashMap::new();
        let mut prev = -1;
        for x in group.keys().sorted() {
            let margin = x - prev - 1;

            if margin > 0 {
                let margin = (empty_size * margin) - 1;
                for star in &data.stars {
                    if star.pos.x >= *x {
                        migrates
                            .entry(star.id)
                            .and_modify(|e| *e += margin)
                            .or_insert(margin);
                    }
                }
            }

            prev = *x;
        }
        migrates
    };

    let mut expanded_stars = data.stars.clone();
    for star in &mut expanded_stars {
        if let Some(margin) = y_migrates.get(&star.id) {
            star.pos.y += margin;
        }
        if let Some(margin) = x_migrates.get(&star.id) {
            star.pos.x += margin;
        }
    }

    expanded_stars
}

fn measure_length(a: &Vec2, b: &Vec2) -> Int {
    let dx = if a.x < b.x { b.x - a.x } else { a.x - b.x };
    let dy = if a.y < b.y { b.y - a.y } else { a.y - b.y };
    dx + dy
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    "#};

    #[test]
    fn test_parse_input() {
        let data = parse_input(INPUT);
        println!("{:#?}", data);
    }

    #[test]
    fn test_make_expanded_stars() {
        let data = parse_input(INPUT);
        let expanded = make_expanded_stars(&data, 2);
        println!("{:#?}", expanded);

        assert_eq!(expanded.len(), 9);
        assert_eq!(expanded[0].pos, Vec2::new(4, 0));
        assert_eq!(expanded[1].pos, Vec2::new(9, 1));
        assert_eq!(expanded[2].pos, Vec2::new(0, 2));
        assert_eq!(expanded[3].pos, Vec2::new(8, 5));
        assert_eq!(expanded[4].pos, Vec2::new(1, 6));
        assert_eq!(expanded[5].pos, Vec2::new(12, 7));
        assert_eq!(expanded[6].pos, Vec2::new(9, 10));
        assert_eq!(expanded[7].pos, Vec2::new(0, 11));
        assert_eq!(expanded[8].pos, Vec2::new(5, 11));
    }

    #[test]
    fn test_make_pair() {
        let pairs = make_pairs(9);
        assert_eq!(pairs.len(), 36);
    }

    #[test]
    fn test_measure_length() {
        assert_eq!(measure_length(&Vec2::new(0, 0), &Vec2::new(3, 1)), 4);
        assert_eq!(measure_length(&Vec2::new(1, 6), &Vec2::new(5, 11)), 9);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT, 2).unwrap();
        assert_eq!(answer, "374");

        let answer = process(INPUT, 10).unwrap();
        assert_eq!(answer, "1030");

        let answer = process(INPUT, 100).unwrap();
        assert_eq!(answer, "8410");
    }
}
