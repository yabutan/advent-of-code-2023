use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{newline, space0, space1};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct Record {
    time: i64,
    distance: i64,
}

impl Record {
    pub fn count_win_ways(&self) -> usize {
        (1..self.time)
            .map(|press_time| calc_distance(press_time, self.time))
            .filter(|&distance| distance > self.distance)
            .count()
    }
}

fn calc_distance(press_time: i64, time: i64) -> i64 {
    press_time * (time - press_time)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Record>> {
    let (input, _) = tuple((tag("Time:"), space0))(input)?;
    let (input, times) = separated_list1(space1, complete::i64)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tuple((tag("Distance:"), space0))(input)?;
    let (input, distances) = separated_list1(space1, complete::i64)(input)?;

    let records = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Record { time, distance })
        .collect();

    Ok((input, records))
}

pub fn merge_records(records: &[Record]) -> Record {
    let time = records
        .iter()
        .map(|o| o.time)
        .join("")
        .parse::<i64>()
        .expect("should be number");

    let distance = records
        .iter()
        .map(|o| o.distance)
        .join("")
        .parse::<i64>()
        .expect("should be number");

    Record { time, distance }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    Time:      7  15   30
    Distance:  9  40  200
    "#};

    #[test]
    fn test_calc_distance() {
        assert_eq!(calc_distance(0, 7), 0);
        assert_eq!(calc_distance(1, 7), 6);
        assert_eq!(calc_distance(2, 7), 10);
        assert_eq!(calc_distance(3, 7), 12);
        assert_eq!(calc_distance(4, 7), 12);
        assert_eq!(calc_distance(5, 7), 10);
        assert_eq!(calc_distance(6, 7), 6);
        assert_eq!(calc_distance(7, 7), 0);
    }

    #[test]
    fn test_count_win_ways() {
        let record = Record {
            time: 7,
            distance: 9,
        };
        assert_eq!(record.count_win_ways(), 4);

        let record = Record {
            time: 15,
            distance: 40,
        };
        assert_eq!(record.count_win_ways(), 8);

        let record = Record {
            time: 30,
            distance: 200,
        };
        assert_eq!(record.count_win_ways(), 9);
    }

    #[test]
    fn test_merge_records() {
        let (_, records) = parse_input(INPUT).unwrap();
        let record = merge_records(&records);
        assert_eq!(record.time, 71530);
        assert_eq!(record.distance, 940200);
    }
}
