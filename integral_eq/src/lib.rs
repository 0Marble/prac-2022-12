use std::fmt::Debug;

use common::function::{Function, Function2d};

pub mod system_of_equations_based;

pub trait FredholmFirstKind {
    type MethodError;
    type ResultFunctionError;

    fn solve<E1, E2>(
        &self,
        kernel: &dyn Function2d<Error = E2>,
        right_side: &dyn Function<Error = E1>,
        from: f64,
        to: f64,
    ) -> Result<Box<dyn Function<Error = Self::ResultFunctionError>>, Self::MethodError>
    where
        E1: Debug,
        E2: Debug;
}

pub trait FredholmSecondKind {
    type MethodError;
    type ResultFunctionError;

    fn solve<E1, E2>(
        &self,
        kernel: &dyn Function2d<Error = E2>,
        right_side: &dyn Function<Error = E1>,
        from: f64,
        to: f64,
        lambda: f64,
    ) -> Result<Box<dyn Function<Error = Self::ResultFunctionError>>, Self::MethodError>
    where
        E1: Debug,
        E2: Debug;
}
