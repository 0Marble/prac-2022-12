use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

mod secant_method_root;
mod simpson_integrator;

use crate::common::function::Function;
use secant_method_root::root;
use simpson_integrator::integrate_step;

#[derive(Debug, Clone, PartialEq)]
pub enum RootError {
    FunctionError(String),
    BadRange(f64, f64),
    ItersEnded { from: f64, to: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FunctionError(String),
    RootError(String),
    ItersEnded,
    RootEpsTooBig,
}

#[allow(clippy::too_many_arguments)]
pub fn calc_area<E>(
    a: &dyn Function<Error = E>,
    b: &dyn Function<Error = E>,
    c: &dyn Function<Error = E>,
    ab_root: [f64; 2],
    ac_root: [f64; 2],
    bc_root: [f64; 2],
    root_start_eps: f64,
    area_eps: f64,
    max_iter_count: usize,
) -> Result<(f64, f64, f64, f64), Error>
where
    E: Debug,
{
    let mut root_eps = root_start_eps;

    for _ in 0..max_iter_count {
        let (abx, aby) = root(a, b, ab_root[0], ab_root[1], root_eps, max_iter_count)
            .map_err(|e| Error::RootError(format!("{:?}", e)))?;
        let (acx, acy) = root(a, c, ac_root[0], ac_root[1], root_eps, max_iter_count)
            .map_err(|e| Error::RootError(format!("{:?}", e)))?;
        let (bcx, bcy) = root(b, c, bc_root[0], bc_root[1], root_eps, max_iter_count)
            .map_err(|e| Error::RootError(format!("{:?}", e)))?;

        let mut sides = [(abx, aby, c), (acx, acy, b), (bcx, bcy, a)];
        sides.sort_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());

        let slope1 = (sides[1].1 - sides[0].1) / (sides[1].0 - sides[0].0);
        let slope2 = (sides[2].1 - sides[0].1) / (sides[2].0 - sides[0].0);

        let res = if slope1 > slope2 {
            calc_area_top_triangle(sides, root_eps, area_eps, max_iter_count)
        } else {
            calc_area_bottom_triangle(sides, root_eps, area_eps, max_iter_count)
        };

        match res {
            Ok(area) => return Ok((area, abx, acx, bcx)),
            Err(e) if e == Error::RootEpsTooBig || e == Error::ItersEnded => root_eps *= 0.1,
            Err(e) => return Err(e),
        }
    }

    Err(Error::ItersEnded)
}

fn calc_area_top_triangle<E>(
    sides: [(f64, f64, &dyn Function<Error = E>); 3],
    root_eps: f64,
    area_eps: f64,
    max_iter_count: usize,
) -> Result<f64, Error>
where
    E: Debug,
{
    let mut max_cache0 = vec![];
    let mut max_cache1 = vec![];
    let mut max_cache2 = vec![];
    let mut min_cache0 = vec![];
    let mut min_cache1 = vec![];
    let mut min_cache2 = vec![];

    let mut max_n0 = 0;
    let mut max_n1 = 0;
    let mut max_n2 = 0;
    let mut min_n0 = 0;
    let mut min_n1 = 0;
    let mut min_n2 = 0;

    let a = sides[0].0;
    let b = sides[1].0;
    let c = sides[2].0;
    let f2 = sides[0].2;
    let f3 = sides[1].2;
    let f1 = sides[2].2;

    let mut calc_smax = || -> Result<f64, Error> {
        Ok(
            integrate_step(f1, a - root_eps, b + root_eps, &mut max_n0, &mut max_cache0)?
                + integrate_step(f2, b - root_eps, c + root_eps, &mut max_n1, &mut max_cache1)?
                - integrate_step(f3, a + root_eps, c - root_eps, &mut min_n2, &mut min_cache2)?,
        )
    };

    let mut calc_smin = || -> Result<f64, Error> {
        Ok(
            integrate_step(f1, a + root_eps, b - root_eps, &mut min_n0, &mut min_cache0)?
                + integrate_step(f2, b + root_eps, c - root_eps, &mut min_n1, &mut min_cache1)?
                - integrate_step(f3, a - root_eps, c + root_eps, &mut max_n2, &mut max_cache2)?,
        )
    };

    let mut smax_prev = calc_smax()?;
    let mut smin_prev = calc_smin()?;

    for _ in 0..max_iter_count {
        let smax = calc_smax()?;
        let smin = calc_smin()?;

        if (smax - smin).abs() > area_eps {
            return Err(Error::RootEpsTooBig);
        }

        if (smax - smax_prev).abs() < area_eps && (smin - smin_prev).abs() < area_eps {
            return Ok((smax + smin) / 2.0);
        }

        smax_prev = smax;
        smin_prev = smin;
    }

    Err(Error::ItersEnded)
}

