extern crate scratch_dl;
use scratch_dl::logic_circuit::LogicCircuit;


fn main() {
    let x1: f32 = 1.0;
    let x2: f32 = 1.0;
    let logic_and = LogicCircuit::and(x1, x2);
    let logic_nand = LogicCircuit::nand(x1, x2);
    let logic_or = LogicCircuit::or(x1, x2);
    let logic_xor = LogicCircuit::xor(x1, x2);
    println!("AND: {}", logic_and);
    println!("NAND: {}", logic_nand);
    println!("OR {}", logic_or);
    println!("XOR {}", logic_xor);
}
