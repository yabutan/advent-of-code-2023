use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::io::{BufReader, Read};

use glam::{IVec2, Vec2Swizzles};
use itertools::Itertools;

use day_23::{parse_input, InputData, Int};

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

fn get_neighbours(
    data: &InputData,
    pos: &IVec2,
    direction: &IVec2,
    paths: &HashSet<IVec2>,
    edges: &HashSet<IVec2>,
) -> Vec<(IVec2, IVec2)> {
    // match data.get(pos).expect("no sign found") {
    //     ">" => return vec![],
    //     "V" => return vec![(*pos + IVec2::Y, IVec2::Y)],
    //     _ => {}
    // }

    let mut neighbours = Vec::new();
    for d in [*direction, direction.yx(), -direction.yx()] {
        let pos = *pos + d;
        if paths.contains(&pos) {
            continue;
        }
        if edges.contains(&pos) {
            continue;
        }

        if let Some(sign) = data.get(&pos) {
            match sign {
                "." => {}
                ">" => {}
                "v" => {}
                // ">" if d == IVec2::X => {}
                // "v" if d == IVec2::Y => {}
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
    entries.insert(
        entry_id_inc,
        (data.start_pos, IVec2::Y, HashSet::new(), None),
    );
    queue.push((0, entry_id_inc));

    let mut records = HashMap::new();
    let mut edges = HashSet::new();

    while let Some((distance, entry_id)) = queue.pop() {
        let (pos, direction, mut paths, prev_path) =
            entries.remove(&entry_id).expect("entry not found");

        if pos == data.end_pos {
            // found the end
            let max = records.get(&(data.end_pos, IVec2::Y));
            println!(
                "found the end: {} queue.len: {} max:{:?}",
                distance,
                queue.len(),
                max
            );
            continue;
        }

        let new_distance = distance + 1;

        let neighbours = get_neighbours(data, &pos, &direction, &paths, &edges);
        if neighbours.is_empty() {
            if let Some(prev_path) = prev_path {
                edges.insert(prev_path);
                println!("edge: {:?}", prev_path);
            }
            continue;
        }

        let is_cross = neighbours.len() >= 2;
        if is_cross {
            // 分岐路であれば、通ったことがあると、path記録しておく。
            paths.insert(pos);
        }
        for next in neighbours {
            entry_id_inc += 1;
            entries.insert(
                entry_id_inc,
                (
                    next.0,
                    next.1,
                    paths.clone(),
                    if is_cross { Some(next.0) } else { None },
                ),
            );
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
    fn test_search() {
        let data = parse_input(INPUT);
        let ret = search(&data);
        println!("ret: {:?}", ret);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "154");
    }
}
