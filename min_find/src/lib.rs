use std::fmt::Debug;

use common::function::Function;

pub struct Minimum1d {
    pub x: f64,
    pub y: f64,
}

pub trait MinFinder1d {
    type Error;
    fn find_min<E>(
        &self,
        from: f64,
        to: f64,
        func: &dyn Function<Error = E>,
    ) -> Result<Minimum1d, Self::Error>
    where
        E: Debug;
}

pub mod golden_ratio_min;

pub struct MinimumNDim {
    pub x: Vec<f64>,
    pub y: f64,
}
