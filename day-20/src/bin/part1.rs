use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{BufReader, Read};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::{IResult, Parser};

use crate::Pulse::{High, Low};

#[derive(Debug)]
struct InputData<'a> {
    modules: Vec<Module<'a>>,
}

#[derive(Debug)]
enum Module<'a> {
    BrodCaster(Vec<&'a str>),
    FlipFlop(&'a str, Vec<&'a str>),
    Conjunction(&'a str, Vec<&'a str>),
}

#[derive(Debug)]
struct FlpFlopState<'a> {
    label: &'a str,
    switch: bool,
}

#[derive(Debug)]
struct ConJunctionState<'a> {
    label: &'a str,
    memories: HashMap<&'a str, Pulse>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Pulse {
    Low,
    High,
}

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-20/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let modules: HashMap<_, _> = data
        .modules
        .iter()
        .map(|m| match *m {
            Module::BrodCaster(_) => ("broadcaster", m),
            Module::FlipFlop(label, _) => (label, m),
            Module::Conjunction(label, _) => (label, m),
        })
        .collect();

    let mut flip_flop_states: HashMap<_, _> = data
        .modules
        .iter()
        .filter_map(|m| match *m {
            Module::FlipFlop(label, _) => Some((
                label,
                FlpFlopState {
                    label,
                    switch: false,
                },
            )),
            _ => None,
        })
        .collect();

    let mut conjunction_states: HashMap<_, _> = data
        .modules
        .iter()
        .filter_map(|m| match *m {
            Module::Conjunction(label, _) => {
                let memories = data
                    .modules
                    .iter()
                    .filter_map(|mm| match mm {
                        Module::BrodCaster(destinations) if destinations.contains(&label) => {
                            Some("broadcaster")
                        }
                        Module::FlipFlop(from_label, destinations)
                            if destinations.contains(&label) =>
                        {
                            Some(*from_label)
                        }
                        Module::Conjunction(from_label, destinations)
                            if destinations.contains(&label) =>
                        {
                            Some(*from_label)
                        }
                        _ => None,
                    })
                    .map(|label| (label, Low))
                    .collect();

                Some((label, ConJunctionState { label, memories }))
            }
            _ => None,
        })
        .collect();

    let mut count = HashMap::new();

    for _ in 0..1000 {
        cycle(
            &mut count,
            &modules,
            &mut flip_flop_states,
            &mut conjunction_states,
        );
    }

    println!("{:?}", count);
    let answer = count.values().product::<i32>();

    Ok(format!("{}", answer))
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (input, modules) = separated_list1(
        line_ending,
        alt((
            preceded(tag("broadcaster -> "), separated_list1(tag(", "), alpha1))
                .map(Module::BrodCaster),
            separated_pair(
                preceded(tag("%"), alpha1),
                tag(" -> "),
                separated_list1(tag(", "), alpha1),
            )
            .map(|(label, destinations)| Module::FlipFlop(label, destinations)),
            separated_pair(
                preceded(tag("&"), alpha1),
                tag(" -> "),
                separated_list1(tag(", "), alpha1),
            )
            .map(|(label, destinations)| Module::Conjunction(label, destinations)),
        )),
    )(input)?;

    Ok((input, InputData { modules }))
}

