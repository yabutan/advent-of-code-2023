use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use glam::IVec2;
use graph::NodeIndex;
use petgraph::algo::all_simple_paths;
use petgraph::graph;
use petgraph::graph::UnGraph;

use day_23::parse_input;

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-23/data/input.txt")?);
    //let mut r = BufReader::new(fs::File::open("day-23/data/input_example.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let data = parse_input(input);

    let make_edge =
        |g: &mut UnGraph<IVec2, ()>, indexes: &HashMap<IVec2, NodeIndex>, a: &IVec2, b: &IVec2| {
            match data.get(b) {
                None | Some("#") => {}
                _ => {
                    g.add_edge(indexes[a], indexes[b], ());
                }
            }
        };

    let mut indexes = HashMap::new();
    let mut g: UnGraph<IVec2, ()> = UnGraph::new_undirected();
    for y in 0..data.size.y {
        for x in 0..data.size.x {
            let current_pos = IVec2::new(x, y);
            if let Some("#") = data.get(&current_pos) {
                continue;
            }

            indexes.insert(IVec2::new(x, y), g.add_node(current_pos));
            make_edge(&mut g, &indexes, &current_pos, &IVec2::new(x - 1, y));
            make_edge(&mut g, &indexes, &current_pos, &IVec2::new(x, y - 1));
        }
    }

    // Output the graph to dot file
    {
        use petgraph::dot::{Config, Dot};
        use std::fs::File;
        use std::io::{BufWriter, Write};
        let mut w = BufWriter::new(File::create("day-23/out/graph.dot")?);
        w.write_all(format!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel])).as_bytes())?;
    }

    let path = all_simple_paths::<Vec<_>, _>(
        &g,
        indexes[&data.start_pos],
        indexes[&data.end_pos],
        0,
        None,
    )
    .max_by_key(|p| p.len())
    .expect("no path found");

    println!("longest path: {:?}", path);

    Ok(format!("{}", path.len() - 1))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    #.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#
    "#};

    #[test]
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "154");
    }
}
