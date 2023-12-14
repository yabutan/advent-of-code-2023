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

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let smudge = 1;

    let mut total = 0;
    for p in &data.patterns {
        let rows = find_reflection(p, smudge);
        let columns = find_reflection(&transpose(p), smudge);

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

struct InputData<'a> {
    patterns: Vec<Pattern<'a>>,
}

#[derive(Debug)]
struct Pattern<'a> {
    lines: Vec<Cow<'a, str>>,
    size: UVec2,
}

#[derive(Debug, Eq, PartialEq)]
struct Reflection {
    count: u32,
    start: u32,
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

fn match_lines(a: &str, b: &str, rest_of_smudge: &mut usize) -> bool {
    let mut a = a.chars();
    let mut b = b.chars();

    loop {
        match (a.next(), b.next()) {
            (None, None) => break,
            (Some(c1), Some(c2)) if c1 == c2 => continue,
            (Some(_), Some(_)) => {
                if *rest_of_smudge == 0 {
                    return false;
                }
                *rest_of_smudge -= 1;
            }
            _ => continue,
        }
    }

    true
}

/// smudge: 汚れ個数
fn find_reflection(pattern: &Pattern, smudge: usize) -> Option<Reflection> {
    let mut reflactions = Vec::new();
    for i in 0..(pattern.lines.len() - 1) {
        if let Some(reflection) = seek(pattern, i, smudge) {
            reflactions.push(reflection);
        }
    }

    // 複数ある場合は、一番広い範囲のものを選ぶ
    reflactions.into_iter().max_by_key(|r| r.count)
}

fn get_line<'a>(pattern: &'a Pattern, i: i32) -> Option<Cow<'a, str>> {
    if i < 0 {
        return None;
    }
    pattern.lines.get(i as usize).cloned()
}

fn seek(pattern: &Pattern, i: usize, mut rest_of_smudge: usize) -> Option<Reflection> {
    let mut u = i as i32;
    let mut d = i as i32 + 1;
    loop {
        let up = get_line(pattern, u);
        let down = get_line(pattern, d);

        // どちらもNoneなら終了
        if up.is_none() && down.is_none() {
            break;
        } else if let (Some(up), Some(down)) = (up, down) {
            // 中身が違うなら終了
            if !match_lines(&up, &down, &mut rest_of_smudge) {
                break;
            }
        }

        u -= 1;
        d += 1;
    }

    if rest_of_smudge != 0 {
        // 汚れの数が一致していない。
        return None;
    }

    Some(Reflection {
        count: (d - i as i32 - 1) as u32,
        start: i as u32 + 1,
    })
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
    fn test_match_lines() {
        assert!(match_lines("#.##..##.", "#.##..##.", &mut 0));
        assert!(!match_lines("#.##..##.", "..##..##.", &mut 0));

        let mut smudge = 1;
        assert!(match_lines("#.##..##.", "#.##..##.", &mut smudge));
        assert_eq!(smudge, 1);

        let mut smudge = 1;
        assert!(match_lines("#.##..##.", "..##..##.", &mut smudge));
        assert_eq!(smudge, 0);
    }

    #[test]
    fn test_transpose() {
        let input = indoc! {r#"
        #.##
        ..#.
        ##..
        "#};

        let expect = indoc! {r#"
        #.#
        ..#
        ##.
        #..
        "#};

        let (_, data) = parse_input(input).unwrap();
        let p = transpose(&data.patterns[0]);
        assert_eq!(p.size, uvec2(3, 4));
        assert_eq!(p.lines.join("\n") + "\n", expect);
    }

    #[test]
    fn test_find_reflection() {
        let (_, data) = parse_input(INPUT).unwrap();

        let p = &data.patterns[0];
        assert_eq!(
            find_reflection(p, 0),
            Some(Reflection { count: 2, start: 3 })
        );
        assert_eq!(
            find_reflection(&transpose(p), 0),
            Some(Reflection { count: 5, start: 5 })
        );

        let p = &data.patterns[1];
        assert_eq!(
            find_reflection(p, 0),
            Some(Reflection { count: 4, start: 4 })
        );
        assert_eq!(
            find_reflection(&transpose(p), 0),
            Some(Reflection { count: 1, start: 7 })
        );
    }

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        for p in &data.patterns {
            println!("{:?}", p);
        }

        assert_eq!(data.patterns.len(), 2);
        assert_eq!(data.patterns[0].size, uvec2(9, 7));
        assert_eq!(data.patterns[1].size, uvec2(9, 7));
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "400");
    }
}
