use std::io::BufRead;

use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::IResult;

/// 空行ごとに合計した値のVecを取得する。
pub fn read_sum_list(mut r: impl BufRead) -> Result<Vec<u32>> {
    let mut buffer = String::new();
    r.read_to_string(&mut buffer)?;

    let list = match parse_input(&buffer) {
        Ok((_, v)) => v,
        Err(e) => return Err(anyhow::anyhow!("Error parsing input: {}", e.to_string())),
    };

    let sum_list: Vec<u32> = list.into_iter().map(|l| l.into_iter().sum()).collect();

    Ok(sum_list)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    // nomを使った、パース処理。
    // 空行区切りの中に、改行区切りの数字があるのを想定。
    separated_list1(tag("\n\n"), separated_list1(tag("\n"), complete::u32))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_get_max() {
        let r = include_str!("../data/sample.txt").as_bytes();
        let list = read_sum_list(r).unwrap();
        let max = list.iter().max();
        assert_eq!(max, Some(&24000));
    }

    #[test]
    fn test_get_sum_of_top_tree() {
        let r = include_str!("../data/sample.txt").as_bytes();

        let list = read_sum_list(r).unwrap();
        let sum_of_top_three: u32 = list.iter().sorted().rev().take(3).sum();
        assert_eq!(sum_of_top_three, 45000);
    }
}
