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
    fn process(&mut self, input: char, dir: &Direction) -> Option<char> {
        let value = match dir {
            Direction::Forward => {
                let input_index = (input as u32 - 'A' as u32 + self.position) % 26;
                Some(self.connections[input_index as usize])
            }

            Direction::Backward => {
                let input_index = self.connections.iter().enumerate()
                    .find(|p| )

                let input_index = (input as u32 - 'A' as u32 + self.position) % 26;
                Some(self.connections[input_index as usize])
            }
        };

        self.position += 1;
        if self.position >= 26 {
            self.position = 26;
        }

        value
    }

    fn reset(&mut self) {}
}
