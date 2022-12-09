use std::fmt::Debug;

use crate::functions::function::Function;

use super::RootError;

pub fn root<E>(
    f: &dyn Function<Error = E>,
    g: &dyn Function<Error = E>,
    from: f64,
    to: f64,
    eps: f64,
    max_iter_count: usize,
) -> Result<(f64, f64), RootError>
where
    E: Debug,
{
    let f = |x| f.apply(x).and_then(|f| g.apply(x).map(|g| f - g));

    let mut a = from;
    let mut b = to;

    let mut f_a = f(a).map_err(|e| RootError::FunctionError(format!("{:?}", e)))?;
    let mut f_b = f(b).map_err(|e| RootError::FunctionError(format!("{:?}", e)))?;

    if f_a == 0.0 {
        return Ok((a, g.apply(a).unwrap()));
    }
    if f_b == 0.0 {
        return Ok((b, g.apply(b).unwrap()));
    }

    if f_a > 0.0 && f_b < 0.0 {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut f_a, &mut f_b);
    } else if f_a < 0.0 && f_b > 0.0 {
    } else {
        return Err(RootError::BadRange(a, b));
    }

    for _ in 0..max_iter_count {
        if a == b || f_a * f_b > 0.0 {
            return Err(RootError::BadRange(a, b));
        }

        let c = (a * f_b - b * f_a) / (f_b - f_a);
        let f_c = f(c).map_err(|e| RootError::FunctionError(format!("{:?}", e)))?;
        if f_c == 0.0 {
            return Ok((c, g.apply(c).unwrap()));
        }

        if f_c > 0.0 {
            if (c - b).abs() < eps && f_c.abs() < eps {
                return Ok((c, g.apply(c).unwrap()));
            }
            b = c;
            f_b = f_c;
        } else {
            if (a - c).abs() < eps && f_c.abs() < eps {
                return Ok((c, g.apply(c).unwrap()));
            }
            a = c;
            f_a = f_c;
        }
    }

    Err(RootError::ItersEnded { from: a, to: b })
}

#[test]
fn secant() -> Result<(), RootError> {
    let f = |x: f64| -> Result<f64, RootError> { Ok(f64::sqrt(f64::exp(f64::sin(x)))) };
    let g = |x: f64| -> Result<f64, RootError> {
        Ok(f64::log(
            f64::abs(f64::sin(f64::powf(2.0, 0.7 * x)) * 5.0),
            std::f64::consts::E,
        ))
    };

    let (x, _) = root(&f, &g, 0.0, 2.0, 0.0001, 10000)?;
    let actual_x = 1.182;

    assert!((x - actual_x).abs() < 0.001);

    Ok(())
}
