use std::fmt::Debug;

use crate::functions::function::Function;

use super::{golden_ratio_min::golden_ratio_min, Minimum1d};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FunctionError(String),
    ItersEnded(Minimum1d, f64),
}

pub fn penalty_min<E>(
    f: &dyn Function<Error = E>,
    constraints: &[&dyn Function<Error = E>],
    from: f64,
    to: f64,
    start_eps: f64,
    min_step: f64,
    max_iter_count: usize,
) -> Result<Minimum1d, Error>
where
    E: Debug,
{
    let mut eps = start_eps;

    let mut prev_min = from;
    let mut prev_prev_min = 0.0;
    for _ in 0..max_iter_count {
        let penalty_func = |x| {
            constraints
                .iter()
                .map(|c| c.apply(x).map(|cx| f64::max(0.0, cx)))
                .map(|m| m.map(|m| m * m))
                .try_fold(0.0, |acc, m| m.map(|m| m + acc))
                .and_then(|sum| f.apply(x).map(|y| y + sum / eps))
        };
        let min = golden_ratio_min(from, to, &penalty_func, min_step, max_iter_count)
            .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;
        if (prev_min - min.x).abs() < min_step {
            return Ok(Minimum1d {
                x: min.x,
                y: f.apply(min.x)
                    .map_err(|e| Error::FunctionError(format!("{:?}", e)))?,
            });
        }
        eps *= 0.5;
        prev_prev_min = prev_min;
        prev_min = min.x;
    }

    Err(Error::ItersEnded(
        Minimum1d {
            x: prev_min,
            y: f.apply(prev_min)
                .map_err(|e| Error::FunctionError(format!("{:?}", e)))?,
        },
        (prev_min - prev_prev_min).abs(),
    ))
}

#[test]
fn penaty() -> Result<(), Error> {
    let f = |x: f64| -> Result<f64, Error> {
        Ok(-3.0 * x * x * x * x - x * x * x + 4.0 * x * x + 2.0 * x - 1.0)
    };

    let c1 = |x: f64| -> Result<f64, Error> { Ok(x * x - 1.0) };
    let c2 = |x: f64| -> Result<f64, Error> { Ok(-(10.0 * x).sin() - 0.5) };
    let from = -10.0;
    let to = 10.0;

    let res = penalty_min(&f, &[&c1, &c2], from, to, 0.001, 0.001, 1001)?;
    let actual = -0.262;
    dbg!(&res);
    assert!((res.x - actual).abs() < 0.01);

    Ok(())
}
