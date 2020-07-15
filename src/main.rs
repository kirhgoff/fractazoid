use num_complex::Complex;

type RealNumber = f64;
type ScreenNumber = i64; // TODO: find better name

struct LimitDetector<F>
where F: Fn(Complex<RealNumber>) -> Complex<RealNumber> {
    max_iterations: ScreenNumber,
    max_absolute_value: RealNumber,
    function: F
}

impl<F> LimitDetector<F>
where F: Fn(Complex<RealNumber>) -> Complex<RealNumber> {
    fn iterations(&self, _: Complex<RealNumber>) -> ScreenNumber {
        return 0
    }
}

fn main() {
    println!("Fractazoid a Fractal Explorer v1.0 [2020-07-15]");
    println!("==================================");

    // let limitDetector = LimitDetector::new(
    //     max_iterations: usize, max_absolute_value: f64, )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_detector1() {
        let constant: Complex<RealNumber> = Complex::new(1.0, 1.0);
        let function = |z| z + constant;
        let limit_detector = LimitDetector {
            max_iterations: 10,
            max_absolute_value: 20.0,
            function
        };
        // iteration z module
        // 0    0,0     0
        // 1    1,1     2
        // 2    2,2     4
        // 3    3,3     18
        // 4    4,4     32

        let input = Complex::new(1.0, 1.0);
        let result = limit_detector.iterations(input);
        assert_eq!(4, result);
    }
}