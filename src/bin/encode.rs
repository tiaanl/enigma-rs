extern crate enigma;

use enigma::module::{SwapModule, RotorModule, ValidationModule, LoggingModule};
use enigma::machine::Machine;
use enigma::Direction;

fn add_enigma_modules(machine: &mut Machine) {
    machine.add_module(Box::new(ValidationModule::new()));
    machine.add_module(Box::new(LoggingModule::with_label("input")));

    machine.add_module(Box::new(SwapModule::with_pairs(vec! {('H', 'P')})));

    let connections = [
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    ];
    machine.add_module(Box::new(RotorModule::new(connections, 0)));

    // machine.add_module(Box::new(LoggingModule::with_label("output")));
}

fn main() {
    let source = "HELLO";
    println!("source: {}", source);

    let mut encode_machine = Machine::new();
    add_enigma_modules(&mut encode_machine);

    let encoded = encode_machine.process(source, &Direction::Forward {});
    println!("encoded: {}", &encoded);

    let mut decode_machine = Machine::new();
    add_enigma_modules(&mut decode_machine);

    let decoded = decode_machine.process(encoded.as_str(), &Direction::Backward {});
    println!("original: {}", &decoded);

    assert_eq!(source, decoded);
}
