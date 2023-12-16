use std::fs;
use std::io::{BufReader, Read};

use linked_hash_map::LinkedHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

type Int = i32;

#[derive(Debug)]
struct InputData<'a> {
    operations: Vec<Operation<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
enum Operation<'a> {
    Remove(&'a str),
    Install(&'a str, Int),
}

#[derive(Debug)]
struct BoxState {
    boxes: Vec<LinkedHashMap<String, Int>>,
}

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-15/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let mut state = BoxState::new();

    for ope in &data.operations {
        state.operate(ope)
    }
    let power = state.calc_power();

    Ok(power.to_string())
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, operations) = separated_list1(tag(","), parse_operation)(input)?;
    Ok((input, InputData { operations }))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(
            tuple((alpha1, tag("="), complete::i32)),
            |(name, _, value)| Operation::Install(name, value),
        ),
        map(tuple((alpha1, tag("-"))), |(name, _)| {
            Operation::Remove(name)
        }),
    ))(input)
}

fn to_hash(input: &str) -> Int {
    let mut value = 0;
    for c in input.chars() {
        value += c as Int;
        value *= 17;
        value %= 256;
    }

    value
}

impl BoxState {
    fn new() -> Self {
        let mut boxes = Vec::new();
        for _ in 0..256 {
            boxes.push(LinkedHashMap::new());
        }

        Self { boxes }
    }

    fn operate(&mut self, operation: &Operation) {
        match operation {
            Operation::Install(name, value) => {
                let i = to_hash(name);
                let inside = &mut self.boxes[i as usize];

                inside
                    .entry(name.to_string())
                    .and_modify(|v| *v = *value)
                    .or_insert(*value);
            }
            Operation::Remove(name) => {
                let i = to_hash(name);
                let inside = &mut self.boxes[i as usize];

                inside.remove(*name);
            }
        }
    }

    fn calc_power(&self) -> Int {
        let mut power = 0;
        for (i, inside) in self.boxes.iter().enumerate() {
            for (j, (_, focal_length)) in inside.iter().enumerate() {
                // power = box * slot * focal length
                power += (i as Int + 1) * (j as Int + 1) * focal_length;
            }
        }
        power
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "#};

    #[test]
    fn test_box_state() {
        let (_, data) = parse_input(INPUT).unwrap();
        let mut state = BoxState::new();

        for ope in &data.operations {
            state.operate(ope)
        }

        for (i, inside) in state.boxes.iter().enumerate() {
            println!("{} {:?}", i, inside)
        }

        assert_eq!(state.boxes[0]["rn"], 1);
        assert_eq!(state.boxes[0]["cm"], 2);
        assert_eq!(state.boxes[3]["ot"], 7);
        assert_eq!(state.boxes[3]["ab"], 5);
        assert_eq!(state.boxes[3]["pc"], 6);
    }

    #[test]
    fn test_hash() {
        assert_eq!(to_hash("HASH"), 52);

        assert_eq!(to_hash("rn"), 0);
        assert_eq!(to_hash("cm"), 0);

        assert_eq!(to_hash("qp"), 1);

        assert_eq!(to_hash("pc"), 3);
        assert_eq!(to_hash("ot"), 3);
        assert_eq!(to_hash("ab"), 3);
    }

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:?}", data);
        assert_eq!(data.operations.len(), 11);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "145");
    }
}
