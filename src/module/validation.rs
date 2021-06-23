use crate::module::Module;
use crate::Direction;

pub struct ValidationModule {}

impl ValidationModule {
    pub fn new() -> Self {
        Self {}
    }
}

impl Module for ValidationModule {
    fn process(&mut self, input: char, _: &Direction) -> Option<char> {
        let validated = input.to_ascii_uppercase();
        match validated {
            'A'..='Z' => Some(validated),
            _ => None
        }
    }

    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_module() {
        let mut module = ValidationModule::new();
        let output = module.process('a', &Direction::Forward {});
        assert_eq!(output, Some('A'));
    }
}
