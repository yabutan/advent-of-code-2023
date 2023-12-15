use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{BufReader, Read};

use glam::IVec2;
use itertools::Itertools;
use nom::bytes::complete::is_a;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug)]
struct InputData<'a> {
    platform: Vec<&'a str>,
    size: IVec2,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum RockShape {
    Round,
    Cube,
}

#[derive(Debug, Clone)]
struct Rock {
    id: EntryId,
    pos: IVec2,
    shape: RockShape,
}

type EntryId = u32;
struct EntryManager<'a> {
    data: &'a InputData<'a>,

    /// id => rock
    entries: HashMap<EntryId, Rock>,

    /// (x,y) => id
    grid: BTreeMap<(i32, i32), EntryId>,
}

impl<'a> EntryManager<'a> {
    fn new(data: &'a InputData) -> Self {
        let rocks = get_rocks(data);
        let entries: HashMap<_, _> = rocks.into_iter().map(|o: Rock| (o.id, o)).collect();
        let grid: BTreeMap<_, _> = entries
            .values()
            .map(|o| ((o.pos.x, o.pos.y), o.id))
            .collect();

        Self {
            data,
            entries,
            grid,
        }
    }

    fn get_v_line(&self, x: i32) -> Vec<EntryId> {
        (0..self.data.size.y)
            .filter_map(|y| self.grid.get(&(x, y)))
            .copied()
            .collect::<Vec<_>>()
    }
    fn get_h_line(&self, y: i32) -> Vec<EntryId> {
        (0..self.data.size.x)
            .filter_map(|x| self.grid.get(&(x, y)))
            .copied()
            .collect::<Vec<_>>()
    }

    fn set_grid(&mut self, id: &EntryId, pos: IVec2) {
        let rock = self.entries.get_mut(id).unwrap();

        self.grid.remove(&(rock.pos.x, rock.pos.y));
        self.grid.insert((pos.x, pos.y), rock.id);
        rock.pos = pos;
    }

    fn move_to_north(&mut self, vertical_line: &[EntryId]) {
        let mut top = 0;
        for entry_id in vertical_line {
            let rock = &self.entries[entry_id];
            match rock.shape {
                RockShape::Round => {
                    if top <= rock.pos.y {
                        self.set_grid(entry_id, IVec2::new(rock.pos.x, top));
                        top += 1;
                    }
                }
                RockShape::Cube => {
                    top = rock.pos.y + 1;
                }
            }
        }
    }

    fn move_to_south(&mut self, vertical_line: &[EntryId]) {
        let mut bottom = self.data.size.y - 1;
        for entry_id in vertical_line.iter().rev() {
            let rock = &self.entries[entry_id];
            match rock.shape {
                RockShape::Round => {
                    if bottom >= rock.pos.y {
                        self.set_grid(entry_id, IVec2::new(rock.pos.x, bottom));
                        bottom -= 1;
                    }
                }
                RockShape::Cube => {
                    bottom = rock.pos.y - 1;
                }
            }
        }
    }

    fn move_to_west(&mut self, horizontal_line: &[EntryId]) {
        let mut left = 0;
        for entry_id in horizontal_line {
            let rock = &self.entries[entry_id];
            match rock.shape {
                RockShape::Round => {
                    if left <= rock.pos.x {
                        self.set_grid(entry_id, IVec2::new(left, rock.pos.y));
                        left += 1;
                    }
                }
                RockShape::Cube => {
                    left = rock.pos.x + 1;
                }
            }
        }
    }

