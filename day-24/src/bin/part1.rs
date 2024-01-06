use std::fs;
use std::io::{BufReader, Read};
use std::ops::RangeInclusive;

use day_24::{parse_input, Float, IVec3, Vec2};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-24/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input, &(200000000000000.0..=400000000000000.0))?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str, range: &RangeInclusive<Float>) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let mut count = 0;
    let lines: Vec<_> = data.hailstones.iter().map(|x| get_line(x, range)).collect();
    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines.iter().skip(i + 1) {
            if get_cross_point(line1, line2).is_some() {
                println!("line1={:?} line2={:?}", line1.stone, line2.stone);
                count += 1;
            }
        }
    }

    Ok(format!("{}", count))
}

fn get_t(x: Float, d: Float, xt: Float) -> Float {
    (xt - x) / d
}

fn get_x(x: Float, d: Float, t: Float) -> Float {
    x + d * t
}

#[derive(Debug, PartialEq)]
struct Line<'a> {
    stone: &'a (IVec3, IVec3),
    start: Vec2,
    end: Vec2,
    time_span: RangeInclusive<Float>,
}

fn get_line<'a>(stone: &'a (IVec3, IVec3), range: &RangeInclusive<Float>) -> Line<'a> {
    let (s, v) = stone;

    let get_range = |s, v| {
        let t1 = get_t(s, v, *range.start());
        let t2 = get_t(s, v, *range.end());
        t1.min(t2)..=t1.max(t2)
    };

    let x_range = get_range(s.x as Float, v.x as Float);
    let y_range = get_range(s.y as Float, v.y as Float);

    let r =
        x_range.start().max(*y_range.start()).max(0.)..=x_range.end().min(*y_range.end()).max(0.);

    let x1 = get_x(s.x as Float, v.x as Float, *r.start());
    let y1 = get_x(s.y as Float, v.y as Float, *r.start());
    let x2 = get_x(s.x as Float, v.x as Float, *r.end());
    let y2 = get_x(s.y as Float, v.y as Float, *r.end());

    Line {
        stone,
        start: Vec2::new(x1, y1),
        end: Vec2::new(x2, y2),
        time_span: r,
    }
}

fn get_cross_point(line1: &Line, line2: &Line) -> Option<Vec2> {
    let x1 = line1.start.x;
    let y1 = line1.start.y;
    let x2 = line1.end.x;
    let y2 = line1.end.y;
    let x3 = line2.start.x;
    let y3 = line2.start.y;
    let x4 = line2.end.x;
    let y4 = line2.end.y;

    let a = x1 * y2 - y1 * x2;
    let b = x3 * y4 - y3 * x4;
    let c = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    let px = (a * (x3 - x4) - (x1 - x2) * b) / c;
    let py = (a * (y3 - y4) - (y1 - y2) * b) / c;

    let (s, v) = &line1.stone;
    let t1 = get_t(s.x as Float, v.x as Float, px);

    let (s, v) = &line2.stone;
    let t2 = get_t(s.x as Float, v.x as Float, px);

    if line1.time_span.contains(&t1) && line2.time_span.contains(&t2) {
        Some(Vec2::new(px, py))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
    19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3
    "#};

    #[test]
    fn test_get_tx() {
        let (_, data) = parse_input(INPUT).unwrap();
        let (s, v) = &data.hailstones[0];

        let t = get_t(s.x as Float, v.x as Float, 7.);
        println!("t1={}", t);
        let x = get_x(s.x as Float, v.x as Float, t);
        println!("x1={}", x);

        let t = get_t(s.x as Float, v.x as Float, 27.);
        println!("t2={}", t);
        let x = get_x(s.x as Float, v.x as Float, t);
        println!("x2={}", x);
    }

    #[test]
    fn test_get_line() {
        let (_, data) = parse_input(INPUT).unwrap();

        let line1 = get_line(&data.hailstones[0], &(7.0..=27.0));
        let line2 = get_line(&data.hailstones[1], &(7.0..=27.0));
        println!("line1={:?}", line1);
        println!("line2={:?}", line2);

        assert_eq!(line1.start, Vec2::new(19.0, 13.0));
        assert_eq!(line1.end, Vec2::new(7.0, 19.0));

        assert_eq!(line2.start, Vec2::new(18.0, 19.0));
        assert_eq!(line2.end, Vec2::new(7.0, 8.0));
    }

    #[test]
    fn test_get_cross_point() {
        let (_, data) = parse_input(INPUT).unwrap();

        let range = 7.0..=27.0;

        assert_eq!(
            get_cross_point(
                &get_line(&data.hailstones[0], &range),
                &get_line(&data.hailstones[1], &range),
            ),
            Some(Vec2::new(14.333333333333334, 15.333333333333334))
        );

        assert_eq!(
            get_cross_point(
                &get_line(&data.hailstones[0], &range),
                &get_line(&data.hailstones[2], &range),
            ),
            Some(Vec2::new(11.666666666666666, 16.666666666666668))
        );

        assert_eq!(
            get_cross_point(
                &get_line(&data.hailstones[0], &range),
                &get_line(&data.hailstones[3], &range),
            ),
            None,
        );

        assert_eq!(
            get_cross_point(
                &get_line(&data.hailstones[0], &range),
                &get_line(&data.hailstones[4], &range),
            ),
            None
        );

        assert_eq!(
            get_cross_point(
                &get_line(&data.hailstones[2], &range),
                &get_line(&data.hailstones[4], &range),
            ),
            None
        );
    }

    #[test]
    fn test_line() {
        let (_, data) = parse_input(INPUT).unwrap();
        let (s, v) = &data.hailstones[0];

        let x1 = get_x(s.x as Float, v.x as Float, -6.);
        let y1 = get_x(s.y as Float, v.y as Float, -6.);

        let x2 = get_x(s.x as Float, v.x as Float, 6.);
        let y2 = get_x(s.y as Float, v.y as Float, 6.);

        println!("x1={}, y1={}", x1, y1);
        println!("x2={}, y2={}", x2, y2);
    }

    #[test]
    fn test_process() {
        let answer = process(INPUT, &(7.0..=27.0)).unwrap();
        assert_eq!(answer, "2");
    }
}
