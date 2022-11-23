use std::{cell::RefCell, fmt::Debug};

use common::function::{Function, FunctionNd};

use crate::{golden_ratio_min::golden_ratio_min, MinimumNd};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FunctionError(String),
    SizeMismatch,
    ItersEnded(MinimumNd, f64),
}

pub fn gradients_min<E1, E2>(
    f: &dyn FunctionNd<Error = E1>,
    grad: &[&dyn FunctionNd<Error = E2>],
    x0: &[f64],
    eps: f64,
    max_iter_count: usize,
) -> Result<MinimumNd, Error>
where
    E1: Debug,
    E2: Debug,
{
    let n = x0.len();
    if grad.len() != n {
        return Err(Error::SizeMismatch);
    }

    let mut h = (0..n)
        .map(|i| grad[i].apply(x0).map(|y| -y))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;
    let mut x = x0.to_owned();
    let mut x_plus_alpha_h = x0.to_owned();

    struct AlphaFunc<'a, 'b, 'c, 'd, E> {
        x_plus_alpha_h: RefCell<&'d mut [f64]>,
        x: &'a [f64],
        h: &'b [f64],
        f: &'c dyn FunctionNd<Error = E>,
    }

    impl<'a, 'b, 'c, 'd, E> Function for AlphaFunc<'a, 'b, 'c, 'd, E> {
        type Error = E;

        fn apply(&self, alpha: f64) -> Result<f64, Self::Error> {
            for i in 0..self.x.len() {
                self.x_plus_alpha_h.borrow_mut()[i] = self.x[i] + alpha * self.h[i];
            }
            self.f.apply(self.x_plus_alpha_h.borrow().as_ref())
        }
    }

    let mut step = 0.0;
    for _ in 0..max_iter_count {
        let norm_h: f64 = h.iter().map(|x| x * x).sum();
        let alpha_res = golden_ratio_min(
            0.0,
            1.0,
            &AlphaFunc {
                x_plus_alpha_h: RefCell::new(&mut x_plus_alpha_h),
                x: &x,
                h: &h,
                f,
            },
            eps,
            max_iter_count,
        )
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;

        let alpha = alpha_res.x;
        step = alpha * alpha * norm_h;
        if step < eps * eps {
            return Ok(MinimumNd {
                y: f.apply(&x_plus_alpha_h)
                    .map_err(|e| Error::FunctionError(format!("{:?}", e)))?,
                x: x_plus_alpha_h,
            });
        }

        x = x_plus_alpha_h.clone();
        (0..n)
            .try_for_each(|i| grad[i].apply(&x).map(|y| h[i] = -y))
            .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;
    }

    Err(Error::ItersEnded(
        MinimumNd {
            y: f.apply(&x_plus_alpha_h)
                .map_err(|e| Error::FunctionError(format!("{:?}", e)))?,
            x: x_plus_alpha_h,
        },
        step.sqrt(),
    ))
}

#[test]
fn gradients() -> Result<(), Error> {
    let f = |x: &[f64]| {
        if x.len() != 2 {
            Err(Error::SizeMismatch)
        } else {
            Ok(10.0 * (x[1] - x[0] * x[0]) * (x[1] - x[0] * x[0]) + (1.0 - x[0]) * (1.0 - x[0]))
        }
    };
    let grad1 = |x: &[f64]| {
        if x.len() != 2 {
            Err(Error::SizeMismatch)
        } else {
            Ok(-40.0 * x[0] * x[1] + 40.0 * x[0] * x[0] * x[0] - 2.0 + 2.0 * x[0])
        }
    };
    let grad2 = |x: &[f64]| {
        if x.len() != 2 {
            Err(Error::SizeMismatch)
        } else {
            Ok(20.0 * x[1] - 20.0 * x[0] * x[0])
        }
    };

    let x0 = [3.0, 3.0];
    let actual = [1.0, 1.0];
    let res = gradients_min(&f, &[&grad1, &grad2], &x0, 0.00001, 10000)?;

    assert!(
        res.x
            .iter()
            .zip(actual.iter())
            .map(|(a, b)| (a - b).abs())
            .map(|x| x * x)
            .fold(0.0, |acc, x| acc + x)
            < 0.001
    );

    Ok(())
}
