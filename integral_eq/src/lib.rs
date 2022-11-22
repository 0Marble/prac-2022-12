use std::fmt::Debug;

use common::function::{Function, Function2d};

pub mod fredholm;
pub mod wolterra;

pub trait FredholmFirstKind {
    type MethodError;
    type ReturnFunction: Function;

    fn solve<E1, E2>(
        &self,
        kernel: &dyn Function2d<Error = E2>,
        right_side: &dyn Function<Error = E1>,
        from: f64,
        to: f64,
    ) -> Result<Self::ReturnFunction, Self::MethodError>
    where
        E1: Debug,
        E2: Debug;
}

pub trait FredholmSecondKind {
    type MethodError;
    type ReturnFunction: Function;

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
        E2: Debug;
}

pub trait WolterraFirstKind {
    type MethodError;
    type ReturnFunction: Function;

    fn solve<E1, E2>(
        &self,
        kernel: &dyn Function2d<Error = E1>,
        right_side: &dyn Function<Error = E2>,
        from: f64,
        to: f64,
    ) -> Result<Self::ReturnFunction, Self::MethodError>
    where
        E1: Debug,
        E2: Debug;
}

pub trait WolterraSecondKind {
    type MethodError;
    type ReturnFunction: Function;

    fn solve<E1, E2>(
        &self,
        kernel: &dyn Function2d<Error = E1>,
        right_side: &dyn Function<Error = E2>,
        from: f64,
        to: f64,
        lambda: f64,
    ) -> Result<Self::ReturnFunction, Self::MethodError>
    where
        E1: Debug,
        E2: Debug;
}