fn calc_area_bottom_triangle<E>(
    sides: [(f64, f64, &dyn Function<Error = E>); 3],
    root_eps: f64,
    area_eps: f64,
    max_iter_count: usize,
) -> Result<f64, Error>
where
    E: Debug,
{
    let mut max_cache0 = vec![];
    let mut max_cache1 = vec![];
    let mut max_cache2 = vec![];
    let mut min_cache0 = vec![];
    let mut min_cache1 = vec![];
    let mut min_cache2 = vec![];

    let mut max_n0 = 0;
    let mut max_n1 = 0;
    let mut max_n2 = 0;
    let mut min_n0 = 0;
    let mut min_n1 = 0;
    let mut min_n2 = 0;

    let a = sides[0].0;
    let b = sides[1].0;
    let c = sides[2].0;
    let f3 = sides[0].2;
    let f1 = sides[1].2;
    let f2 = sides[2].2;

    let mut calc_smax = || -> Result<f64, Error> {
        Ok(
            integrate_step(f1, a - root_eps, c + root_eps, &mut max_n0, &mut max_cache0)?
                - integrate_step(f2, a + root_eps, b - root_eps, &mut min_n1, &mut min_cache1)?
                - integrate_step(f3, b + root_eps, c - root_eps, &mut min_n2, &mut min_cache2)?,
        )
    };

    let mut calc_smin = || -> Result<f64, Error> {
        Ok(
            integrate_step(f1, a + root_eps, c - root_eps, &mut min_n0, &mut min_cache0)?
                - integrate_step(f2, a - root_eps, b + root_eps, &mut max_n1, &mut max_cache1)?
                - integrate_step(f3, b - root_eps, c + root_eps, &mut max_n2, &mut max_cache2)?,
        )
    };

    let mut smax_prev = calc_smax()?;
    let mut smin_prev = calc_smin()?;

    for _ in 0..max_iter_count {
        let smax = calc_smax()?;
        let smin = calc_smin()?;

        if (smax - smin).abs() > area_eps {
            return Err(Error::RootEpsTooBig);
        }

        if (smax - smax_prev).abs() < area_eps && (smin - smin_prev).abs() < area_eps {
            return Ok((smax + smin) / 2.0);
        }

        smax_prev = smax;
        smin_prev = smin;
    }

    Err(Error::ItersEnded)
}

#[test]
fn area_bottom() -> Result<(), Error> {
    let f = |x: f64| -> Result<f64, RootError> { Ok(1.0 + 4.0 / (x * x + 1.0)) };
    let g = |x: f64| -> Result<f64, RootError> { Ok(2.0f64.powf(-x)) };
    let h = |x: f64| -> Result<f64, RootError> { Ok(x * x * x) };

    let (res, _, _, _) = calc_area(
        &f,
        &g,
        &h,
        [-2.0, -1.0],
        [0.5, 1.5],
        [0.5, 1.5],
        0.001,
        0.001,
        1000,
    )?;

    let actual = 6.5910711;
    assert!((res - actual).abs() < 0.001);

    Ok(())
}

#[test]
fn area_top() -> Result<(), Error> {
    let f = |x: f64| -> Result<f64, RootError> { Ok(f64::exp(x) + 2.0) };
    let g = |x: f64| -> Result<f64, RootError> { Ok(-2.0 * x + 8.0) };
    let h = |x: f64| -> Result<f64, RootError> { Ok(-5.0 / x) };

    let (res, _, _, _) = calc_area(
        &f,
        &g,
        &h,
        [0.0, 2.0],
        [-4.0, -1.0],
        [-2.0, -0.1],
        0.001,
        0.0001,
        1000,
    )?;

    let actual = 9.807;
    assert!((res - actual).abs() < 0.001);

    Ok(())
}