use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufReader, Read};
use std::ops::RangeInclusive;

use anyhow::Context;
use glam::{IVec2, IVec3, Vec3Swizzles};
use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::{char, line_ending};
use nom::multi::separated_list1;
use nom::IResult;

type Int = i32;

#[derive(Debug)]
struct Brick(IVec3, IVec3);

#[derive(Debug)]
struct InputData {
    bricks: Vec<Brick>,
    size: IVec3,
}

#[derive(Debug)]
struct BrickEntry<'a> {
    id: Int,
    brick: &'a Brick,
    bottom: Int,
}

struct HeightMap {
    size: IVec2,
    /// (entry_id, height)
    data: Vec<(Int, Int)>,
    /// (bottom_entry_id, above_entry_id)
    supported: HashSet<(Int, Int)>,
}

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-22/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).unwrap();
    let mut entries = make_brick_entries(&data);

    let mut height_map = HeightMap::new(data.size.xy());
    let mut comsumed = HashSet::new();
    for current_z in 1..=data.size.z {
        let ids = find_bricks_with_z(&entries, current_z);
        for id in ids {
            if comsumed.contains(&id) {
                continue;
            }

            arrange(&mut height_map, &mut entries, id);
            comsumed.insert(id);
        }
    }

    dump_height_map_data(&height_map);
    dump_height_map_supported(&height_map);

    let mut candidates = Vec::new();
    entries.values().sorted_by_key(|e| e.bottom).for_each(|e| {
        if can_be_disintegrate(&height_map, e.id) {
            println!("OK: {:?}", e);
        } else {
            candidates.push(e.id);
            println!("    {:?}", e);
        }
    });

    println!("candidates: {:?}", candidates);

    // 壊して影響あるidを探索して、一番壊れる数が多いものを探索する。
    let count: usize = candidates
        .into_iter()
        .map(|id| {
            let mut supported = height_map.supported.clone();
            let c = search_chain(&mut supported, id);
            println!(
                "id:{} count:{} after supported:{:?}",
                id,
                c.len(),
                supported
            );
            c.len()
        })
        .sum();

    Ok(format!("{}", count))
}

fn arrange(height_map: &mut HeightMap, entries: &mut HashMap<Int, BrickEntry>, id: Int) {
    let entry = entries.get_mut(&id).expect("no entry");

    let bottom = entry.bottom;
    let xy_list: Vec<IVec2> = entry.brick.xy_list();

    let new_bottom = xy_list.iter().map(|xy| height_map.get(xy).1).max().unwrap() + 1;
    if new_bottom > bottom {
        panic!("new bottom is higher than old bottom entry_id:{}", id)
    }
    entry.bottom = new_bottom;

    for p in &xy_list {
        height_map.stack(p, entry);
    }
}

fn can_be_disintegrate(height_map: &HeightMap, entry_id: Int) -> bool {
    let above_ids: HashSet<_> = height_map
        .supported
        .iter()
        .filter(|(bottom, _)| bottom == &entry_id)
        .map(|(_, above)| *above)
        .collect();

    if above_ids.is_empty() {
        // 支えているブロックが一つもない。
        return true;
    }

    // 支えているブロックがある場合、そのブロックが他のブロックに支えられているかを確認する。
    // 一つでも支えられていないブロックがあれば、このブロックは破壊できない。
    above_ids.iter().all(|above_id| {
        height_map
            .supported
            .iter()
            .filter(|(_, above)| above == above_id)
            .any(|(bottom, _)| bottom != &entry_id)
    })
}

fn find_bricks_with_z(entries: &HashMap<Int, BrickEntry>, z: Int) -> Vec<Int> {
    entries
        .values()
        .filter(|e| e.z_range().contains(&z))
        .map(|e| e.id)
        .collect()
}

fn make_brick_entries(data: &InputData) -> HashMap<Int, BrickEntry> {
    let mut entries = HashMap::new();
    for (i, brick) in data.bricks.iter().enumerate() {
        let id = i as Int + 1;
        let bottom = brick.0.z.min(brick.1.z);
        entries.insert(id, BrickEntry { id, brick, bottom });
    }
    entries
}

/// `supported` is (bottom_entry_id, above_entry_id)
fn search_chain(supported: &mut HashSet<(Int, Int)>, id: Int) -> HashSet<Int> {
    //    println!("target id:{} supported: {:?}", id, supported);

    let above_entry_ids: Vec<_> = supported
        .iter()
        .filter(|(bottom, _)| bottom == &id)
        .map(|(_, above)| *above)
        .collect();

    for above_id in &above_entry_ids {
        supported.remove(&(id, *above_id));
    }

    let mut removed = HashSet::new();
    for above_id in above_entry_ids {
        if supported.iter().any(|(_, a)| a == &above_id) {
            continue;
        }
        removed.insert(above_id);
    }

    let mut count = removed.clone();
    for above_entry_id in &removed {
        let children = search_chain(supported, *above_entry_id);
        count.extend(children);
    }

    count
}

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    let (input, arr1) = separated_list1(char(','), complete::i32)(input)?;
    let (input, _) = char('~')(input)?;
    let (input, arr2) = separated_list1(char(','), complete::i32)(input)?;

    Ok((
        input,
        Brick(IVec3::from_slice(&arr1), IVec3::from_slice(&arr2)),
    ))
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, bricks) = separated_list1(line_ending, parse_brick)(input)?;
    let max_z = bricks.iter().map(|b| b.top()).max().expect("no bricks");
    let max_x = bricks
        .iter()
        .map(|b| b.0.x.max(b.1.x))
        .max()
        .expect("no bricks");
    let max_y = bricks
        .iter()
        .map(|b| b.0.y.max(b.1.y))
        .max()
        .expect("no bricks");
    let size = IVec3::new(max_x + 1, max_y + 1, max_z + 1);
    Ok((input, InputData { bricks, size }))
}

