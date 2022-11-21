use std::fmt::Debug;

use crate::{MinFinder1d, Minimum};

#[derive(Debug, Clone, PartialEq)]
pub struct GoldenRatioMinFinder {
    max_iter_count: usize,
    min_width: f64,
}

impl GoldenRatioMinFinder {
    pub fn new(max_iter_count: usize, min_width: f64) -> Self {
        Self {
            max_iter_count,
            min_width,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FunctionError(String),
}

impl MinFinder1d for GoldenRatioMinFinder {
    type Error = Error;

    fn find_min<E>(
        &self,
        from: f64,
        to: f64,
        func: &dyn common::function::Function<Error = E>,
    ) -> Result<Minimum, Self::Error>
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
        let mut iter_count = 0;

        loop {
            if (a - b).abs() < self.min_width || iter_count >= self.max_iter_count {
                return Ok(Minimum { x: a, y: f_a });
            }
            iter_count += 1;

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
    }
}

#[test]
fn find_min() -> Result<(), Error> {
    #[derive(Debug, Clone, PartialEq)]
    pub enum DummyError {}

    let f = |x: f64| -> Result<f64, DummyError> {
        Ok((x * x - 6.0 * x + 12.0) / (x * x + 6.0 * x + 20.0))
    };
    let a = 0.0;
    let b = 20.0;
    let eps = 0.001;
    let max_iter = 10000;

    let min_finder = GoldenRatioMinFinder::new(max_iter, eps);
    let min = min_finder.find_min(a, b, &f)?;

    let actual_min_x = 3.389;
    assert!((min.x - actual_min_x).abs() < 0.01);

    Ok(())
}
