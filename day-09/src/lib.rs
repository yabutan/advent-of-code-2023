use complete::newline;
use nom::character::complete;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug)]
pub struct InputData {
    pub lines: Vec<Vec<i32>>,
}

pub fn get_sequences(line: &[i32]) -> Vec<Vec<i32>> {
    fn seq(line: &[i32]) -> Vec<i32> {
        let mut differs = Vec::new();
        for i in 1..line.len() {
            let (a, b) = (line[i - 1], line[i]);
            differs.push(b - a);
        }
        differs
    }
    fn all_zero(line: &[i32]) -> bool {
        line.iter().all(|&x| x == 0)
    }

    let mut sequences = Vec::new();
    sequences.push(line.to_vec());

    let mut current = line;
    loop {
        let next = seq(current);
        if all_zero(&next) {
            sequences.push(next);
            break;
        }
        sequences.push(next);
        current = &sequences.last().expect("should have last");
    }

    sequences
}

pub fn get_prediction(sequences: &[Vec<i32>]) -> i32 {
    sequences.iter().map(|seq| seq.last().unwrap_or(&0)).sum()
}

pub fn get_prev_prediction(sequences: &[Vec<i32>]) -> i32 {
    fn prev(i: usize, sequences: &[Vec<i32>]) -> i32 {
        if i >= sequences.len() {
            return 0;
        }
        sequences[i][0] - prev(i + 1, sequences)
    }

    prev(0, sequences)
}

pub fn parse_input(input: &str) -> IResult<&str, InputData> {
    fn line(input: &str) -> IResult<&str, Vec<i32>> {
        separated_list1(space1, complete::i32)(input)
    }

    let (input, lines) = separated_list1(newline, line)(input)?;
    Ok((input, InputData { lines }))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    "#};

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        println!("{:#?}", data);
        assert_eq!(data.lines.len(), 3);
    }

    #[test]
    fn test_get_prediction() {
        fn dump(sequences: &[Vec<i32>]) {
            for seq in sequences {
                println!("{:?}", seq);
            }
        }

        let (_, data) = parse_input(INPUT).unwrap();
        let sequences = get_sequences(&data.lines[0]);
        dump(&sequences);
        assert_eq!(get_prediction(&sequences), 18);

        let sequences = get_sequences(&data.lines[1]);
        dump(&sequences);
        assert_eq!(get_prediction(&sequences), 28);

        let sequences = get_sequences(&data.lines[2]);
        dump(&sequences);
        assert_eq!(get_prediction(&sequences), 68);
    }

    #[test]
    fn test_get_prev_prediction() {
        fn dump(sequences: &[Vec<i32>]) {
            for seq in sequences {
                println!("{:?}", seq);
            }
        }

        let (_, data) = parse_input(INPUT).unwrap();
        let sequences = get_sequences(&data.lines[0]);
        dump(&sequences);
        assert_eq!(get_prev_prediction(&sequences), -3);

        let sequences = get_sequences(&data.lines[1]);
        dump(&sequences);
        assert_eq!(get_prev_prediction(&sequences), 0);

        let sequences = get_sequences(&data.lines[2]);
        dump(&sequences);
        assert_eq!(get_prev_prediction(&sequences), 5);
    }
}