    fn move_to_east(&mut self, horizontal_line: &[EntryId]) {
        let mut right = self.data.size.x - 1;
        for entry_id in horizontal_line.iter().rev() {
            let rock = &self.entries[entry_id];
            match rock.shape {
                RockShape::Round => {
                    if right >= rock.pos.x {
                        self.set_grid(entry_id, IVec2::new(right, rock.pos.y));
                        right -= 1;
                    }
                }
                RockShape::Cube => {
                    right = rock.pos.x - 1;
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-14/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, lines) = separated_list1(newline, is_a(".O#"))(input)?;
    let size = IVec2::new(lines[0].len() as i32, lines.len() as i32);

    Ok((
        input,
        InputData {
            platform: lines,
            size,
        },
    ))
}

fn get_rocks(data: &InputData) -> Vec<Rock> {
    let mut rocks = Vec::new();
    for (y, line) in data.platform.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            let id = rocks.len() as u32 + 1;
            let pos = IVec2::new(x as i32, y as i32);
            let shape = match c {
                'O' => RockShape::Round,
                '#' => RockShape::Cube,
                _ => unreachable!(),
            };

            rocks.push(Rock { id, pos, shape });
        }
    }

    rocks
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let target_i = find_cycle(&data, 1000000000).expect("not found");
    println!("target_id: {}", target_i);

    let mut manager = EntryManager::new(&data);
    for _ in 0..=target_i {
        cycle(&mut manager);
    }

    let total = calc_total(&data, manager.entries.values());

    Ok(total.to_string())
}

fn make_key(manager: &EntryManager) -> String {
    manager
        .grid
        .keys()
        .map(|(x, y)| format!("{}-{}", x, y))
        .join(",")
}
fn find_cycle(data: &InputData, max: i32) -> Option<i32> {
    let mut manager = EntryManager::new(data);

    let mut cache: HashMap<_, i32> = HashMap::new();
    for i in 0..max {
        println!("i: {}", i);

        let key = make_key(&manager);
        match cache.get(&key) {
            None => {
                cycle(&mut manager);
                cache.insert(key, i);
            }
            Some(&start_i) => {
                // キャッシュが存在すれば、繰り返しが存在する。
                println!("found cycled start_i: {} to i: {}", start_i, i);
                // 繰り返しの間隔
                let span = i - start_i;

                // 最後まで繰り返した場合と同じになるインデックスを計算
                let index = ((max - start_i - 1) % span) + start_i;
                return Some(index);
            }
        }
    }
    None
}

fn cycle(manager: &mut EntryManager) {
    // move to north
    for x in 0..manager.data.size.x {
        let line = manager.get_v_line(x);
        manager.move_to_north(&line);
    }

    // move to west
    for y in 0..manager.data.size.y {
        let line = manager.get_h_line(y);
        manager.move_to_west(&line);
    }

    // move to south
    for x in 0..manager.data.size.x {
        let line = manager.get_v_line(x);
        manager.move_to_south(&line);
    }

    // move to east
    for y in 0..manager.data.size.y {
        let line = manager.get_h_line(y);
        manager.move_to_east(&line);
    }
}

fn dump_rocks<'a>(data: &InputData, rocks: impl Iterator<Item = &'a Rock>) -> Vec<String> {
    let rocks = rocks
        .map(|o| {
            let c = match o.shape {
                RockShape::Round => 'O',
                RockShape::Cube => '#',
            };

            (o.pos, c)
        })
        .collect::<HashMap<IVec2, char>>();

    let mut lines = Vec::new();
    for y in 0..data.size.y {
        let mut line = String::new();
        for x in 0..data.size.x {
            let c = rocks.get(&IVec2::new(x, y)).unwrap_or(&'.');
            line.push(*c);
        }
        lines.push(line);
    }

    lines
}

fn calc_total<'a>(data: &InputData, rocks: impl Iterator<Item = &'a Rock>) -> i32 {
    rocks
        .filter(|o| o.shape == RockShape::Round)
        .map(|o| data.size.y - o.pos.y)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    "#};

    #[test]
    fn test_cycle() {
        let (_, data) = parse_input(INPUT).unwrap();

        let mut manager = EntryManager::new(&data);
        cycle(&mut manager);
        assert_eq!(
            dump_rocks(&data, manager.entries.values()).join("\n") + "\n",
            indoc! {r#"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
            "#},
            "after 1 cycle"
        );

        cycle(&mut manager);
        assert_eq!(
            dump_rocks(&data, manager.entries.values()).join("\n") + "\n",
            indoc! {r#"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O
            "#},
            "after 2 cycles"
        );

        cycle(&mut manager);
        assert_eq!(
            dump_rocks(&data, manager.entries.values()).join("\n") + "\n",
            indoc! {r#"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O
            "#},
            "after 3 cycles"
        );
    }

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        let mut manager = EntryManager::new(&data);

        // move to north
        for x in 0..data.size.x {
            let line = manager.get_v_line(x);
            manager.move_to_north(&line);
        }

        assert_eq!(
            dump_rocks(&data, manager.entries.values()).join("\n") + "\n",
            indoc! {r#"
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
            "#}
        );
        assert_eq!(calc_total(&data, manager.entries.values()), 136);

        // move to west
        for y in 0..data.size.y {
            let line = manager.get_h_line(y);
            manager.move_to_west(&line);
        }
        assert_eq!(
            dump_rocks(&data, manager.entries.values()).join("\n") + "\n",
            indoc! {r#"
            OOOO.#O...
            OO..#....#
            OOO..##O..
            O..#OO....
            ........#.
            ..#....#.#
            O....#OO..
            O.........
            #....###..
            #....#....
            "#}
        );

        // move to south
        for x in 0..data.size.x {
            let line = manager.get_v_line(x);
            manager.move_to_south(&line);
        }
        assert_eq!(
            dump_rocks(&data, manager.entries.values()).join("\n") + "\n",
            indoc! {r#"
            .....#....
            ....#.O..#
            O..O.##...
            O.O#......
            O.O....O#.
            O.#..O.#.#
            O....#....
            OO....OO..
            #O...###..
            #O..O#....
            "#}
        );

        // move to east
        for y in 0..data.size.y {
            let line = manager.get_h_line(y);
            manager.move_to_east(&line);
        }
        assert_eq!(
            dump_rocks(&data, manager.entries.values()).join("\n") + "\n",
            indoc! {r#"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
            "#}
        );
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "64");
    }
}
