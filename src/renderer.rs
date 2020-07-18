use crate::types::*;
use crate::parameters::Parameters;

use std::collections::HashMap;

pub struct Renderer {
    pub max_iterations: ScreenNumber,
    pub chars: HashMap<ScreenNumber, char>,
    iterations_per_char: RealNumber,
    chars_count: ScreenNumber
}

impl Renderer {
    pub fn from(params: &Parameters) -> Renderer {
        return Renderer::new(
            params.max_iterations,
            params.chars
        );
    }

    pub fn new(max_iterations: ScreenNumber, all_chars: &str) -> Renderer {
        let mut chars = HashMap::new();
        let chars_count = char_count(all_chars);
        let iterations_per_char = (max_iterations as RealNumber) / (chars_count as RealNumber);

        let  mut index = 0;
        for character in all_chars.chars() {
            chars.insert(index, character);
            index += 1
        }

        return Renderer { max_iterations, chars, iterations_per_char, chars_count }
    }

    pub fn render(&self, iteration: ScreenNumber) -> &char {
        assert!(
            iteration >= 0 && iteration < self.max_iterations,
            format!(
                "Invalid iteration: {} allowed max: {}",
                iteration, self.max_iterations
            )
        );
        let index = ((iteration as RealNumber) / self.iterations_per_char).floor() as ScreenNumber;
        assert!(
            index < self.chars_count,
            format!("Index {} should be less than {}", index, self.chars_count)
        );
        return self.chars.get(&index).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn renderer() -> Renderer {
        // 12 characters
        return Renderer::new(24, &String::from("abcdefghhijk"));
    }

    #[test]
    fn test_renderer_first() {
        assert_eq!('a', *renderer().render(0));
    }

    #[test]
    fn test_renderer_last() {
        assert_eq!('k', *renderer().render(24 - 1));
    }

    #[test]
    fn test_renderer_some() {
        assert_eq!('d', *renderer().render(6)); // a:0,1, b:2,3, c:4,5 d:6
    }
}