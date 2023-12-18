use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alphanumeric1, char, newline, space1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::delimited;

pub type Int = i64;

pub type IVec2 = glam::I64Vec2;

#[derive(Debug)]
pub struct Operation<'a> {
    pub direction: &'a str,
    pub distance: Int,
    pub color: &'a str,
}

#[derive(Debug)]
pub struct InputData<'a> {
    pub operations: Vec<Operation<'a>>,
}

pub fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, operations) = separated_list1(newline, parse_operation)(input)?;
    Ok((input, InputData { operations }))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, direction) = alt((tag("R"), tag("L"), tag("U"), tag("D")))(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = complete::i64(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = delimited(tag("(#"), alphanumeric1, char(')'))(input)?;

    Ok((
        input,
        Operation {
            direction,
            distance,
            color,
        },
    ))
}

pub fn convert_correct_input(data: InputData) -> InputData {
    let operations = data
        .operations
        .into_iter()
        .map(|o| {
            let code = &o.color[0..5];
            let distance = Int::from_str_radix(code, 16).expect("invalid code");

            let direction = match &o.color[5..] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => panic!("unknown direction"),
            };

            Operation {
                direction,
                distance,
                color: o.color,
            }
        })
        .collect();

    InputData { operations }
}

pub fn make_vertices(data: &InputData) -> Vec<IVec2> {
    let mut pos = IVec2::splat(0);
    let mut list = Vec::new();
    list.push(pos);

    for ope in &data.operations {
        let direction = match ope.direction {
            "L" => IVec2::new(-1, 0),
            "R" => IVec2::new(1, 0),
            "U" => IVec2::new(0, -1),
            "D" => IVec2::new(0, 1),
            _ => panic!("unknown direction"),
        };

        pos += direction * ope.distance;
        list.push(pos);
    }

    list
}
pub fn calc_area(data: &InputData, p: &[IVec2]) -> Int {
    // Shoelace Theorem (座標式, 靴紐法)
    let mut area = 0;
    for i in 0..p.len() {
        let i2 = (i + 1) % p.len();
        area += p[i].x * p[i2].y - p[i2].x * p[i].y;
    }
    let area = area.abs();

    // Pick's theorem
    let perimeter: Int = data.operations.iter().map(|o| o.distance).sum();
    perimeter + (area - perimeter) / 2 + 1
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)
    "#};

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:?}", data);

        assert_eq!(data.operations.len(), 14);
        assert_eq!(data.operations[0].direction, "R");
        assert_eq!(data.operations[0].distance, 6);
        assert_eq!(data.operations[0].color, "70c710");

        assert_eq!(data.operations[13].direction, "U");
        assert_eq!(data.operations[13].distance, 2);
        assert_eq!(data.operations[13].color, "7a21e3");

        let data = convert_correct_input(data);
        assert_eq!(data.operations[0].direction, "R");
        assert_eq!(data.operations[0].distance, 461937);
        assert_eq!(data.operations[13].direction, "U");
        assert_eq!(data.operations[13].distance, 500254);
    }
}
