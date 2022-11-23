use common::{function::*, table_function::TableFunction};
use std::fmt::Debug;

use super::{conjugate_gradients::*, Error};

pub fn fredholm_1st_system<E1, E2>(
    kernel: &dyn Function2d<Error = E1>,
    right_side: &dyn Function<Error = E2>,
    from: f64,
    to: f64,
    n: usize,
    eps: f64,
    max_iter_count: usize,
) -> Result<TableFunction, Error>
where
    E1: Debug,
    E2: Debug,
{
    let step = (to - from) / (n as f64 - 1.0);

    let mut mat = (0..n * n).map(|_| 0.0).collect::<Vec<_>>();
    let mut mat_transpozed = (0..n * n).map(|_| 0.0).collect::<Vec<_>>();
    let mut identity = (0..n * n).map(|_| 0.0).collect::<Vec<_>>();

    for i in 0..n {
        for j in 0..n {
            let x = (i as f64) * step + from;
            let y = (j as f64) * step + from;

            mat[i * n + j] = kernel
                .apply(x, y)
                .map(|res| res * step)
                .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;
            mat_transpozed[j * n + i] = mat[i * n + j];
        }
        identity[i * n + i] = 1.0;
    }

    let mut a = (0..n * n).map(|_| 0.0).collect::<Vec<_>>();
    let mut f = (0..n * n).map(|_| 0.0).collect::<Vec<_>>();

    mult_mat(&mat_transpozed, &mat, &mut a, n);
    apply(
        &mat_transpozed,
        (0..n)
            .map(|i| right_side.apply((i as f64) * step + from))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Error::FunctionError(format!("{:?}", e)))?
            .as_ref(),
        &mut f,
        n,
    );

    let mut res = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    conjugate_gradient_method(&a, &identity, &mut res, &f, n, eps, max_iter_count);

    Ok(TableFunction::from_table(
        res.iter()
            .enumerate()
            .map(|(i, y)| ((i as f64) * step + from, *y))
            .collect(),
    ))
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

    let res = fredholm_1st_system(&kernel, &right_side, from, to, n, 1e-8, 10000)?
        .sample(from, to, n)
        .map_err(|e| Error::FunctionError(format!("{:?}", e)))?;

    let eps = 0.05;
    assert!(res[1..res.len() - 1]
        .iter()
        .map(|(_, y)| (y - 1.0).abs())
        .all(|diff| diff < eps));

    Ok(())
}
