mod rotor;

pub use rotor::RotorModule;

use crate::Direction;

pub trait Module {
    fn process(&mut self, input: char, dir: &Direction) -> Option<char>;
    fn reset(&mut self);
}

pub struct LoggingModule {
    label: String,
    counter: u32,
}

impl LoggingModule {
    pub fn new() -> Self {
        Self {
            label: "input".to_string(),
            counter: 0,
        }
    }

    pub fn with_label(label: &str) -> Self {
        Self {
            label: label.to_string(),
            counter: 0,
        }
    }
}

impl Module for LoggingModule {
    fn process(&mut self, input: char, _: &Direction) -> Option<char> {
        println!("{}> {}: {}", &self.counter, &self.label, input);
        self.counter += 1;

        Some(input)
    }

    fn reset(&mut self) {
        self.counter = 0;
    }
}

struct InputValidationModule(u32, u32);

impl Module for InputValidationModule {
    fn process(&mut self, input: char, _: &Direction) -> Option<char> {
        if input as u32 >= self.0 && input as u32 <= self.1 {
            Some(input)
        } else {
            None
        }
    }

    fn reset(&mut self) {}
}

type Pair = (char, char);

pub struct SwapModule {
    pairs: Vec<Pair>,
}

impl SwapModule {
    pub fn with_pairs(pairs: Vec<Pair>) -> Self {
        Self {
            pairs
        }
    }
}

impl Module for SwapModule {
    fn process(&mut self, input: char, dir: &Direction) -> Option<char> {
        Some(match dir {
            Direction::Forward =>
                if let Some(pair) = self.pairs.iter().find(|&p| p.0 == input) {
                    Some(pair.1)
                } else {
                    None
                }
            Direction::Backward =>
                if let Some(pair) = self.pairs.iter().find(|&p| p.1 == input) {
                    Some(pair.0)
                } else {
                    None
                }
        }.unwrap_or(input))
    }

    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_module() {
        let module = SwapModule::with_pairs(vec!(('A', 'B')));
        assert_eq!(Some('B'), module.process('A', Direction::Forward));
        assert_eq!(Some('B'), module.process('B', Direction::Forward));
        assert_eq!(Some('A'), module.process('A', Direction::Backward));
        assert_eq!(Some('A'), module.process('B', Direction::Backward));
    }

    #[test]
    fn logging_module() {
        assert_eq!('A', LoggingModule::new().process('A', Direction::Forward).unwrap());
    }
}
