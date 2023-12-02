use std::cmp::max;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

use day_02::GameInfo;
use day_02::{parse_game, Cube};

fn main() -> anyhow::Result<()> {
    let r = BufReader::new(fs::File::open("day-02/data/input.txt")?);

    let mut sum = 0;
    for line in r.lines() {
        let line = line?;
        let (_, game_info) =
            parse_game(&line).map_err(|e| anyhow::anyhow!("Failed to parse line: {}", e))?;
        sum += power(&game_info);
    }

    println!("answer: {}", sum);
    Ok(())
}

fn power(game_info: &GameInfo) -> u32 {
    let mut map: HashMap<Cube, u32> = HashMap::new();

    for set in &game_info.sets {
        for (num, cube_color) in set {
            map.entry(*cube_color)
                .and_modify(|e| *e = max(*e, *num))
                .or_insert(*num);
        }
    }

    map.into_values()
        .reduce(|acc, x| acc * x)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_power() {
        let input = indoc! { r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#};

        let powers = input
            .lines()
            .map(|line| parse_game(line).unwrap().1)
            .map(|game_info| power(&game_info))
            .collect::<Vec<_>>();

        assert_eq!(powers, vec![48, 12, 1560, 630, 36]);
        assert_eq!(powers.iter().sum::<u32>(), 2286);
    }
}
