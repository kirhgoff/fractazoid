mod renderer;
mod screen;
mod limit_detector;
mod types;

use num_complex::Complex;

// TODO: should it be like that?
use crate::limit_detector::LimitDetector;
use crate::screen::Screen;
use crate::types::*;

fn main() {
    println!("Fractazoid a Fractal Explorer v1.0 [2020-07-15]");
    println!("==================================");

    let limit_detector = LimitDetector {
        max_iterations: 10,
        max_absolute_value: 20.0,
        function: &|z: Complex<RealNumber>| z + Complex::new(1.0, 1.0)
    };

    let screen = Screen::new(
        -0.5,
        0.5,
        1.0,
        1.0,
        10,
        10
    );
}