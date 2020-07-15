use num_complex::Complex;

pub type RealNumber = f64;
pub type ScreenNumber = i64; // TODO: find better name
pub type Function = dyn Fn(Complex<RealNumber>) -> Complex<RealNumber>;

pub fn char_count(chars: &str) -> ScreenNumber {
    // TODO: really?
    let mut count = 0;
    for _ in chars.chars() {
        count += 1;
    }
    return count;
}