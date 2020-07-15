use num_complex::Complex;
use crate::types::*;

pub struct LimitDetector {
    pub max_iterations: ScreenNumber,
    pub max_absolute_value: RealNumber,
    pub function: &'static dyn Fn(Complex<RealNumber>) -> Complex<RealNumber>
}

impl LimitDetector {
    pub fn iterations(&self, number: Complex<RealNumber>) -> ScreenNumber {

        let mut current = number;
        let mut iterations = 0;

        loop {
            if iterations >= self.max_iterations || current.norm_sqr() >= self.max_absolute_value {
                break;
            }
            current = (self.function)(current);
            iterations += 1;
        }
        return iterations;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_limit_detector(max_iterations: ScreenNumber, max_absolute_value: RealNumber, function: &'static Function) -> LimitDetector {
        return LimitDetector { max_iterations, max_absolute_value, function };
    }

    #[test]
    fn test_limit_detector_moving_out() {
        // iteration z module
        // 0    0,0     0
        // 1    1,1     2
        // 2    2,2     8
        // 3    3,3     18
        // 4    4,4     32

        let limit_detector = build_limit_detector(
            10,
            20.0,
            &|z: Complex<RealNumber>| z + Complex::new(1.0, 1.0)
        );
        let result = limit_detector.iterations(Complex::new(0.0, 0.0));
        assert_eq!(4, result);
    }

    #[test]
    fn test_limit_detector_stable_point() {
        let limit_detector = build_limit_detector(
            10,
            20.0,
            &|z: Complex<RealNumber>| z
        );
        let result = limit_detector.iterations(Complex::new(0.0, 0.0));
        assert_eq!(10, result);
    }

    #[test]
    fn test_limit_detector_greater_than_abs_number() {
        let limit_detector = build_limit_detector(
            10,
            20.0,
            &|z: Complex<RealNumber>| z
        );
        let result = limit_detector.iterations(Complex::new(100.0, 100.0));
        assert_eq!(0, result);
    }
}