pub mod golden_ratio_min;
pub mod gradients_min;
pub mod penalty_min;

#[derive(Debug, Clone, PartialEq)]
pub struct Minimum1d {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MinimumNd {
    pub x: Vec<f64>,
    pub y: f64,
}
