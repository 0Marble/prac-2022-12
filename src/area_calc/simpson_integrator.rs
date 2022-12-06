use std::fmt::Debug;

use crate::function::function::Function;

use super::Error;

pub fn integrate_step<E>(
    f: &dyn Function<Error = E>,
    from: f64,
    to: f64,
    n: &mut usize,
    cached_pts: &mut Vec<f64>,
) -> Result<f64, Error>
where
    E: Debug,
{
    if cached_pts.len() < 3 {
        cached_pts.push(
            f.apply(from)
                .map_err(|e| Error::FunctionError(format!("{:?}", e)))?,
        );
        cached_pts.push(
            f.apply(to)
                .map_err(|e| Error::FunctionError(format!("{:?}", e)))?,
        );
        cached_pts.push(
            f.apply((from + to) / 2.0)
                .map_err(|e| Error::FunctionError(format!("{:?}", e)))?,
        );
        *n = 2;
        return Ok(
            (2.0 * cached_pts[0] + 2.0 * cached_pts[1] + 4.0 * cached_pts[2]) * (to - from) / 6.0,
        );
    }

    let step = (to - from) / (*n as f64);
    let sum = (0..*n)
        .map(|i| (i as f64) * step + from)
        .map(|x| {
            f.apply(x).map(|y| {
                cached_pts.push(y);
                y
            })
        })
        .try_fold(0.0, |acc, x| x.map(|x| x + acc))
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?
        * 4.0
        + (1..*n).map(|i| cached_pts[i]).sum::<f64>() * 2.0
        + cached_pts[0]
        + cached_pts[*n];

    *n *= 2;
    let new_step = (to - from) / (*n as f64);

    Ok(sum * new_step / 3.0)
}

#[test]
fn integrate() -> Result<(), Error> {
    let f = |x: f64| -> Result<f64, Error> { Ok(2.0f64.powf(-x)) };
    let mut computed_points = vec![];
    let mut n = 0;

    let mut prev_s = integrate_step(&f, 0.0, 1.0, &mut n, &mut computed_points)?;
    for _ in 0..1000 {
        let cur_s = integrate_step(&f, 0.0, 1.0, &mut n, &mut computed_points)?;
        if f64::abs(prev_s - cur_s) < 0.0001 {
            break;
        }
        prev_s = cur_s;
    }

    assert!((prev_s - 0.721347520444).abs() < 0.001);

    Ok(())
}
