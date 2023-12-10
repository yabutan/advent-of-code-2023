use std::collections::HashSet;
use std::ops::Range;

use glam::IVec2;
use nom::InputIter;

pub type Path = Vec<IVec2>;
pub type Area = HashSet<IVec2>;

pub fn search_path(data: &InputData) -> Vec<Path> {
    fn s(data: &InputData, results: &mut Vec<Path>, path: &Path) {
        let current = path.last().expect("no last");

        let prev = if path.len() >= 2 {
            path.get(path.len() - 2)
        } else {
            None
        };

        for (next_s, next) in data.get_pos_connected(current) {
            if let Some(prev) = prev {
                if next == *prev {
                    // すでに通ってきた道は通らない
                    continue;
                }
            }

            let mut new_path = path.clone();
            new_path.push(next);

            if next_s == "S" {
                results.push(new_path);
            } else {
                s(data, results, &new_path);
            }
        }
    }

    let mut path_results = Vec::new();
    s(data, &mut path_results, &vec![data.start]);
    path_results
}

pub fn search_area(data: &InputData, path: &Path) -> HashSet<IVec2> {
    let mut area = HashSet::new();
    let path_set = path.iter().cloned().collect::<HashSet<_>>();
    for w in path.windows(2) {
        let from = &w[0];
        let to = &w[1];

        let s = data.get_s(to).expect("Illegal position");
        let right_side_positions = match s {
            "-" => {
                if from.x < to.x {
                    vec![*to + IVec2::new(0, 1)]
                } else {
                    vec![*to + IVec2::new(0, -1)]
                }
            }
            "|" => {
                if from.y < to.y {
                    vec![*to + IVec2::new(-1, 0)]
                } else {
                    vec![*to + IVec2::new(1, 0)]
                }
            }
            "L" => {
                if from.y < to.y {
                    vec![*to + IVec2::new(-1, 0), *to + IVec2::new(0, 1)]
                } else {
                    continue;
                }
            }
            "J" => {
                if from.x < to.x {
                    vec![*to + IVec2::new(1, 0), *to + IVec2::new(0, 1)]
                } else {
                    continue;
                }
            }
            "7" => {
                if from.y > to.y {
                    vec![*to + IVec2::new(1, 0), *to + IVec2::new(0, -1)]
                } else {
                    continue;
                }
            }
            "F" => {
                if from.x > to.x {
                    vec![*to + IVec2::new(-1, 0), *to + IVec2::new(0, -1)]
                } else {
                    continue;
                }
            }
            _ => continue,
        };

        for right_side_pos in &right_side_positions {
            if !data.in_range(right_side_pos) {
                continue;
            };
            if path_set.contains(right_side_pos) {
                continue;
            }

            fill_area(data, &mut area, right_side_pos, &path_set);
        }
    }

    area
}

fn fill_area(data: &InputData, area: &mut HashSet<IVec2>, pos: &IVec2, path_set: &HashSet<IVec2>) {
    if !area.insert(*pos) {
        return;
    }

    for d in [
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
        IVec2::new(0, -1),
        IVec2::new(0, 1),
    ] {
        let next = *pos + d;
        if !data.in_range(&next) {
            continue;
        };

        if path_set.contains(&next) {
            continue;
        }

        fill_area(data, area, &next, path_set);
    }
}

/// 外側と隣接している場合は外側
pub fn is_outside(data: &InputData, area: &Area) -> bool {
    area.iter().any(|pos| {
        data.x_range.start == pos.x
            || data.x_range.end - 1 == pos.x
            || data.y_range.start == pos.y
            || data.y_range.end - 1 == pos.y
    })
}

