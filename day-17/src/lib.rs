use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::ops::RangeInclusive;

use glam::{ivec2, IVec2};
use nom::character::complete::{digit1, newline};
use nom::multi::separated_list1;
use nom::IResult;

type Int = i32;

#[derive(Debug)]
pub struct InputData {
    grid: Vec<Vec<Int>>,
    size: IVec2,
}

pub struct Searcher<'a> {
    pub data: &'a InputData,
    pub consecutive: RangeInclusive<Int>,
}

#[derive(Debug, Eq, PartialEq)]
struct Next {
    pos: IVec2,
    direction: IVec2,
    heat_loss: Int,
}

#[rustfmt::skip]
const DIRECTIONS: [IVec2; 4] = [
    ivec2(-1, 0),
    ivec2(1, 0),
    ivec2(0, -1),
    ivec2(0, 1),
];

pub fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, lines) = separated_list1(newline, digit1)(input)?;
    let size = IVec2::new(lines[0].len() as Int, lines.len() as Int);

    let mut grid = Vec::new();
    for (_y, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (_x, c) in line.chars().enumerate() {
            row.push(c.to_digit(10).unwrap() as Int);
        }
        grid.push(row);
    }

    Ok((input, InputData { grid, size }))
}

impl InputData {
    pub fn get(&self, pos: &IVec2) -> Option<Int> {
        self.grid
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
            .copied()
    }
}

impl Searcher<'_> {
    pub fn search(&self) -> Int {
        let data = self.data;
        let end_pos = ivec2(data.size.x - 1, data.size.y - 1);

        // (heat_loss, [x,y], [dx,dy])
        let mut queues = BinaryHeap::new();
        queues.push((Reverse(0), ivec2(0, 0).to_array(), ivec2(0, 0).to_array()));

        // (pos, direction) => heat_loss
        let mut nodes = HashMap::new();

        while let Some((Reverse(heat_loss), pos, direction)) = queues.pop() {
            println!("queues.len: {}", queues.len());
            let pos = IVec2::from(pos);
            let direction = IVec2::from(direction);

            if pos == end_pos {
                return heat_loss;
            }

            if nodes
                .get(&(pos, direction))
                .is_some_and(|recorded| recorded < &heat_loss)
            {
                continue;
            }

            let candidates = self.get_next(&pos, &direction);
            for next in candidates {
                let new_heat_loss = heat_loss + next.heat_loss;

                let key = (next.pos, next.direction);
                if new_heat_loss >= *nodes.get(&key).unwrap_or(&Int::MAX) {
                    continue;
                }

                nodes.insert(key, new_heat_loss);

                queues.push((
                    Reverse(new_heat_loss),
                    next.pos.to_array(),
                    next.direction.to_array(),
                ));
            }
        }
        unreachable!()
    }

    fn get_next(&self, pos: &IVec2, d: &IVec2) -> Vec<Next> {
        let mut candidates = Vec::new();
        for direction in DIRECTIONS {
            if *d == direction || -direction == *d {
                // 逆戻りと、同じ方向には生成しない。(max指定分すでに進行させているため)
                continue;
            }

            let mut total_loss = 0;
            for m in (1 as Int)..=(*self.consecutive.end()) {
                let next_pos = *pos + (direction * m);
                let Some(heat_loss) = self.data.get(&next_pos) else {
                    break;
                };

                total_loss += heat_loss;

                if m < *self.consecutive.start() {
                    continue;
                }

                candidates.push(Next {
                    pos: next_pos,
                    direction,
                    heat_loss: total_loss,
                });
            }
        }

        candidates
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    use glam::ivec2;
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    "#};

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:?}", data);

        assert_eq!(data.get(&ivec2(0, 0)), Some(2));
        assert_eq!(data.get(&ivec2(12, 0)), Some(3));
        assert_eq!(data.get(&ivec2(0, 12)), Some(4));
        assert_eq!(data.get(&ivec2(12, 12)), Some(3));
    }

    #[test]
    fn test_next() {
        let (_, data) = parse_input(INPUT).unwrap();

        let right = ivec2(1, 0);
        let down = ivec2(0, 1);

        let s = Searcher {
            data: &data,
            consecutive: 1..=3,
        };

        assert_eq!(
            s.get_next(&ivec2(0, 0), &ivec2(0, 0)),
            vec![
                Next {
                    pos: ivec2(1, 0),
                    direction: right,
                    heat_loss: 4
                },
                Next {
                    pos: ivec2(2, 0),
                    direction: right,
                    heat_loss: 5
                },
                Next {
                    pos: ivec2(3, 0),
                    direction: right,
                    heat_loss: 8
                },
                Next {
                    pos: ivec2(0, 1),
                    direction: down,
                    heat_loss: 3
                },
                Next {
                    pos: ivec2(0, 2),
                    direction: down,
                    heat_loss: 6
                },
                Next {
                    pos: ivec2(0, 3),
                    direction: down,
                    heat_loss: 9
                },
            ]
        );

        let s = Searcher {
            data: &data,
            consecutive: 3..=3,
        };

        assert_eq!(
            s.get_next(&ivec2(0, 0), &ivec2(0, 0)),
            vec![
                Next {
                    pos: ivec2(3, 0),
                    direction: right,
                    heat_loss: 8
                },
                Next {
                    pos: ivec2(0, 3),
                    direction: down,
                    heat_loss: 9
                },
            ]
        );
    }

    #[test]
    fn test_binary_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(3);
        heap.push(9);

        // ただのイテレーション時には、順序付はされていない。
        println!("{:?}", heap);

        // popする際には最大順に出力される。
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(1));

        // Reverseを使うと、最小順に出力される。
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(1));
        heap.push(Reverse(3));
        heap.push(Reverse(9));
        assert_eq!(heap.pop(), Some(Reverse(1)));
        assert_eq!(heap.pop(), Some(Reverse(3)));
        assert_eq!(heap.pop(), Some(Reverse(9)));
    }
}