impl<'a> FlpFlopState<'a> {
    fn process(
        &mut self,
        pulse: Pulse,
        destinations: &[&'a str],
        queues: &mut VecDeque<(&'a str, Pulse, &'a str)>,
    ) {
        match (self.switch, pulse) {
            (false, Low) => {
                self.switch = true;
                for next in destinations {
                    queues.push_back((self.label, High, next));
                }
            }
            (true, Low) => {
                self.switch = false;
                for next in destinations {
                    queues.push_back((self.label, Low, next));
                }
            }
            (_, High) => (),
        }
    }
}

impl<'a> ConJunctionState<'a> {
    fn process(
        &mut self,
        from: &'a str,
        pulse: Pulse,
        destinations: &[&'a str],
        queues: &mut VecDeque<(&'a str, Pulse, &'a str)>,
    ) {
        self.memories.entry(from).and_modify(|m| *m = pulse);

        let send_pulse = if self.memories.values().all(|&p| p == High) {
            Low
        } else {
            High
        };

        for next in destinations {
            queues.push_back((self.label, send_pulse, next));
        }
    }
}

fn cycle<'a>(
    counts: &mut HashMap<Pulse, i32>,
    modules: &HashMap<&str, &Module<'a>>,
    flip_flop_states: &mut HashMap<&'a str, FlpFlopState<'a>>,
    conjunction_states: &mut HashMap<&'a str, ConJunctionState<'a>>,
) {
    let mut queues = VecDeque::new();
    queues.push_back(("button", Low, "broadcaster"));

    while let Some((from, pulse, queue)) = queues.pop_front() {
        println!("{} -{:?}-> {:?}", from, pulse, queue);
        counts.entry(pulse).and_modify(|c| *c += 1).or_insert(1);

        let Some(module) = modules.get(queue) else {
            continue;
        };

        match module {
            Module::BrodCaster(destinations) => {
                for next in destinations {
                    queues.push_back(("broadcaster", pulse, next));
                }
            }
            Module::FlipFlop(label, destinations) => {
                let state = flip_flop_states.get_mut(label).unwrap();
                state.process(pulse, destinations, &mut queues);
            }
            Module::Conjunction(label, destinations) => {
                let state = conjunction_states.get_mut(label).unwrap();
                state.process(from, pulse, destinations, &mut queues);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};
    use std::iter;

    use indoc::indoc;

    use crate::Pulse::Low;

    use super::*;

    const INPUT: &str = indoc! {r#"
    broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output
    "#};

    #[test]
    fn test_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:?}", data);
        assert_eq!(data.modules.len(), 5);
    }

    #[test]
    fn test_flip_flop() {
        let mut state = FlpFlopState {
            label: "broadcaster",
            switch: false,
        };
        let mut queues = VecDeque::new();

        state.process(High, &["a"], &mut queues);
        println!("step1 {:?} {:?}", state, queues);
        assert!(!state.switch);
        assert_eq!(queues, vec![]);

        state.process(Low, &["a"], &mut queues);
        println!("step2 {:?} {:?}", state, queues);
        assert!(state.switch);
        assert_eq!(queues, vec![("broadcaster", High, "a")]);

        state.process(High, &["a"], &mut queues);
        println!("step3 {:?} {:?}", state, queues);
        assert!(state.switch);
        assert_eq!(queues, vec![("broadcaster", High, "a")]);

        state.process(Low, &["a"], &mut queues);
        println!("step4 {:?} {:?}", state, queues);
        assert!(!state.switch);
        assert_eq!(
            queues,
            vec![("broadcaster", High, "a"), ("broadcaster", Low, "a")]
        );
    }

    #[test]
    fn test_conjunction() {
        let mut state = ConJunctionState {
            label: "inv",
            memories: HashMap::from_iter(iter::once(("broadcaster", Low))),
        };
        let mut queues = VecDeque::new();

        state.process("broadcaster", Low, &["a"], &mut queues);
        println!("step1 {:?} {:?}", state, queues);
        assert_eq!(state.memories.get("broadcaster"), Some(&Low));
        assert_eq!(queues, vec![("inv", High, "a")]);

        state.process("broadcaster", High, &["a"], &mut queues);
        println!("step2 {:?} {:?}", state, queues);
        assert_eq!(state.memories.get("broadcaster"), Some(&High));
        assert_eq!(queues, vec![("inv", High, "a"), ("inv", Low, "a")]);

        state.process("broadcaster", Low, &["a"], &mut queues);
        println!("step3 {:?} {:?}", state, queues);
        assert_eq!(state.memories.get("broadcaster"), Some(&Low));
        assert_eq!(
            queues,
            vec![("inv", High, "a"), ("inv", Low, "a"), ("inv", High, "a")]
        );

        state.process("broadcaster", Low, &["a"], &mut queues);
        println!("step4 {:?} {:?}", state, queues);
        assert_eq!(state.memories.get("broadcaster"), Some(&Low));
        assert_eq!(
            queues,
            vec![
                ("inv", High, "a"),
                ("inv", Low, "a"),
                ("inv", High, "a"),
                ("inv", High, "a")
            ]
        );
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "11687500");
    }
}
