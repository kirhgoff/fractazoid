mod parameters;
mod renderer;
mod screen;
mod limit_detector;
mod types;

use num_complex::Complex;

// TODO: should it be like that?
use crate::limit_detector::LimitDetector;
use crate::screen::Screen;
use crate::renderer::Renderer;
use crate::parameters::Parameters;
use crate::types::Function;

//const CHARS: &str = ".`:,;'_^\"></-!~=)(|j?}{ ][ti+l7v1%yrfcJ32uIC$zwo96sngaT5qpkYVOL40&mG8*xhedbZUSAQPFDXWK#RNEHBM@ ";
const CHARS: &str = "─━│┃┄┅┆┇┈┉┊┋┌┍┎┏┐┑┒┓└┕┖┗┘┙┚┛├┝┞┟┠┡┢┣┤┥┦┧┨┩┪┫┬┭┮┯┰┱┲┳┴┵┶┷┸┹┺┻┼┽┾┿╀╁╂╃╄╅╆╇╈╉╊╋ ";
//const CHARS: &str = "┥┝┺╋┞┣┡┿┲┹┳┨┊╅┼┪╊┩┅┚┋┫└┵┍┠┇┏╇╆┛┗┙│┖┃┕┰╃ ┒┘┆├┭┟┦┈┷┽╂━┯─┧┎┶┱┸╀┢╉┮╁┾┄┓┉╄┐┑┴┌┻╈┬┤";
const FORMULA: &Function = &| z | z*z + Complex::new(-0.75, 0.1);

fn main() {
    println!("Fractazoid a Fractal Explorer v1.0 [2020-07-15]");
    println!("==================================");

    let params = Parameters::build(
        // Complex plane
        -1.5,
        1.5,
        3.0,
        3.0,
        4.0,

        //Viewport
        80,
        20,

        FORMULA,
        CHARS
    );

    let limit_detector = LimitDetector::from(&params);
    let screen = Screen::from(&params);
    let renderer = Renderer::from(&params);

    let mut output: Vec<char> = vec![];
    for y in 0..params.screen_height {
        for x in 0..params.screen_width {
            let z = screen.convert(x, y);
            let iterations = limit_detector.iterations(z);
            let cell = renderer.render(iterations);
            output.push(*cell)
        }
        output.push('\n')
    }
    let string: String = output.into_iter().collect();
    println!("{}", string);
}