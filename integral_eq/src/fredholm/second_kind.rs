use crate::FredholmSecondKind;
use common::{function::*, table_function::TableFunction};
use std::fmt::Debug;

use super::{conjugate_gradients::*, Error};

#[derive(Debug, Clone, PartialEq)]
pub struct FredholmSecondKindSystemOfEquations {
    eps: f64,
    n: usize,
    max_iter_count: usize,
}

impl FredholmSecondKindSystemOfEquations {
    pub fn new(eps: f64, n: usize, max_iter_count: usize) -> Self {
        Self {
            eps,
            n,
            max_iter_count,
        }
    }
}

impl FredholmSecondKind for FredholmSecondKindSystemOfEquations {
    type MethodError = Error;
    type ReturnFunction = TableFunction;

    fn solve<E1, E2>(
        &self,
        kernel: &dyn Function2d<Error = E2>,
        right_side: &dyn Function<Error = E1>,
        from: f64,
        to: f64,
        lambda: f64,
    ) -> Result<Self::ReturnFunction, Self::MethodError>
    where
        E1: Debug,
        E2: Debug,
    {
        let step = (to - from) / (self.n as f64 - 1.0);

        let mut mat = (0..self.n * self.n).map(|_| 0.0).collect::<Vec<_>>();
        let mut mat_transpozed = (0..self.n * self.n).map(|_| 0.0).collect::<Vec<_>>();
        let mut identity = (0..self.n * self.n).map(|_| 0.0).collect::<Vec<_>>();

        for i in 0..self.n {
            for j in 0..self.n {
                let x = (i as f64) * step + from;
                let y = (j as f64) * step + from;

                mat[i * self.n + j] = -lambda
                    * kernel
                        .apply(x, y)
                        .map(|res| res * step)
                        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;
                mat_transpozed[j * self.n + i] = mat[i * self.n + j];
            }
            identity[i * self.n + i] = 1.0;
            mat[i * self.n + i] += 1.0;
            mat_transpozed[i * self.n + i] += 1.0;
        }

        let mut a = (0..self.n * self.n).map(|_| 0.0).collect::<Vec<_>>();
        let mut f = (0..self.n * self.n).map(|_| 0.0).collect::<Vec<_>>();

        mult_mat(&mat_transpozed, &mat, &mut a, self.n);
        apply(
            &mat_transpozed,
            (0..self.n)
                .map(|i| right_side.apply((i as f64) * step + from))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| Error::FunctionError(format!("{:?}", e)))?
                .as_ref(),
            &mut f,
            self.n,
        );

        let mut res = (0..self.n).map(|_| 0.0).collect::<Vec<_>>();
        conjugate_gradient_method(
            &a,
            &identity,
            &mut res,
            &f,
            self.n,
            self.eps,
            self.max_iter_count,
        );

        Ok(TableFunction::from_table(
            res.iter()
                .enumerate()
                .map(|(i, y)| ((i as f64) * step + from, *y))
                .collect(),
        ))
    }
}

#[test]
fn fredholm_2nd() -> Result<(), Error> {
    #[derive(Debug, Clone, PartialEq)]
    enum DummyError {}

    let kernel = |x: f64, y: f64| -> Result<f64, DummyError> { Ok(x - y) };
    let right_side = |x: f64| -> Result<f64, DummyError> { Ok(3.0 - 2.0 * x) };
    let from = 0.0;
    let to = 1.0;
    let n = 100;

    let solver = FredholmSecondKindSystemOfEquations::new(0.000000001, n, 10000);
    let res = solver
        .solve(&kernel, &right_side, from, to, 1.0)?
        .sample(from, to, n)
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;

    let eps = 0.05;
    dbg!(&res);
    assert!(res[1..res.len() - 1]
        .iter()
        .map(|(_, y)| (y - 2.0).abs())
        .all(|diff| diff < eps));

    Ok(())
}
