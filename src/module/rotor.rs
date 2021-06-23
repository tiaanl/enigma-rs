use crate::module::Module;
use crate::Direction;

pub struct RotorModule {
    connections: [char; 26],
    position: u32,
}

impl RotorModule {
    pub fn new(connections: [char; 26], position: u32) -> Self {
        Self {
            connections,
            position,
        }
    }
}

impl Module for RotorModule {
    fn process(&mut self, input: char, _: &Direction) -> Option<char> {
        /*
        let input_index = input as usize - 'A' as usize;

        let value = match dir {
            Direction::Forward => {
                let index = (input_index + self.position) % 26;
                Some(self.connections[index])
            }

            Direction::Backward => {
                if let some(found_index) = self.connections.iter().enumerate().find_map(|&p| if p.1 == input { Some(p.0) } else { None }) {
                    Some(('A' as usize + found_index) as char)
                }
            }
        };

        self.position += 1;
        if self.position >= 26 {
            self.position = 26;
        }

        value
        */
        Some(input)
    }

    fn reset(&mut self) {}
}
