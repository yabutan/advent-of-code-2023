use std::borrow::Cow;
use std::fs;
use std::io::{BufReader, Read};

use glam::{uvec2, UVec2};
use nom::bytes::complete::is_a;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-13/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Reflection {
    count: u32,
    start: u32,
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let mut total = 0;
    for p in &data.patterns {
        let rows = find_reflection_rows(p, 1);
        let columns = find_reflection_rows(&transpose(p), 1);

        total += match (&rows, &columns) {
            (Some(rows), Some(columns)) if rows.count > columns.count => rows.start * 100,
            (Some(rows), Some(columns)) if rows.count < columns.count => columns.start,
            (Some(rows), None) => rows.start * 100,
            (None, Some(columns)) => columns.start,
            _ => panic!("invalid pattern row{:?} columns{:?}", rows, columns),
        };
    }

    Ok(total.to_string())
}

#[derive(Debug)]
struct Pattern<'a> {
    lines: Vec<Cow<'a, str>>,
    size: UVec2,
}
struct InputData<'a> {
    patterns: Vec<Pattern<'a>>,
}

fn transpose<'a>(pattern: &Pattern) -> Pattern<'a> {
    let mut lines = Vec::new();
    for x in 0..pattern.size.x {
        let mut line = String::new();
        for y in 0..pattern.size.y {
            line.push(pattern.lines[y as usize].chars().nth(x as usize).unwrap());
        }
        lines.push(line);
    }

    let lines = lines.into_iter().map(Cow::from).collect::<Vec<_>>();

    Pattern {
        lines,
        size: uvec2(pattern.size.y, pattern.size.x),
    }
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    fn pattern(input: &str) -> IResult<&str, Pattern> {
        let (input, lines) = separated_list1(newline, is_a(".#"))(input)?;

        let lines = lines.into_iter().map(Cow::from).collect::<Vec<_>>();
        let size = uvec2(lines[0].len() as u32, lines.len() as u32);

        Ok((input, Pattern { lines, size }))
    }

    let (input, patterns) = separated_list1(tuple((newline, newline)), pattern)(input)?;
    Ok((input, InputData { patterns }))
}

/// l: 許容する不一致文字数
/// return: 一致するなら、Some(まだ許容できる不一致数)
///        一致しないなら、None
fn match_line(a: &str, b: &str, mut l: usize) -> Option<usize> {
    let mut a = a.chars();
    let mut b = b.chars();

    loop {
        let (Some(c1), Some(c2)) = (a.next(), b.next()) else {
            break;
        };

        if c1 == c2 {
            continue;
        }

        if l == 0 {
            return None;
        }

        l -= 1;
    }
    Some(l)
}

fn find_reflection_rows(pattern: &Pattern, l: usize) -> Option<Reflection> {
    let mut reflactions = Vec::new();
    for (i, lines) in pattern.lines.windows(2).enumerate() {
        if let Some(l) = match_line(&lines[0], &lines[0], l) {
            let (reflection, l) = count_reflection(pattern, i, l);
            if l == 0 {
                reflactions.push(reflection);
            }
        }
    }

    reflactions.into_iter().max_by_key(|r| r.count)
}

fn get_row<'a>(pattern: &'a Pattern, i: i32) -> Option<Cow<'a, str>> {
    if i < 0 {
        return None;
    }
    pattern.lines.get(i as usize).cloned()
}

/// l: 許容する不一致文字数
fn count_reflection(pattern: &Pattern, i: usize, mut l: usize) -> (Reflection, usize) {
    let mut u = i as i32;
    let mut d = i as i32 + 1;
    loop {
        let up = get_row(pattern, u);
        let down = get_row(pattern, d);

        // どちらもNoneなら終了
        if up.is_none() && down.is_none() {
            break;
        }

        // 中身が違うなら終了
        if let (Some(up), Some(down)) = (up, down) {
            if let Some(l2) = match_line(&up, &down, l) {
                l = l2;
            } else {
                break;
            }
        }

        u -= 1;
        d += 1;
    }

    (
        Reflection {
            count: (d - i as i32 - 1) as u32,
            start: i as u32 + 1,
        },
        l,
    )
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    "#};

    #[test]
    fn test_match_line() {
        assert_eq!(match_line("#.##..##.", "#.##..##.", 0), Some(0));
        assert_eq!(match_line("#.##..##.", "#.##..##.", 1), Some(1));
        assert_eq!(match_line("#.##..##.", "..##..##.", 0), None);
        assert_eq!(match_line("#.##..##.", "..##..##.", 1), Some(0));
    }

    #[test]
    fn test_find_reflection_rows() {
        let (_, data) = parse_input(INPUT).unwrap();

        let p = &data.patterns[0];
        assert_eq!(
            find_reflection_rows(p, 0),
            Some(Reflection { count: 2, start: 3 })
        );
        let p = &transpose(p);
        assert_eq!(
            find_reflection_rows(p, 0),
            Some(Reflection { count: 5, start: 5 })
        );

        let p = &data.patterns[1];
        assert_eq!(
            find_reflection_rows(p, 0),
            Some(Reflection { count: 4, start: 4 })
        );
        let p = &transpose(p);
        assert_eq!(
            find_reflection_rows(p, 0),
            Some(Reflection { count: 1, start: 7 })
        );
    }

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        for p in &data.patterns {
            println!("{:?}", p);
        }
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "400");
    }
}
