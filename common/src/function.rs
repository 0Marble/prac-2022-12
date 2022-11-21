use std::fmt::Write;

pub trait Function {
    type Error;

    fn apply(&self, x: f64) -> Result<f64, Self::Error>;
    fn pts_to_str(&self, pts: &[f64]) -> Result<String, Self::Error>
    where
        Self::Error: From<std::fmt::Error>,
    {
        let mut s = String::new();
        for x in pts {
            writeln!(&mut s, "{},{}", x, self.apply(*x)?)?;
        }
        Ok(s)
    }

    fn sample(&self, from: f64, to: f64, n: usize) -> Result<Vec<(f64, f64)>, Self::Error> {
        let step = (to - from) / (n as f64);
        (0..=n)
            .map(|i| (i as f64) * step + from)
            .map(|x| self.apply(x).map(|y| (x, y)))
            .collect()
    }
}

pub trait Function2d {
    type Error;

    fn apply(&self, x: f64, y: f64) -> Result<f64, Self::Error>;
}

impl<E, F> Function for F
where
    F: Fn(f64) -> Result<f64, E>,
{
    type Error = E;

    fn apply(&self, x: f64) -> Result<f64, Self::Error> {
        (self)(x)
    }
}

impl<E, F> Function2d for F
where
    F: Fn(f64, f64) -> Result<f64, E>,
{
    type Error = E;

    fn apply(&self, x: f64, y: f64) -> Result<f64, Self::Error> {
        (self)(x, y)
    }
}
