use std::borrow::Cow;
use std::collections::HashMap;

use itertools::Itertools;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::newline;
use nom::character::streaming::space1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug)]
pub struct InputData<'a> {
    pub criteria: Vec<Criterion<'a>>,
}

#[derive(Debug)]
pub struct Criterion<'a> {
    springs: Cow<'a, str>,
    nums: Vec<i32>,
}

pub fn parse_input(input: &str) -> IResult<&str, InputData> {
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

pub fn unfold<'a>(criterion: &Criterion) -> Criterion<'a> {
    let spring = (0..5).map(|_| criterion.springs.to_string()).join("?");

    let nums = criterion.nums.repeat(5);
    Criterion {
        springs: Cow::from(spring),
        nums,
    }
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

pub fn arrangements(criterion: &Criterion, memo: &mut HashMap<(String, Vec<i32>), usize>) -> usize {
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
    }

    if sharp_count > n {
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
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:#?}", data);

        assert_eq!(data.criteria.len(), 6);
        assert_eq!(data.criteria[0].springs, "???.###");
        assert_eq!(data.criteria[0].nums, [1, 1, 3]);

        assert_eq!(data.criteria[5].springs, "?###????????");
        assert_eq!(data.criteria[5].nums, [3, 2, 1]);
    }
}
