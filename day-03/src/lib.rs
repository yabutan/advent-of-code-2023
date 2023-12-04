use glam::{uvec2, UVec2};
use std::cmp::{max, min};

use nom::bytes::complete::take_till;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use nom_locate::LocatedSpan;

#[derive(Debug, Eq, PartialEq)]
pub struct Number<'a> {
    pub value: &'a str,
    pub pos: UVec2,
}

impl<'a> Number<'a> {
    pub fn value_as_u64(&self) -> u64 {
        self.value.parse::<u64>().expect("should be number")
    }

    /// シンボルと隣接しているかどうかを判定
    pub fn is_adjacent_symbol(&self, data: &[&str]) -> bool {
        self.find_round(data, |c: char| c != '.' && c.is_ascii_punctuation())
            .is_some()
    }

    /// 隣接しているギアの位置を返す、なければNone
    pub fn gear_pos(&self, data: &[&str]) -> Option<UVec2> {
        self.find_round(data, |c: char| c == '*')
    }

    /// 周りの指定文字の位置を返す、なければNone
    fn find_round(&self, data: &[&str], pat: fn(char) -> bool) -> Option<UVec2> {
        let pos_x = self.pos.x as usize;
        let pos_y = self.pos.y as usize;

        let range_x =
            max(0, pos_x.saturating_sub(1))..min(data[0].len(), pos_x + self.value.len() + 1);
        let range_y = max(0, pos_y.saturating_sub(1))..min(data.len(), pos_y + 2);

        for y in range_y {
            let line = data[y];
            let s = &line[range_x.clone()];
            if let Some(index) = s.find(pat) {
                return Some(uvec2((range_x.start + index) as u32, y as u32));
            }
        }
        None
    }
}

type Span<'a> = LocatedSpan<&'a str>;

pub fn parse_numbers(input: &str) -> anyhow::Result<Vec<Number>> {
    let input = Span::new(input);
    let (_, numbers) = many0(parse_number)(input)
        .map_err(|e| anyhow::anyhow!("failed to parse Numbers caused by {:?}", e))?;
    Ok(numbers)
}

fn parse_number(input: Span) -> IResult<Span, Number> {
    let (input, _) = take_till(|c: char| c.is_numeric())(input)?;
    let (input, num) = digit1(input)?;

    Ok((
        input,
        Number {
            value: num.fragment(),
            pos: uvec2(num.get_column() as u32 - 1, num.location_line() - 1),
        },
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#};

    #[test]
    fn test_is_adjacent_symbol() {
        let data = INPUT.lines().collect::<Vec<_>>();

        #[rustfmt::skip]
            let patterns = [
            (Number { value: "467", pos: uvec2(0, 0) }, true),
            (Number { value: "35",  pos: uvec2(2, 2) }, true),
            (Number { value: "755", pos: uvec2(6, 7) }, true),

            (Number { value: "114", pos: uvec2(5, 0) }, false),
            (Number { value: "58",  pos: uvec2(7, 5) }, false),
        ];

        for (number, expected) in patterns {
            assert_eq!(number.is_adjacent_symbol(&data), expected);
        }
    }

    #[test]
    fn test_parse_numbers() {
        let numbers = parse_numbers(INPUT).unwrap();

        #[rustfmt::skip]
        let patterns = [
            (0, Number { value: "467", pos: uvec2(0, 0)}),
            (1, Number { value: "114", pos: uvec2(5, 0)}),
            (8, Number { value: "664", pos: uvec2(1, 9)}),
            (9, Number { value: "598", pos: uvec2(5, 9)}),
        ];

        assert_eq!(numbers.len(), 10);
        for (i, number) in patterns {
            assert_eq!(numbers[i], number);
        }
    }

    #[test]
    fn test_gear_pos() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        let numbers = parse_numbers(INPUT).unwrap();

        #[rustfmt::skip]
        let patterns = [
            (0, Some(uvec2(3, 1))),
            (1, None),
            (2, Some(uvec2(3, 1))),
            (3, None),
            (4, Some(uvec2(3, 4))),
            (5, None),
            (6, None),
            (7, Some(uvec2(5, 8))),
            (8, None),
            (9, Some(uvec2(5, 8))),
        ];

        for (i, gear_pos) in patterns {
            assert_eq!(numbers[i].gear_pos(&lines), gear_pos);
        }
    }
}
