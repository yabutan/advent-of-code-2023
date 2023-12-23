use std::collections::HashMap;

pub use crate::parser::parse_input;

/// (category, op, value)
pub type Condition<'a> = (&'a str, &'a str, Int);

pub type Int = i32;

#[derive(Debug)]
pub struct InputData<'a> {
    pub workflows: Vec<Workflow<'a>>,
    pub parts: Vec<Part<'a>>,
}

#[derive(Debug)]
pub enum Operation<'a> {
    /// Then((category, op, value), next),
    Then(Condition<'a>, &'a str),

    /// Else(next),
    Else(&'a str),
}

#[derive(Debug)]
pub struct Workflow<'a> {
    pub label: &'a str,
    pub operations: Vec<Operation<'a>>,
}

#[derive(Debug)]
pub struct Part<'a> {
    pub ratings: HashMap<&'a str, Int>,
}

/// 承認される条件を抽出する。
/// Find conditions to accept the parts
pub fn find_conditions<'a>(data: &'a InputData) -> Vec<Vec<Condition<'a>>> {
    let workflows: HashMap<_, _> = data.workflows.iter().map(|o| (o.label, o)).collect();

    let mut conditions = Vec::new();
    make_conditions(&workflows, "in", &mut conditions, &[]);

    conditions
}

fn make_conditions<'a>(
    workflows: &HashMap<&str, &'a Workflow>,
    current: &str,
    accepted: &mut Vec<Vec<Condition<'a>>>,
    conditions: &[Condition<'a>],
) {
    match current {
        "A" => accepted.push(conditions.to_vec()),
        "R" => (),
        _ => {
            let mut conditions = conditions.to_vec();
            for ope in &workflows[current].operations {
                match *ope {
                    Operation::Then((category, op, value), next) => {
                        // nextに進んだ場合
                        {
                            let mut conditions = conditions.clone();
                            conditions.push((category, op, value));
                            make_conditions(workflows, next, accepted, &conditions);
                        }

                        // nextに進まなかった場合、否定条件に変換して次に続ける。
                        conditions.push(match op {
                            "<" => (category, ">", value - 1),
                            ">" => (category, "<", value + 1),
                            _ => unreachable!(),
                        });
                    }
                    Operation::Else(next) => {
                        make_conditions(workflows, next, accepted, &conditions);
                        return;
                    }
                }
            }
        }
    }
}

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::{is_a, tag};
    use nom::character::complete;
    use nom::character::complete::{alpha1, line_ending};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, separated_pair, tuple};
    use nom::{IResult, Parser};

    use super::*;

    pub fn parse_input(input: &str) -> IResult<&str, InputData> {
        let (input, workflows) = separated_list1(line_ending, workflow)(input)?;
        let (input, _) = many1(line_ending)(input)?;
        let (input, parts) = separated_list1(line_ending, part)(input)?;
        Ok((input, InputData { workflows, parts }))
    }

    fn operation(input: &str) -> IResult<&str, Operation> {
        alt((
            tuple((is_a("xmas"), is_a("<>"), complete::i32, tag(":"), alpha1))
                .map(|(category, op, value, _, next)| Operation::Then((category, op, value), next)),
            alpha1.map(Operation::Else),
        ))(input)
    }
    fn workflow(input: &str) -> IResult<&str, Workflow> {
        let (input, label) = alpha1(input)?;
        let (input, operations) =
            delimited(tag("{"), separated_list1(tag(","), operation), tag("}"))(input)?;

        Ok((input, Workflow { label, operations }))
    }

    fn part(input: &str) -> IResult<&str, Part> {
        let (input, values) = delimited(
            tag("{"),
            separated_list1(
                tag(","),
                separated_pair(is_a("xmas"), tag("="), complete::i32),
            ),
            tag("}"),
        )(input)?;

        let values = values.into_iter().collect();
        Ok((input, Part { ratings: values }))
    }
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
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:#?}", data);
    }

    #[test]
    fn test_find_conditions() {
        let (_, data) = parse_input(INPUT).unwrap();

        let conditions = find_conditions(&data);
        for x in &conditions {
            println!("{:?}", x);
        }

        assert_eq!(conditions.len(), 9);
    }
}
