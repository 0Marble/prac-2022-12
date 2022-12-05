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
    fn sample(
        &self,
        from_x: f64,
        to_x: f64,
        from_y: f64,
        to_y: f64,
        x_n: usize,
        y_n: usize,
    ) -> Result<Vec<(f64, f64, f64)>, Self::Error> {
        let x_step = (to_x - from_x) / (x_n as f64 - 1.0);
        let y_step = (to_y - from_y) / (y_n as f64 - 1.0);

        (0..x_n * y_n)
            .map(|i| {
                let x = ((i % x_n) as f64) * x_step + from_x;
                let y = ((i / x_n) as f64) * y_step + from_y;

                self.apply(x, y).map(|z| (x, y, z))
            })
            .collect()
    }
}

pub trait FunctionNd {
    type Error;
    fn apply(&self, args: &[f64]) -> Result<f64, Self::Error>;
    fn sample(&self, from: &[f64], to: &[f64], n: &[usize]) -> Result<Vec<Vec<f64>>, Self::Error> {
        let mut pts = vec![];
        let mut iter: Vec<usize> = (0..n.len()).map(|_| 0).collect();
        let total_iter_count: usize = n.iter().product();
        let steps: Vec<f64> = from
            .iter()
            .zip(to.iter())
            .zip(n.iter())
            .map(|((from, to), n)| (to - from) / (*n as f64 - 1.0))
            .collect();

        for _ in 0..total_iter_count {
            for i in 0..n.len() {
                if iter[i] + 1 < n[i] {
                    iter[i] += 1;
                    break;
                } else {
                    iter[i] = 0;
                }
            }

            let mut coords: Vec<f64> = steps
                .iter()
                .enumerate()
                .map(|(i, step)| (iter[i] as f64) * step + from[i])
                .collect();
            coords.push(self.apply(&coords)?);
            pts.push(coords);
        }

        Ok(pts)
    }
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

impl<E, F> FunctionNd for F
where
    F: Fn(&[f64]) -> Result<f64, E>,
{
    type Error = E;

    fn apply(&self, args: &[f64]) -> Result<f64, Self::Error> {
        (self)(args)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoError {}

impl Function for f64 {
    type Error = NoError;

    fn apply(&self, _: f64) -> Result<f64, Self::Error> {
        Ok(*self)
    }
}

impl Function2d for f64 {
    type Error = NoError;

    fn apply(&self, _: f64, _: f64) -> Result<f64, Self::Error> {
        Ok(*self)
    }
}
