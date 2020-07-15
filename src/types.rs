use num_complex::Complex;

pub type RealNumber = f64;
pub type ScreenNumber = i64; // TODO: find better name
pub type Function = dyn Fn(Complex<RealNumber>) -> Complex<RealNumber>;