pub type RangeT = u32;
pub type ProbT = f64;

pub const WINDOW_SIZE: usize = 200;
pub static THRESHOLDS: &'static [ProbT] = &[0.000, 0.005, 0.005, 0.0023, 0.0038];