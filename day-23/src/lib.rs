use glam::IVec2;

pub type Int = i32;

#[derive(Debug)]
pub struct InputData<'a> {
    pub lines: Vec<&'a str>,
    pub start_pos: IVec2,
    pub end_pos: IVec2,
    pub size: IVec2,
}

pub fn parse_input(input: &str) -> InputData {
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
    pub fn get(&'a self, pos: &IVec2) -> Option<&'a str> {
        self.lines.get(pos.y as usize).and_then(|line| {
            let x = pos.x as usize;
            line.get(x..=x)
        })
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2Swizzles;
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
}
