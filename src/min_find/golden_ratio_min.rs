use std::fmt::Debug;

use crate::functions::function::Function;

use super::Minimum1d;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FunctionError(String),
    ItersEnded(Minimum1d, f64),
}

pub fn golden_ratio_min<E>(
    from: f64,
    to: f64,
    func: &dyn Function<Error = E>,
    min_width: f64,
    max_iter_count: usize,
) -> Result<Minimum1d, Error>
where
    E: Debug,
{
    let a_coef = (3.0 - 5.0f64.sqrt()) * 0.5;
    let b_coef = (-1.0 + 5.0f64.sqrt()) * 0.5;

    let mut a = f64::min(from, to);
    let mut b = f64::max(from, to);
    let mut f_a = func
        .apply(a)
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;
    let mut f_b = func
        .apply(b)
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;

    for _ in 0..max_iter_count {
        if (a - b).abs() < min_width {
            return Ok(Minimum1d { x: a, y: f_a });
        }

        let x1 = a * a_coef + b * b_coef;
        let x2 = f64::max(a + b - x1, x1);
        let x1 = a + b - x2;

        let f_x1 = func
            .apply(x1)
            .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;
        let f_x2 = func
            .apply(x2)
            .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;

        if f_a < f_x1 && f_a < f_x2 && f_a < f_b {
            b = x1;
            f_b = f_x1;
        }
        if f_b < f_x1 && f_b < f_x2 && f_b < f_a {
            a = x2;
            f_a = f_x2;
        }
        if f_x1 < f_a && f_x1 < f_x2 && f_x1 < f_b {
            b = x2;
            f_b = f_x2;
        }
        if f_x2 < f_a && f_x2 < f_x1 && f_x2 < f_b {
            a = x1;
            f_a = f_x1;
        }
    }

    Err(Error::ItersEnded(Minimum1d { x: a, y: f_a }, (b - a).abs()))
}

#[test]
fn find_min() -> Result<(), Error> {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DummyError {}

    let f = |x: f64| -> Result<f64, DummyError> {
        Ok((x * x - 6.0 * x + 12.0) / (x * x + 6.0 * x + 20.0))
    };
    let a = 0.0;
    let b = 20.0;
    let eps = 0.001;
    let max_iter = 10000;

    let min = golden_ratio_min(a, b, &f, eps, max_iter)?;

    let actual_min_x = 3.389;
    assert!((min.x - actual_min_x).abs() < 0.01);

    Ok(())
}
