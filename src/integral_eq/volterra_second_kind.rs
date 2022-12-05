use crate::common::{function::*, table_function::TableFunction};
use std::fmt::Debug;

use super::Error;

pub fn volterra_2nd_system<E1, E2>(
    kernel: &dyn Function2d<Error = E1>,
    right_side: &dyn Function<Error = E2>,
    from: f64,
    to: f64,
    lambda: f64,
    n: usize,
) -> Result<TableFunction, Error>
where
    E1: Debug,
    E2: Debug,
{
    let step = (to - from) / (n as f64 - 1.0);
    let mut y: Vec<(f64, f64)> = (0..n)
        .map(|i| (i as f64) * step + from)
        .map(|x| (x, 0.0))
        .collect();

    y[0].1 = right_side
        .apply(from)
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;

    for i in 1..n {
        let div = 1.0
            - lambda
                * kernel
                    .apply(from + step * (i as f64), from + step * (i as f64))
                    .map_err(|e| Error::FunctionError(format!("{:?}", e)))?
                * step
                * 0.5;
        let sum = 0.5
            * kernel
                .apply(from + step * (i as f64), from)
                .map_err(|e| Error::FunctionError(format!("{:?}", e)))?
            * step
            * lambda
            + step
                * (1..i).try_fold(0.0, |acc, j| -> Result<f64, Error> {
                    Ok(kernel
                        .apply(from + step * (i as f64), from + step * (j as f64))
                        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?
                        * y[j].1
                        + acc)
                })?;

        y[i].1 = (right_side
            .apply(from + step * (i as f64))
            .map_err(|e| Error::FunctionError(format!("{:?}", e)))?
            + lambda * sum)
            / div;
    }

    Ok(TableFunction::from_table(y))
}

#[test]
fn wolterra_2nd() -> Result<(), Error> {
    #[derive(Debug, Clone, PartialEq)]
    enum DummyError {}
    let k = |x: f64, s: f64| -> Result<f64, DummyError> { Ok((x - s).exp()) };
    let f = 1.0;

    let from = 0.0;
    let to = 1.0;
    let lambda = 1.0;
    let n = 50;
    let res = volterra_2nd_system(&k, &f, from, to, lambda, n)?;

    let eps = 0.001;
    let res_pts = res.sample(from, to, n)?;

    let actual = |x: f64| 0.5 * ((2.0 * x).exp() + 1.0);

    assert!(res_pts[1..res_pts.len() - 1]
        .iter()
        .map(|(x, y)| (y - actual(*x)).abs())
        .all(|diff| diff < eps));

    Ok(())
}
