use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use day_19::{parse_input, Int, Operation};

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
    let workflows: HashMap<_, _> = data.workflows.iter().map(|o| (o.label, o)).collect();

    let mut acccepted = Vec::new();
    'part: for part in &data.parts {
        let mut current = "in";

        'workflow: loop {
            match current {
                "A" => {
                    acccepted.push(part);
                    continue 'part;
                }
                "R" => continue 'part,
                _ => {}
            }

            for ope in &workflows[current].operations {
                match *ope {
                    Operation::Then((category, op, value), next) => match op {
                        "<" => {
                            if part.ratings[category] < value {
                                current = next;
                                continue 'workflow;
                            }
                        }
                        ">" => {
                            if part.ratings[category] > value {
                                current = next;
                                continue 'workflow;
                            }
                        }
                        _ => unreachable!(),
                    },
                    Operation::Else(next) => {
                        current = next;
                        continue 'workflow;
                    }
                }
            }
        }
    }

    let total = acccepted
        .iter()
        .map(|p| p.ratings.values().sum::<Int>())
        .sum::<Int>();
    Ok(format!("{}", total))
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
        assert_eq!(answer, "19114");
    }
}
