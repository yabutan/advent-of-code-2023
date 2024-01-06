use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;

pub type IVec3 = glam::I64Vec3;
pub type Vec2 = glam::DVec2;
pub type Float = f64;

#[derive(Debug)]
pub struct InputData {
    pub hailstones: Vec<(IVec3, IVec3)>,
}

fn parse_ivec3(input: &str) -> IResult<&str, IVec3> {
    let sep = || tuple((tag(","), space1));

    let (input, (x, y, z)) = tuple((
        complete::i64,
        preceded(sep(), complete::i64),
        preceded(sep(), complete::i64),
    ))(input)?;

    Ok((input, IVec3::new(x, y, z)))
}

pub fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, hailstones) = separated_list1(
        line_ending,
        separated_pair(
            parse_ivec3,
            delimited(space1, tag("@"), space1),
            parse_ivec3,
        ),
    )(input)?;

    Ok((input, InputData { hailstones }))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3
    "#};

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();

        for x in data.hailstones {
            println!("{:?}", x);
        }
    }
}