pub fn dump_map(data: &InputData, area: &Area, path: &Path) {
    let path_set = path.iter().cloned().collect::<HashSet<_>>();

    for y in data.y_range.clone() {
        for x in data.x_range.clone() {
            let pos = IVec2::new(x, y);
            if area.contains(&pos) {
                print!("#");
            } else if path_set.contains(&pos) {
                print!("{}", data.get_s(&pos).expect("Illegal position"));
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[derive(Debug)]
pub struct InputData<'a> {
    pub grid: Vec<&'a str>,
    pub start: IVec2,

    pub x_range: Range<i32>,
    pub y_range: Range<i32>,
}

pub fn parse_input(input: &str) -> anyhow::Result<InputData> {
    let grid = input.lines().collect::<Vec<_>>();

    let mut start = None;
    for (y, line) in grid.iter().enumerate() {
        if let Some(x) = line.position(|c| c == 'S') {
            start = Some(IVec2::new(x as i32, y as i32));
            break;
        }
    }

    Ok(InputData::new(grid, start.expect("no start")))
}

impl InputData<'_> {
    fn new(grid: Vec<&str>, start: IVec2) -> InputData {
        let x_range = 0..grid[0].len() as i32;
        let y_range = 0..grid.len() as i32;

        InputData {
            grid,
            start,
            x_range,
            y_range,
        }
    }

    pub fn get_s(&self, pos: &IVec2) -> Option<&str> {
        if !self.in_range(pos) {
            return None;
        }

        let line = self.grid[pos.y as usize];
        let s = &line[(pos.x as usize)..pos.x as usize + 1];

        Some(s)
    }

    pub fn in_range(&self, pos: &IVec2) -> bool {
        self.x_range.contains(&pos.x) && self.y_range.contains(&pos.y)
    }

    /// | is a vertical pipe connecting north and south.
    /// - is a horizontal pipe connecting east and west.
    /// L is a 90-degree bend connecting north and east.
    /// J is a 90-degree bend connecting north and west.
    /// 7 is a 90-degree bend connecting south and west.
    /// F is a 90-degree bend connecting south and east.
    /// . is ground; there is no pipe in this tile.
    /// S is the starting position of the animal; there is a pipe on this
    fn get_pos_as_possible(&self, pos: &IVec2) -> Vec<IVec2> {
        let Some(s) = self.get_s(pos) else {
            return vec![];
        };

        let pos_list = match s {
            "S" => vec![
                *pos + IVec2::new(0, -1),
                *pos + IVec2::new(0, 1),
                *pos + IVec2::new(-1, 0),
                *pos + IVec2::new(1, 0),
            ],
            "|" => vec![*pos + IVec2::new(0, -1), *pos + IVec2::new(0, 1)],
            "-" => vec![*pos + IVec2::new(-1, 0), *pos + IVec2::new(1, 0)],
            "L" => vec![*pos + IVec2::new(0, -1), *pos + IVec2::new(1, 0)],
            "J" => vec![*pos + IVec2::new(0, -1), *pos + IVec2::new(-1, 0)],
            "7" => vec![*pos + IVec2::new(0, 1), *pos + IVec2::new(-1, 0)],
            "F" => vec![*pos + IVec2::new(0, 1), *pos + IVec2::new(1, 0)],
            "." => vec![],
            _ => panic!("unknown sign: {}", s),
        };

        pos_list
            .into_iter()
            .filter(|pos| self.in_range(pos))
            .collect()
    }

    pub fn get_pos_connected(&self, pos: &IVec2) -> Vec<(&str, IVec2)> {
        let mut connected = Vec::new();

        let possibles = self.get_pos_as_possible(pos);
        for possible in possibles {
            if self.get_pos_as_possible(&possible).iter().any(|p| p == pos) {
                let s = self.get_s(&possible).expect("Illegal position");
                connected.push((s, possible));
            }
        }

        connected
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    .....
    .S-7.
    .|.|.
    .L-J.
    .....
    "#};

    const INPUT2: &str = indoc! {r#"
    ..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...
    "#};

    #[test]
    fn test_parse_input() {
        let data = parse_input(INPUT).unwrap();
        for (y, line) in data.grid.iter().enumerate() {
            println!("{}: {}", y, line);
        }
        println!("start:{:?}", data.start);

        println!("------------");

        let data = parse_input(INPUT2).unwrap();
        for (y, line) in data.grid.iter().enumerate() {
            println!("{}: {}", y, line);
        }
        println!("start:{:?}", data.start);
    }

    #[test]
    fn test_get_pos_as_possible() {
        let data = parse_input(INPUT).unwrap();
        for (y, line) in data.grid.iter().enumerate() {
            println!("{}: {}", y, line);
        }
        println!("start:{:?}", data.start);

        let connected = data.get_pos_connected(&IVec2::new(1, 1));
        #[rustfmt::skip] assert_eq!(connected, vec![
            ("|", IVec2::new(1, 2)), 
            ("-", IVec2::new(2, 1)),
        ]);

        let connected = data.get_pos_connected(&IVec2::new(2, 1));
        #[rustfmt::skip] assert_eq!(connected, vec![
            ("S", IVec2::new(1, 1)),
            ("7", IVec2::new(3, 1)),
        ]);

        let connected = data.get_pos_connected(&IVec2::new(3, 1));
        #[rustfmt::skip] assert_eq!(connected, vec![
            ("|", IVec2::new(3, 2)),
            ("-", IVec2::new(2, 1)),
        ]);
    }
}
