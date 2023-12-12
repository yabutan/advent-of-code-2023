use std::fs;
use std::io::{BufReader, Read};

use complete::char;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::newline;
use nom::character::streaming::space1;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-12/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("parse_input: {:?}", e))?;

    let mut total = 0;
    for (i, criterion) in data.criteria.iter().enumerate() {
        print!("({}/{}) {:?}", i, data.criteria.len(), criterion);
        let patterns = make_patterns(criterion);
        println!(" -> {}", patterns.len());

        total += patterns.len();
    }

    Ok(total.to_string())
}

#[derive(Debug)]
struct InputData<'a> {
    criteria: Vec<Criterion<'a>>,
}

#[derive(Debug)]
struct Criterion<'a> {
    springs: &'a str,
    nums: Vec<i32>,
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    fn criterion(input: &str) -> IResult<&str, Criterion> {
        let (input, springs) = take_until(" ")(input)?;
        let (input, _) = space1(input)?;
        let (input, nums) = separated_list1(tag(","), complete::i32)(input)?;
        Ok((input, Criterion { springs, nums }))
    }

    let (input, criteria) = separated_list1(newline, criterion)(input)?;
    Ok((input, InputData { criteria }))
}

#[derive(Debug)]
struct Pattern {
    simulate: String,
}

enum Sign {
    Dot(u8),
    Hash(u8),
    Question(u8),
}

type Spring = Vec<Sign>;

fn parse_spring(input: &str) -> IResult<&str, Spring> {
    many1(alt((
        map(many1(char('.')), |c| Sign::Dot(c.len() as u8)),
        map(many1(char('#')), |c| Sign::Hash(c.len() as u8)),
        map(many1(char('?')), |c| Sign::Question(c.len() as u8)),
    )))(input)
}

fn get_problems(input: &str) -> anyhow::Result<Vec<i32>> {
    let (_, spring) = parse_spring(input).expect("parse_spring");
    let mut problems = Vec::new();
    for s in &spring {
        match s {
            Sign::Dot(_) => {}
            Sign::Hash(n) => problems.push(*n as i32),
            Sign::Question(_) => return Err(anyhow::anyhow!("? is not allowed")),
        }
    }
    Ok(problems)
}

fn make_patterns(criterion: &Criterion) -> Vec<String> {
    let question_count = criterion.springs.chars().filter(|c| c == &'?').count();

    let mut patterns = Vec::new();

    let combinations = make_combinations(question_count);
    for mut arr in combinations {
        let mut s = String::new();
        for c in criterion.springs.chars() {
            if c == '?' {
                s.push(arr.pop().expect("pop"));
            } else {
                s.push(c);
            }
        }

        let problems = get_problems(&s).expect("get_problems");
        if problems != criterion.nums {
            continue;
        }

        patterns.push(s);
    }

    patterns
}

fn make_combinations(len: usize) -> Vec<String> {
    let mut patterns = Vec::new();
    for i in 0..2usize.pow(len as u32) {
        let mut s = String::new();
        for j in 0..len {
            if i & (1 << j) != 0 {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        patterns.push(s);
    }
    patterns
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1
    "#};

    #[test]
    fn test_make_patterns_len() {
        let (_, data) = parse_input(INPUT).unwrap();

        assert_eq!(make_patterns(&data.criteria[0]).len(), 1);
        assert_eq!(make_patterns(&data.criteria[1]).len(), 4);
        assert_eq!(make_patterns(&data.criteria[2]).len(), 1);
        assert_eq!(make_patterns(&data.criteria[3]).len(), 1);
        assert_eq!(make_patterns(&data.criteria[4]).len(), 4);
        assert_eq!(make_patterns(&data.criteria[5]).len(), 10);
    }

    #[test]
    fn test_make_patterns() {
        let (_, data) = parse_input(INPUT).unwrap();

        let patterns = make_patterns(&data.criteria[0]);
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0], "#.#.###");
    }

    #[test]
    fn test_make_combinations() {
        let len = 2;
        let patterns = make_combinations(len);

        assert_eq!(patterns.len(), 4);
        assert_eq!(patterns, ["..", "#.", ".#", "##",]);
    }

    #[test]
    fn test_get_problems() {
        assert!(get_problems("???.###").is_err());
        assert_eq!(get_problems(".###.##.#...").unwrap(), [3, 2, 1]);
    }

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:#?}", data);

        assert_eq!(data.criteria.len(), 6);
        assert_eq!(data.criteria[0].springs, "???.###");
        assert_eq!(data.criteria[0].nums, [1, 1, 3]);

        assert_eq!(data.criteria[5].springs, "?###????????");
        assert_eq!(data.criteria[5].nums, [3, 2, 1]);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "21");
    }
}
