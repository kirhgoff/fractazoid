use crate::types::*;

pub struct Parameters {
    pub origin_re: RealNumber,
    pub origin_im: RealNumber,
    pub real_width: RealNumber,
    pub real_height: RealNumber,
    pub max_absolute_value: RealNumber,
    pub max_iterations: ScreenNumber,

    pub screen_width: ScreenNumber,
    pub screen_height: ScreenNumber,

    pub function: &'static Function,
    pub chars: &'static str
}

impl Parameters {
    pub fn build(
        origin_re: RealNumber,
        origin_im: RealNumber,
        real_width: RealNumber,
        real_height: RealNumber,
        max_absolute_value: RealNumber,

        screen_width: ScreenNumber,
        screen_height: ScreenNumber,

        function: &'static Function,
        chars: &'static str)
        -> Parameters
    {
        let max_iterations = char_count(chars);

        return Parameters {
            origin_re,
            origin_im,
            real_width,
            real_height,
            max_absolute_value,
            max_iterations,
            screen_width,
            screen_height,
            function,
            chars
        }

    }
}