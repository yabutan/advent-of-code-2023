use std::fs;
use std::io::{BufReader, Read};

use z3::ast::{Ast, Int, Real};
use z3::{Config, Context, SatResult, Solver};

use day_24::{parse_input, InputData};

fn main() -> anyhow::Result<()> {
    let mut r = BufReader::new(fs::File::open("day-24/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let answer = process(&input)?;
    println!("answer: {}", answer);

    Ok(())
}

fn process(input: &str) -> anyhow::Result<String> {
    let (_, data) = parse_input(input).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let v = solve(&data).expect("no answer");
    Ok(v)
}

fn solve(data: &InputData) -> Option<String> {
    // 3点だけ使って、方程式の条件を満たす結果を取得する。
    let stones = &data.hailstones[..3];

    let ctx = Context::new(&Config::new());
    let solver = Solver::new(&ctx);

    // 変数定義
    let zero = Int::from_i64(&ctx, 0).to_real();
    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");
    let z = Real::new_const(&ctx, "z");
    let dx = Real::new_const(&ctx, "dx");
    let dy = Real::new_const(&ctx, "dy");
    let dz = Real::new_const(&ctx, "dz");

    // 方程式の定義
    for (i, (p, pd)) in stones.iter().enumerate() {
        let [px, py, pz] = p.to_array().map(|v| Int::from_i64(&ctx, v).to_real());
        let [pdx, pdy, pdz] = pd.to_array().map(|v| Int::from_i64(&ctx, v).to_real());

        let t = Real::new_const(&ctx, format!("t{i}"));
        solver.assert(&t.ge(&zero));
        solver.assert(&((&px + &pdx * &t)._eq(&(&x + &dx * &t))));
        solver.assert(&((&py + &pdy * &t)._eq(&(&y + &dy * &t))));
        solver.assert(&((&pz + &pdz * &t)._eq(&(&z + &dz * &t))));
    }

    if solver.check() == SatResult::Sat {
        // 方程式を満たす変数結果を取得
        let model = solver.get_model().unwrap();
        // x + y + z
        let v = model.eval(&(&x + &y + &z), true).unwrap();

        Some(v.to_string().trim_end_matches(".0").to_string())
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
    fn test_process() {
        let answer = process(INPUT).unwrap();
        assert_eq!(answer, "47");
    }
}
