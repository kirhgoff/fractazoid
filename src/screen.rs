use num_complex::Complex;
use crate::types::*;
use crate::parameters::Parameters;

pub struct Screen {
    pub origin: Complex<RealNumber>,
    pub real_width: RealNumber,
    pub real_height: RealNumber,
    pub screen_width: ScreenNumber,
    pub screen_height: ScreenNumber,
    cell_width: RealNumber,
    cell_height: RealNumber
}

impl Screen {
    pub fn from(params: &Parameters) -> Screen {
        return Screen::new(
            params.origin_re,
            params.origin_im,
            params.real_width,
            params.real_height,
            params.screen_width,
            params.screen_height
        );
    }

    pub fn new(
        origin_re: RealNumber,
        origin_im: RealNumber,
        real_width: RealNumber,
        real_height: RealNumber,
        screen_width: ScreenNumber,
        screen_height: ScreenNumber)
        -> Screen
    {
        let cell_width = real_width / (screen_width as RealNumber);
        let cell_height = real_height / (screen_height as RealNumber);

        return Screen {
            origin: Complex::new(origin_re, origin_im),
            real_width,
            real_height,
            screen_width,
            screen_height,
            cell_width,
            cell_height
        }
    }

    pub fn convert(&self, x: ScreenNumber, y: ScreenNumber) -> Complex<RealNumber> {
        let dx = (x as RealNumber) * self.cell_width;
        let dy = (y as RealNumber) * self.cell_height;

        return Complex::new(
            self.origin.re + dx,
            self.origin.im - dx
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build() -> Screen {
        return Screen::new(
             -0.5,
             0.5,
             1.0,
            1.0,
            10,
            10
        )
    }
    #[test]
    fn test_screen_origin() {
        let result = build().convert(0, 0);
        assert_eq!(result, Complex::new(-0.5, 0.5));
    }

    #[test]
    fn test_screen_center() {
        let result = build().convert(5, 5);
        assert_eq!(result, Complex::new(0.0, 0.0));
    }
}