impl Brick {
    fn bottom(&self) -> Int {
        self.0.z.min(self.1.z)
    }

    fn top(&self) -> Int {
        self.0.z.max(self.1.z)
    }

    fn xy_list(&self) -> Vec<IVec2> {
        let mut list = Vec::new();
        for x in self.0.x..=self.1.x {
            for y in self.0.y..=self.1.y {
                list.push(IVec2::new(x, y));
            }
        }
        list
    }
}

impl BrickEntry<'_> {
    fn z_range(&self) -> RangeInclusive<Int> {
        let from = self.bottom;
        let to = self.bottom + (self.brick.0.z - self.brick.1.z).abs();
        from..=to
    }
}

impl HeightMap {
    fn new(size: IVec2) -> Self {
        let data = vec![(0, 0); (size.x * size.y) as usize];
        Self {
            data,
            size,
            supported: HashSet::new(),
        }
    }
    fn get(&self, pos: &IVec2) -> (Int, Int) {
        self.data
            .get((self.size.x * pos.y + pos.x) as usize)
            .copied()
            .with_context(|| format!("size:{:?} pos: {:?}", self.size, pos))
            .expect("no data")
    }

    fn stack(&mut self, pos: &IVec2, entry: &BrickEntry) {
        let entry_id = entry.id;
        let z_range = entry.z_range();

        let (bottom_entry_id, prev_height) = self.data[(self.size.x * pos.y + pos.x) as usize];
        self.data[(self.size.x * pos.y + pos.x) as usize] = (entry_id, *z_range.end());

        // 下と設置している場合に、supportedに追加する。
        if prev_height + 1 == *z_range.start() {
            self.supported.insert((bottom_entry_id, entry_id));
        }
    }
}

fn dump_height_map_data(height_map: &HeightMap) {
    println!("data (entry_id, height)");
    height_map
        .data
        .chunks(height_map.size.x as usize)
        .for_each(|row| {
            println!("data {:?}", row);
        });
}

fn dump_height_map_supported(height_map: &HeightMap) {
    height_map
        .supported
        .iter()
        .sorted_by_key(|e| e.0)
        .for_each(|e| {
            println!("supported {:?}", e);
        });
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9
    "#};

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();

        for b in &data.bricks {
            println!("{:?}", b);
        }
    }

    #[test]
    fn test_can_be_disintegrated() {
        let (_, data) = parse_input(INPUT).unwrap();
        let mut entries = make_brick_entries(&data);

        let mut height_map = HeightMap::new(data.size.xy());
        let mut comsumed = HashSet::new();
        for current_z in 1..=data.size.z {
            let ids = find_bricks_with_z(&entries, current_z);
            for id in ids {
                if comsumed.contains(&id) {
                    continue;
                }

                arrange(&mut height_map, &mut entries, id);
                comsumed.insert(id);
            }
        }

        dump_height_map_data(&height_map);
        dump_height_map_supported(&height_map);

        entries.values().sorted_by_key(|e| e.bottom).for_each(|e| {
            if can_be_disintegrate(&height_map, e.id) {
                println!("OK: {:?}", e)
            } else {
                println!("NG: {:?}", e)
            }
        });

        assert!(!can_be_disintegrate(&height_map, 1));
        assert!(!can_be_disintegrate(&height_map, 6));

        assert!(can_be_disintegrate(&height_map, 2));
        assert!(can_be_disintegrate(&height_map, 3));
        assert!(can_be_disintegrate(&height_map, 4));
        assert!(can_be_disintegrate(&height_map, 5));
        assert!(can_be_disintegrate(&height_map, 7));
    }

    #[test]
    fn test_sample() {
        let input = indoc! {r#"
        0,0,1~2,0,1
        3,0,1~3,0,1
        1,0,2~3,0,2
        0,0,3~2,0,3
        0,0,4~0,0,4
        "#};

        let (_, data) = parse_input(input).unwrap();
        let mut entries = make_brick_entries(&data);

        let mut height_map = HeightMap::new(data.size.xy());
        let mut comsumed = HashSet::new();
        for current_z in 1..=data.size.z {
            let ids = find_bricks_with_z(&entries, current_z);
            for id in ids {
                if comsumed.contains(&id) {
                    continue;
                }

                arrange(&mut height_map, &mut entries, id);
                comsumed.insert(id);
            }
        }

        dump_height_map_data(&height_map);
        dump_height_map_supported(&height_map);

        entries.values().sorted_by_key(|e| e.bottom).for_each(|e| {
            if can_be_disintegrate(&height_map, e.id) {
                println!("OK: {:?}", e)
            } else {
                println!("NG: {:?}", e)
            }
        });

        assert!(!can_be_disintegrate(&height_map, 3));
        assert!(!can_be_disintegrate(&height_map, 4));

        assert!(can_be_disintegrate(&height_map, 1));
        assert!(can_be_disintegrate(&height_map, 2));
        assert!(can_be_disintegrate(&height_map, 5));
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "7");
    }
}
