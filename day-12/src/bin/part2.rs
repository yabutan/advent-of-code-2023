use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use complete::char;
use itertools::Itertools;
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

    let mut memo = HashMap::new();
    for (i, criterion) in data.criteria.iter().enumerate() {
        let criterion = unfold(criterion);

        print!("({}/{}) {:?}", i, data.criteria.len(), criterion);
        let len = arrangements(&criterion, &mut memo);
        println!(" -> {}", len);

        total += len;
    }

    Ok(total.to_string())
}

#[derive(Debug)]
struct InputData<'a> {
    criteria: Vec<Criterion<'a>>,
}

#[derive(Debug)]
struct Criterion<'a> {
    springs: Cow<'a, str>,
    nums: Vec<i32>,
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    fn criterion(input: &str) -> IResult<&str, Criterion> {
        let (input, springs) = take_until(" ")(input)?;
        let (input, _) = space1(input)?;
        let (input, nums) = separated_list1(tag(","), complete::i32)(input)?;
        Ok((
            input,
            Criterion {
                springs: Cow::from(springs),
                nums,
            },
        ))
    }

    let (input, criteria) = separated_list1(newline, criterion)(input)?;
    Ok((input, InputData { criteria }))
}

fn unfold<'a>(criterion: &Criterion) -> Criterion<'a> {
    let spring = (0..5).map(|_| criterion.springs.to_string()).join("?");

    let nums = criterion.nums.repeat(5);
    Criterion {
        springs: Cow::from(spring),
        nums,
    }
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

fn get_problems(memo: &mut HashMap<String, Vec<i32>>, input: &str) -> anyhow::Result<Vec<i32>> {
    if let Some(problems) = memo.get(input) {
        println!("use memoized: {}", input);
        return Ok(problems.clone());
    }

    let (_, spring) = parse_spring(input).expect("parse_spring");
    let mut problems = Vec::new();
    for s in &spring {
        match s {
            Sign::Dot(_) => {}
            Sign::Hash(n) => problems.push(*n as i32),
            Sign::Question(_) => return Err(anyhow::anyhow!("? is not allowed")),
        }
    }

    memo.insert(input.to_string(), problems.clone());
    Ok(problems)
}

// fn get_problems(input: &str) -> anyhow::Result<Vec<i32>> {
//     println!("get_problems: {}", input);
//
//     let (_, spring) = parse_spring(input).expect("parse_spring");
//     let mut problems = Vec::new();
//     for s in &spring {
//         match s {
//             Sign::Dot(_) => {}
//             Sign::Hash(n) => problems.push(*n as i32),
//             Sign::Question(_) => return Err(anyhow::anyhow!("? is not allowed")),
//         }
//     }
//     Ok(problems)
// }

fn make_patterns(criterion: &Criterion) -> Vec<String> {
    let question_count = criterion.springs.chars().filter(|c| c == &'?').count();

    let mut patterns = Vec::new();

    let combinations = make_combinations(question_count);
    println!("combinations: {}", combinations.len());

    let mut memo = HashMap::new();
    for mut arr in combinations {
        let mut s = String::new();
        for c in criterion.springs.chars() {
            if c == '?' {
                s.push(arr.pop().expect("pop"));
            } else {
                s.push(c);
            }
        }

        let problems = get_problems(&mut memo, &s).expect("get_problems");
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

fn count_char(input: &str, c: &[char]) -> usize {
    let mut count = 0;
    for x in input.chars() {
        if c.iter().any(|&c| c == x) {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn arrangements(criterion: &Criterion, memo: &mut HashMap<(String, Vec<i32>), usize>) -> usize {
    // remove duplicated dot
    let mut new_springs = String::new();
    let mut prev = ' ';
    for c in criterion.springs.trim_matches('.').chars() {
        if c == '.' && prev == '.' {
            continue;
        }
        new_springs.push(c);
        prev = c;
    }

    println!("springs {} -> {}", criterion.springs, new_springs);

    get_arrangements(&new_springs, &criterion.nums, memo)
}

fn get_arrangements(
    spring: &str,
    nums: &[i32],
    memo: &mut HashMap<(String, Vec<i32>), usize>,
) -> usize {
    let spring = spring.trim_start_matches('.');
    let key = (spring.to_string(), nums.to_vec());
    if let Some(&count) = memo.get(&key) {
        println!("use memoized: {} {:?}", spring, nums);
        return count;
    } else {
        println!("get_arrangements: {} {:?}", spring, nums);
    }

    if nums.is_empty() {
        return if spring.chars().any(|c| c == '#') {
            // もうないはずなのに、#があるなら成立していない。
            0
        } else {
            1
        };
    }

    let n = nums[0] as usize;
    let sharp_count = count_char(spring, &['#']);
    if sharp_count == n {
        let new_spring = &spring[n..];
        return if !new_spring.is_empty() && new_spring.starts_with('?') {
            get_arrangements(&new_spring[1..], &nums[1..], memo)
        } else {
            get_arrangements(new_spring, &nums[1..], memo)
        };
    } else if sharp_count > n {
        // #が多いなら、成立していない。
        return 0;
    } else {
        let new_spring = &spring[sharp_count..];
        if new_spring.is_empty() || !new_spring.starts_with('?') {
            // #が足りない見立てなら、成立しない。
            return 0;
        }
    }

    let mut count = 0;
    {
        let replaced = spring.replacen('?', "#", 1);
        if spring != replaced {
            count += get_arrangements(&replaced, nums, memo);
        }
    }
    {
        let replaced = spring.replacen('?', ".", 1);
        if spring != replaced {
            count += get_arrangements(&replaced, nums, memo);
        }
    }

    memo.insert(key, count);
    count
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
    fn test_get_arrangement() {
        let mut memo = HashMap::new();

        assert_eq!(get_arrangements("?", &[1], &mut memo), 1);
        assert_eq!(get_arrangements("#", &[1], &mut memo), 1);
        assert_eq!(get_arrangements(".", &[1], &mut memo), 0);
        assert_eq!(get_arrangements("..", &[1], &mut memo), 0);
        assert_eq!(get_arrangements("#.", &[1], &mut memo), 1);
        assert_eq!(get_arrangements(".#", &[1], &mut memo), 1);
        assert_eq!(get_arrangements("??", &[1], &mut memo), 2);
        assert_eq!(get_arrangements("?.?", &[1, 1], &mut memo), 1);
        assert_eq!(get_arrangements("#.?", &[1, 1], &mut memo), 1);
        assert_eq!(get_arrangements("?.#", &[1, 1], &mut memo), 1);
        assert_eq!(get_arrangements("???", &[1, 1], &mut memo), 1);
        assert_eq!(get_arrangements("???.###", &[1, 1, 3], &mut memo), 1);
    }

    #[test]
    fn test_make_patterns_test() {
        let (_, data) = parse_input(INPUT).unwrap();
        let criterion = &data.criteria[1];
        println!("{:?}", criterion);

        let patterns = make_patterns(&data.criteria[1]);
        for x in patterns {
            println!("{}", x);
        }
    }

    #[test]
    fn test_unfold() {
        let (_, data) = parse_input(INPUT).unwrap();

        let criterion = &data.criteria[0];
        let unfolded = unfold(criterion);

        assert_eq!(unfolded.springs, "???.###????.###????.###????.###????.###");
        assert_eq!(unfolded.nums, [1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]);
    }

    #[test]
    fn test_arrangements() {
        let (_, data) = parse_input(INPUT).unwrap();

        let mut memo = HashMap::new();
        assert_eq!(arrangements(&data.criteria[0], &mut memo), 1);
        assert_eq!(arrangements(&data.criteria[1], &mut memo), 4);
        assert_eq!(arrangements(&data.criteria[2], &mut memo), 1);
        assert_eq!(arrangements(&data.criteria[3], &mut memo), 1);
        assert_eq!(arrangements(&data.criteria[4], &mut memo), 4);
        assert_eq!(arrangements(&data.criteria[5], &mut memo), 10);
    }

    #[test]
    fn test_make_patterns_unfold_len() {
        let (_, data) = parse_input(INPUT).unwrap();
        let mut memo = HashMap::new();
        let criterion = unfold(&data.criteria[0]);
        assert_eq!(arrangements(&criterion, &mut memo), 1);

        let criterion = unfold(&data.criteria[3]);
        assert_eq!(arrangements(&criterion, &mut memo), 16);
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
        let mut memo = HashMap::new();
        assert!(get_problems(&mut memo, "???.###").is_err());
        assert_eq!(get_problems(&mut memo, ".###.##.#...").unwrap(), [3, 2, 1]);
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
        assert_eq!(answer, "525152");
    }
}
