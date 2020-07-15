use num_complex::Complex;

type RealNumber = f64;
type ScreenNumber = i64; // TODO: find better name

struct LimitDetector {
    max_iterations: ScreenNumber,
    max_absolute_value: RealNumber,
    function: &'static dyn Fn(Complex<RealNumber>) -> Complex<RealNumber>
}

impl LimitDetector {
    fn iterations(&self, number: Complex<RealNumber>) -> ScreenNumber {

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

fn main() {
    println!("Fractazoid a Fractal Explorer v1.0 [2020-07-15]");
    println!("==================================");

    // let limitDetector = LimitDetector::new(
    //     max_iterations: usize, max_absolute_value: f64, )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_limit_detector() -> LimitDetector {
        //let constant: Complex<RealNumber> = Complex::new(1.0, 1.0);
        // let function = ;
        return LimitDetector {
            max_iterations: 10,
            max_absolute_value: 20.0,
            function: &|z: Complex<RealNumber>| z + Complex::new(1.0, 1.0)
        };
    }

    #[test]
    fn test_limit_detector1() {
        let limit_detector = LimitDetector {
            max_iterations: 10,
            max_absolute_value: 20.0,
            function: &|z| z + Complex::new(1.0, 1.0)
        };
        // iteration z module
        // 0    0,0     0
        // 1    1,1     2
        // 2    2,2     8
        // 3    3,3     18
        // 4    4,4     32

        let input = Complex::new(0.0, 0.0);
        let result = limit_detector.iterations(input);
        assert_eq!(4, result);
    }
}