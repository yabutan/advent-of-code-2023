use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::io::{BufReader, Read};

use glam::{IVec2, Vec2Swizzles};
use itertools::Itertools;

type Int = i32;

#[derive(Debug)]
struct InputData<'a> {
    lines: Vec<&'a str>,
    start_pos: IVec2,
    end_pos: IVec2,
    size: IVec2,
}

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-23/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let data = parse_input(input);
    let ret = search(&data).expect("no answer found");
    Ok(format!("{}", ret))
}

fn parse_input(input: &str) -> InputData {
    let lines = input.lines().collect::<Vec<_>>();

    let size = IVec2::new(
        lines
            .first()
            .map(|line| line.len())
            .expect("no lines found") as Int,
        lines.len() as Int,
    );

    let start_pos = IVec2::new(
        lines
            .first()
            .and_then(|line| line.chars().position(|c| c == '.'))
            .expect("no start position found") as Int,
        0,
    );

    let end_pos = IVec2::new(
        lines
            .last()
            .and_then(|line| line.chars().position(|c| c == '.'))
            .expect("no end position found") as Int,
        size.y - 1,
    );

    InputData {
        lines,
        start_pos,
        end_pos,
        size,
    }
}

impl<'a> InputData<'a> {
    fn get(&'a self, pos: &IVec2) -> Option<&'a str> {
        self.lines.get(pos.y as usize).and_then(|line| {
            let x = pos.x as usize;
            line.get(x..=x)
        })
    }
}

fn get_neighbours(data: &InputData, pos: &IVec2, direction: &IVec2) -> Vec<(IVec2, IVec2)> {
    match data.get(pos).expect("no sign found") {
        ">" => return vec![(*pos + IVec2::X, IVec2::X)],
        "V" => return vec![(*pos + IVec2::Y, IVec2::Y)],
        _ => {}
    }

    let mut neighbours = Vec::new();
    for d in [*direction, direction.yx(), -direction.yx()] {
        let pos = *pos + d;
        if let Some(sign) = data.get(&pos) {
            match sign {
                "." => {}
                ">" if d == IVec2::X => {}
                "v" if d == IVec2::Y => {}
                _ => continue,
            }

            neighbours.push((pos, d));
        }
    }
    neighbours
}

fn search(data: &InputData) -> Option<Int> {
    let mut queue = BinaryHeap::new();
    let mut entries = HashMap::new();

    let mut entry_id_inc = 1;
    entries.insert(entry_id_inc, (data.start_pos, IVec2::Y));
    queue.push((0, entry_id_inc));

    let mut records = HashMap::new();

    while let Some((distance, entry_id)) = queue.pop() {
        let (pos, direction) = entries.remove(&entry_id).expect("entry not found");

        if pos == data.end_pos {
            // found the end
            continue;
        }

        let new_distance = distance + 1;
        for next in get_neighbours(data, &pos, &direction) {
            entry_id_inc += 1;
            entries.insert(entry_id_inc, next);
            queue.push((new_distance, entry_id_inc));

            records
                .entry(next)
                .and_modify(|best_distance| {
                    if *best_distance < new_distance {
                        *best_distance = new_distance;
                    }
                })
                .or_insert(new_distance);
        }
    }

    for x in records.iter().sorted_by_key(|(_, v)| *v) {
        println!("record: {:?}", x);
    }

    records.get(&(data.end_pos, IVec2::Y)).cloned()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    #.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#
    "#};

    #[test]
    fn test_parse_input() {
        let data = parse_input(INPUT);
        assert_eq!(data.size, IVec2::new(23, 23));
        assert_eq!(data.start_pos, IVec2::new(1, 0));
        assert_eq!(data.end_pos, IVec2::new(21, 22));

        assert_eq!(data.get(&IVec2::new(0, 0)), Some("#"));
        assert_eq!(data.get(&IVec2::new(1, 1)), Some("."));
        assert_eq!(data.get(&IVec2::new(10, 3)), Some(">"));

        assert_eq!(data.get(&IVec2::new(-1, 0)), None);
        assert_eq!(data.get(&IVec2::new(0, -1)), None);
        assert_eq!(data.get(&IVec2::new(0, 100)), None);
        assert_eq!(data.get(&IVec2::new(100, 0)), None);
    }

    #[test]
    fn test_search() {
        let data = parse_input(INPUT);
        let ret = search(&data);
        println!("ret: {:?}", ret);
    }

    #[test]
    fn test_direction() {
        let a = IVec2::X;
        println!("a: {}", a.yx());
        println!("a: {}", -a.yx());

        let a = IVec2::Y;
        println!("a: {}", a.yx());
        println!("a: {}", -a.yx());

        let a = IVec2::NEG_X;
        println!("a: {}", a.yx());
        println!("a: {}", -a.yx());

        let a = IVec2::NEG_Y;
        println!("a: {}", a.yx());
        println!("a: {}", -a.yx());
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "94");
    }
}
