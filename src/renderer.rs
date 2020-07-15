use crate::types::*;

use std::collections::HashMap;

pub struct Renderer {
    pub max_iterations: ScreenNumber,
    pub chars: HashMap<ScreenNumber, char>,
    iterations_per_char: RealNumber,
}

impl Renderer {
    pub fn new(max_iterations: ScreenNumber, all_chars: &str) -> Renderer {
        let mut chars = HashMap::new();
        let mut index = 0;
        for character in all_chars.chars() {
            index += 1;
            chars.insert(index, character);
        }
        let chars_count = index;
        let iterations_per_char = (max_iterations as RealNumber) / (chars_count as RealNumber);

        return Renderer { max_iterations, chars, iterations_per_char }
    }

    pub fn render(&self, iteration: ScreenNumber) -> char {
        return 's';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn renderer() -> Renderer {
        // 12 characters
        return Renderer::new(24, &String::from(".,-~:;=!*#$@"));
    }

    #[test]
    fn test_renderer_first() {
        assert_eq!('.', renderer().render(0));
    }

    #[test]
    fn test_renderer_last() {
        assert_eq!('@', renderer().render(24));
    }

    #[test]
    fn test_renderer_some() {
        assert_eq!('-', renderer().render(6));
    }
}