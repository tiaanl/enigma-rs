use crate::module::Module;
use crate::Direction;

pub struct Enigma {
    modules: Vec<Box<dyn Module>>,
}

impl Enigma {
    pub fn new() -> Self {
        Self {
            modules: Vec::new()
        }
    }

    pub fn add_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    pub fn process(&mut self, input: &str, dir: &Direction) -> String {
        let mut result = String::new();

        for input_char in input.chars() {
            let mut current_char = input_char;
            self.modules.iter_mut().for_each(|m| {
                if let Some(processed_char) = m.process(current_char, dir) {
                    current_char = processed_char;
                } else {
                    panic!("Could not process char: {}", current_char);
                }
            });
            result.push(current_char);
        };

        result
    }

    pub fn reset(&mut self) {
        self.modules.iter_mut().for_each(|m| m.reset());
    }
}
