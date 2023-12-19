use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use day_19::{find_conditions, parse_input};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-19/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let range_map = HashMap::from([
        ("x", 1..=4000),
        ("m", 1..=4000),
        ("a", 1..=4000),
        ("s", 1..=4000),
    ]);

    let mut combinations = 0;
    for cs in &find_conditions(&data) {
        let mut map = range_map.clone();

        for &(category, op, value) in cs {
            let range = map.get_mut(category).unwrap();
            match op {
                "<" => {
                    *range = (*range.start())..=(value - 1).min(*range.end());
                }
                ">" => {
                    *range = (value + 1).max(*range.start())..=(*range.end());
                }
                _ => unreachable!(),
            }
        }

        combinations += map
            .values()
            .map(|r| (r.end() - r.start() + 1) as i64)
            .product::<i64>();
    }

    Ok(format!("{}", combinations))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}

    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "167409079868000");
    }
}
