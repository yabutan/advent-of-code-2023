use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

use day_02::game_possible;
use day_02::{parse_game, Cube};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-02/data/input.txt")?);

    let max_map: HashMap<Cube, u32> = [(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)]
        .into_iter()
        .collect();

    let sum = part1(&mut r, &max_map)?;
    println!("answer: {}", sum);
    Ok(())
}

fn part1(r: &mut impl BufRead, max_map: &HashMap<Cube, u32>) -> anyhow::Result<u32> {
    let mut total = 0;
    for line in r.lines() {
        let line = line?;

        let (_, game_info) =
            parse_game(&line).map_err(|e| anyhow::anyhow!("Failed to parse line: {}", e))?;

        if game_possible(&game_info, max_map) {
            total += game_info.number;
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use indoc::indoc;

    use super::*;

    #[test]
    fn test_par1_sample() -> anyhow::Result<()> {
        let input = indoc! { r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#};

        let only: HashMap<Cube, u32> = [(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)]
            .into_iter()
            .collect();

        let sum = part1(&mut input.as_bytes(), &only)?;
        assert_eq!(sum, 8);
        Ok(())
    }
}
