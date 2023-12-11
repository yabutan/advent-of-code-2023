use std::collections::HashMap;

use glam::I64Vec2;
use itertools::Itertools;

pub type Int = i64;
pub type Vec2 = I64Vec2;

pub fn make_pairs(num: usize) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..num {
        for j in (i + 1)..num {
            pairs.push((i, j));
        }
    }
    pairs
}

#[derive(Debug, Clone)]
pub struct Star {
    pub id: i32,
    pub pos: Vec2,
}

#[derive(Debug)]
pub struct InputData {
    pub stars: Vec<Star>,
}

pub fn parse_input(input: &str) -> InputData {
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

fn make_mutation(
    data: &InputData,
    empty_size: Int,
    get_pos: fn(&Star) -> Int,
) -> HashMap<i32, Int> {
    let group = data.stars.iter().into_group_map_by(|star| get_pos(star));
    let mut migrates = HashMap::new();
    let mut prev = -1;
    for v in group.keys().sorted() {
        let margin = v - prev - 1;

        if margin > 0 {
            let margin = (empty_size * margin) - 1;
            for star in &data.stars {
                if get_pos(star) >= *v {
                    migrates
                        .entry(star.id)
                        .and_modify(|e| *e += margin)
                        .or_insert(margin);
                }
            }
        }

        prev = *v;
    }

    migrates
}

pub fn make_expanded_stars(data: &InputData, empty_size: Int) -> Vec<Star> {
    let y_mut = make_mutation(data, empty_size, |star| star.pos.y);
    let x_mut = make_mutation(data, empty_size, |star| star.pos.x);

    data.stars
        .iter()
        .cloned()
        .map(|mut star| {
            if let Some(margin) = y_mut.get(&star.id) {
                star.pos.y += margin;
            }
            if let Some(margin) = x_mut.get(&star.id) {
                star.pos.x += margin;
            }
            star
        })
        .collect::<Vec<_>>()
}

pub fn measure_length(a: &Vec2, b: &Vec2) -> Int {
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
}
