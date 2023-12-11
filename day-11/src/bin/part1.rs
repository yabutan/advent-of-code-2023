use std::fs;
use std::io::{BufReader, Read};

use glam::IVec2;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-11/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let data = parse_input(input);
    let stars = make_expanded_stars(&data);
    let pairs = make_pairs(stars.len());

    let mut total = 0;
    for (i, j) in pairs {
        let distance = masure_lenght(&stars[i].pos, &stars[j].pos);
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
    pos: IVec2,
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
                    pos: IVec2::new(x as i32, y as i32),
                });
            };
        }
    }

    InputData { stars }
}

fn find_next_margin_y(current_y: i32, stars: &[Star]) -> Option<i32> {
    stars
        .iter()
        .map(|o| o.pos.y)
        .filter(|y| y > &current_y)
        .min()
}
fn find_next_margin_x(current_x: i32, stars: &[Star]) -> Option<i32> {
    stars
        .iter()
        .map(|o| o.pos.x)
        .filter(|x| x > &current_x)
        .min()
}

fn make_expanded_stars(data: &InputData) -> Vec<Star> {
    let mut expanded_stars = data.stars.clone();

    let mut prev_y = -1;
    while let Some(next_y) = find_next_margin_y(prev_y, &expanded_stars) {
        println!("prev_y: {}, next_y: {}", prev_y, next_y);

        let margin = next_y - prev_y - 1;
        prev_y = next_y;
        if margin == 0 {
            continue;
        }

        // 元座標にマージン分を加算する。
        for star in &mut expanded_stars {
            if star.pos.y >= next_y {
                star.pos.y += margin;
            }
        }
    }

    let mut prev_x = -1;
    while let Some(next_x) = find_next_margin_x(prev_x, &expanded_stars) {
        println!("prev_x: {}, next_x: {}", prev_x, next_x);

        let margin = next_x - prev_x - 1;
        prev_x = next_x;
        if margin == 0 {
            continue;
        }

        // 元座標にマージン分を加算する。
        for star in &mut expanded_stars {
            if star.pos.x >= next_x {
                star.pos.x += margin;
            }
        }
    }

    expanded_stars
}

fn distance(a: &IVec2, b: &IVec2) -> i32 {
    a.distance_squared(*b)
}

fn masure_lenght(a: &IVec2, b: &IVec2) -> i32 {
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
        let expanded = make_expanded_stars(&data);
        println!("{:#?}", expanded);

        assert_eq!(expanded.len(), 9);
        assert_eq!(expanded[0].pos, IVec2::new(4, 0));
        assert_eq!(expanded[1].pos, IVec2::new(9, 1));
        assert_eq!(expanded[2].pos, IVec2::new(0, 2));
        assert_eq!(expanded[3].pos, IVec2::new(8, 5));
        assert_eq!(expanded[4].pos, IVec2::new(1, 6));
        assert_eq!(expanded[5].pos, IVec2::new(12, 7));
        assert_eq!(expanded[6].pos, IVec2::new(9, 10));
        assert_eq!(expanded[7].pos, IVec2::new(0, 11));
        assert_eq!(expanded[8].pos, IVec2::new(5, 11));
    }

    #[test]
    fn test_make_pair() {
        let pairs = make_pairs(9);
        assert_eq!(pairs.len(), 36);
    }

    #[test]
    fn test_distance() {
        let ret = distance(&IVec2::new(0, 0), &IVec2::new(3, 1));
        println!("ret: {}", ret);

        let ret = distance(&IVec2::new(0, 0), &IVec2::new(-3, 1));
        println!("ret: {}", ret);

        let ret = distance(&IVec2::new(0, 0), &IVec2::new(-3, 2));
        println!("ret: {}", ret);

        let ret = distance(&IVec2::new(0, 0), &IVec2::new(0, 0));
        println!("ret: {}", ret);

        let ret = distance(&IVec2::new(0, 0), &IVec2::new(0, 1));
        println!("ret: {}", ret);
    }

    #[test]
    fn test_measure_length() {
        assert_eq!(masure_lenght(&IVec2::new(0, 0), &IVec2::new(3, 1)), 4);
        assert_eq!(masure_lenght(&IVec2::new(1, 6), &IVec2::new(5, 11)), 9);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "374");
    }
}
