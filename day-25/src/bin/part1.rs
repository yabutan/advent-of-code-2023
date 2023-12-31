use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufReader, Read, Write};

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;
use petgraph::prelude::*;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-25/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let connections = make_connections(&data);

    let labels: HashSet<_> = connections
        .iter()
        .flat_map(|&(left, right)| [left, right])
        .collect();

    let mut node_map = HashMap::new();
    let mut g: UnGraph<&str, ()> = Graph::new_undirected();
    for &x in &labels {
        node_map.insert(x, g.add_node(x));
    }

    for (left, right) in &connections {
        g.add_edge(
            *node_map.get(left).unwrap(),
            *node_map.get(right).unwrap(),
            (),
        );
    }

    // {
    //     let mut w = BufWriter::new(File::create("day-25/out/graph.dot")?);
    //     w.write_all(format!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel])).as_bytes())?;
    // }

    let min_cut_res: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&g, |_| Ok(1));
    println!("min_cut_res: {:?}", min_cut_res);

    let (cut_num, nodes) = min_cut_res
        .map_err(|e| anyhow::anyhow!("{:?}", e))?
        .expect("min cut should be found");

    assert_eq!(cut_num, 3);

    let group_a = nodes.len();
    let group_b = labels.len() - nodes.len();

    let answer = group_a * group_b;

    Ok(format!("{}", answer))
}

struct InputData<'a> {
    lines: Vec<(&'a str, Vec<&'a str>)>,
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    let parse_line = |input| {
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, components) = separated_list1(space1, alpha1)(input)?;
        Ok((input, (name, components)))
    };

    let (input, lines) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, InputData { lines }))
}

fn make_connections<'a>(data: &'a InputData) -> HashSet<(&'a str, &'a str)> {
    let mut connections = HashSet::new();
    for (left, components) in &data.lines {
        for right in components {
            connections.insert((*left.min(right), *left.max(right)));
        }
    }
    connections
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr
    "#};

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        for x in &data.lines {
            println!("{:?}", x);
        }

        let connections = make_connections(&data);
        println!("flowchart LR");
        for (left, right) in &connections {
            println!("    {} --- {}", left, right);
        }
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "54");
    }
}
