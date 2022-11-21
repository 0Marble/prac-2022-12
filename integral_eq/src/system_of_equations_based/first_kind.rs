use crate::FredholmFirstKind;
use common::table_function::{error::Error as TableFunctionError, TableFunction};
use std::fmt::Debug;

use super::{conjugate_gradients::*, Error};

#[derive(Debug, Clone, PartialEq)]
pub struct FredholmFirstKindSystemOfEquations {
    eps: f64,
    n: usize,
    max_iter_count: usize,
}

impl FredholmFirstKindSystemOfEquations {
    pub fn new(eps: f64, n: usize, max_iter_count: usize) -> Self {
        Self {
            eps,
            n: n + 1,
            max_iter_count,
        }
    }
}

impl FredholmFirstKind for FredholmFirstKindSystemOfEquations {
    type MethodError = Error;
    type ResultFunctionError = TableFunctionError;

    fn solve<E1, E2>(
        &self,
        kernel: &dyn common::function::Function2d<Error = E2>,
        right_side: &dyn common::function::Function<Error = E1>,
        from: f64,
        to: f64,
    ) -> Result<
        Box<dyn common::function::Function<Error = Self::ResultFunctionError>>,
        Self::MethodError,
    >
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

                mat[i * self.n + j] = kernel
                    .apply(x, y)
                    .map(|res| res * step)
                    .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;
                mat_transpozed[j * self.n + i] = mat[i * self.n + j];
            }
            identity[i * self.n + i] = 1.0;
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

        Ok(Box::new(TableFunction::from_table(
            res.iter()
                .enumerate()
                .map(|(i, y)| ((i as f64) * step + from, *y))
                .collect(),
        )))
    }
}

#[test]
fn fredholm_1st() -> Result<(), Error> {
    #[derive(Debug, Clone, PartialEq)]
    enum DummyError {}

    let kernel = |x: f64, y: f64| -> Result<f64, DummyError> { Ok((x - y).abs()) };
    let right_side = |x: f64| -> Result<f64, DummyError> { Ok(1.0 + x * x) };
    let from = -1.0;
    let to = 1.0;
    let n = 50;

    let solver = FredholmFirstKindSystemOfEquations::new(0.000000001, n, 10000);
    let res = solver
        .solve(&kernel, &right_side, from, to)?
        .sample(from, to, n)
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;

    let eps = 0.05;
    dbg!(&res);
    assert!(res[1..res.len() - 1]
        .iter()
        .map(|(_, y)| (y - 1.0).abs())
        .all(|diff| diff < eps));

    Ok(())
}
