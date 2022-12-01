#[derive(Debug)]
pub enum PathKind {
    Line,
    Filled,
}

#[derive(Debug)]
pub struct Path {
    pub pts: Vec<(f64, f64)>,
    pub kind: PathKind,
    pub color: (f32, f32, f32),
}

#[derive(Debug)]
pub struct Viewport {
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
    pub top: f64,
}

impl Viewport {
    pub fn new(left: f64, right: f64, bottom: f64, top: f64) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
        }
    }

    pub fn convert(from: &Viewport, to: &Viewport, pt: (f64, f64)) -> (f64, f64) {
        let (x, y) = pt;
        (
            (x - from.left) / (from.right - from.left) * (to.right - to.left) + to.left,
            (y - from.bottom) / (from.top - from.bottom) * (to.top - to.bottom) + to.bottom,
        )
    }
}

#[derive(Debug)]
pub struct Graph {
    pub paths: Vec<Path>,
    pub viewport: Viewport,
}

impl Graph {
    pub fn new(paths: Vec<Path>) -> Option<Self> {
        let left = paths
            .iter()
            .filter_map(|p| p.pts.iter().map(|(x, _)| *x).reduce(f64::min))
            .reduce(f64::min)?;

        let right = paths
            .iter()
            .filter_map(|p| p.pts.iter().map(|(x, _)| *x).reduce(f64::max))
            .reduce(f64::max)?;

        let bottom = paths
            .iter()
            .filter_map(|p| p.pts.iter().map(|(_, x)| *x).reduce(f64::min))
            .reduce(f64::min)?;

        let top = paths
            .iter()
            .filter_map(|p| p.pts.iter().map(|(_, x)| *x).reduce(f64::max))
            .reduce(f64::max)?;

        Some(Self {
            paths,
            viewport: Viewport::new(left, right, bottom, top),
        })
    }
}
