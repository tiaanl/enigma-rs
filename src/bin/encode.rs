extern crate enigma;

use enigma::module::{SwapModule, RotorModule};
use enigma::machine::Enigma;
use enigma::Direction;

fn main() {
    let mut machine = Enigma::new();

    // machine.add_module(Box::new(LoggingModule::with_label("input")));

    let swap_module = SwapModule::with_pairs(vec! {('H', 'P')});
    machine.add_module(Box::new(swap_module));

    let connections = [
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    ];
    let fast_rotor = RotorModule::new(connections, 0);
    machine.add_module(Box::new(fast_rotor));

    // machine.add_module(Box::new(LoggingModule::with_label("output")));

    let source = "HELLO";

    println!("source: {}", source);

    let encoded = machine.process(source, &Direction::Forward {});
    println!("encoded: {}", &encoded);
    machine.reset();

    let decoded = machine.process(encoded.as_str(), &Direction::Backward {});
    println!("original: {}", &decoded);
    machine.reset();

    assert_eq!(source, decoded);
}
