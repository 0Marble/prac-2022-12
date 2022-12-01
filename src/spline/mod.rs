use crate::common::function::Function;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Io(String),
    PointOutOfBounds { x: f64, min: f64, max: f64 },
    NoKnownPoints,
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Error::Io(e.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spline {
    pts: Vec<(f64, f64)>,
    coefs: Vec<(f64, f64, f64, f64)>,
}

impl Spline {
    pub fn new(known_points: Vec<(f64, f64)>) -> Self {
        Self {
            coefs: calc_spline_params(&known_points),
            pts: known_points,
        }
    }

    pub fn write_coefs(&self) -> Result<String, Error> {
        let mut s = String::new();

        for (a, b, c, d) in self.coefs.iter() {
            writeln!(s, "{},{},{},{}", a, b, c, d)?;
        }

        Ok(s)
    }
}

impl Function for Spline {
    type Error = Error;

    fn apply(&self, arg: f64) -> Result<f64, Self::Error> {
        if self.pts.is_empty() {
            return Err(Error::NoKnownPoints);
        }

        for i in 1..self.pts.len() {
            let (x, _) = self.pts[i];
            let (prev_x, _) = self.pts[i - 1];

            if prev_x <= arg && x >= arg {
                let (a, b, c, d) = self.coefs[i - 1];
                let val = d * x * x * x + c * x * x + b * x + a;
                return Ok(val);
            }
        }

        Err(Error::PointOutOfBounds {
            x: arg,
            min: self.pts.first().unwrap().0,
            max: self.pts.last().unwrap().0,
        })
    }
}

fn calc_spline_params(pts: &[(f64, f64)]) -> Vec<(f64, f64, f64, f64)> {
    let n = pts.len();
    let mut b = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    let mut d = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    let mut a = (0..n - 1).map(|_| 0.0).collect::<Vec<_>>();
    let mut c = (0..n - 1).map(|_| 0.0).collect::<Vec<_>>();

    for i in 1..n - 1 {
        let mui = (pts[i].0 - pts[i - 1].0) / (pts[i + 1].0 - pts[i - 1].0);
        let lambdai = (pts[i + 1].0 - pts[i].0) / (pts[i + 1].0 - pts[i - 1].0);

        d[i] = 3.0
            * (mui * (pts[i + 1].1 - pts[i].1) / (pts[i + 1].0 - pts[i].0)
                + lambdai * (pts[i].1 - pts[i - 1].1) / (pts[i].0 - pts[i - 1].0));
        a[i - 1] = lambdai;
        b[i] = 2.0;
        c[i] = mui;
    }

    d[0] = 3.0 * (pts[1].1 - pts[0].1) / (pts[1].0 - pts[0].0);
    d[n - 1] = 3.0 * (pts[n - 1].1 - pts[n - 2].1) / (pts[n - 1].0 - pts[n - 2].0);
    b[0] = 2.0;
    c[0] = 1.0;
    a[n - 2] = 1.0;
    b[n - 1] = 2.0;

    let mut y = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    let mut alpha = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    let mut beta = (0..n).map(|_| 0.0).collect::<Vec<_>>();

    y[0] = b[0];
    alpha[0] = -c[0] / y[0];
    beta[0] = d[0] / y[0];
    for i in 1..n - 1 {
        y[i] = b[i] + a[i - 1] * alpha[i - 1];
        alpha[i] = -c[i] / y[i];
        beta[i] = (d[i] - a[i - 1] * beta[i - 1]) / y[i];
    }

    let mut m = (0..n).map(|_| 0.0).collect::<Vec<_>>();
    m[n - 1] = beta[n - 1];
    for i in 1..n - 1 {
        let j = n - i - 1;
        m[j] = alpha[j] * m[j + 1] + beta[j];
    }

    (0..n - 1)
        .map(|i| {
            let a = pts[i].1;
            let b = pts[i + 1].1;
            let c = pts[i].0;
            let d = pts[i + 1].0;
            let n = m[i + 1];
            let m = m[i];

            let div1 = (d - c) * (d - c) * (d - c);
            let div2 = (d - c) * (d - c);

            (
                (a * d * d * d - 3.0 * a * c * d * d - c * c * c * b + 3.0 * d * c * c * b) / div1
                    + (-m * c * d * d - n * d * c * c) / div2,
                (6.0 * a * d * c + 2.0 * b * c * c - 2.0 * c * c * b - 6.0 * d * b * c) / div1
                    + (m * d * d + 2.0 * m * d * c + 2.0 * n * d * c + n * c * c) / div2,
                (-3.0 * a * d - 3.0 * a * c + 3.0 * b * c + 3.0 * d * b) / div1
                    + (-2.0 * m * d - m * c - n * d - 2.0 * n * c) / div2,
                (2.0 * a - 2.0 * b) / div1 + (m + n) / div2,
            )
        })
        .collect()
}

#[test]
fn spline() -> Result<(), Error> {
    let from = 0.0;
    let to = 10.0;
    let n = 100;
    let step = (to - from) / (n as f64);

    let pts = (0..=n)
        .map(|i| {
            let x = (i as f64) * step;
            (x, x.sin())
        })
        .collect::<Vec<_>>();

    let spline = Spline::new(pts);
    spline.write_coefs()?;

    let check_n = n * 10;
    let check_step = (to - from) / (check_n as f64);

    let eps = 0.1;
    assert!((0..=check_n)
        .map(|i| (i as f64) * check_step)
        .map(|x| (x.sin() - spline.apply(x).unwrap()).abs())
        .all(|diff| diff < eps));

    Ok(())
}
