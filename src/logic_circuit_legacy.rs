extern crate ndarray;
use ndarray::prelude::*;


struct LogicCalculator {
    y1: f32,
    y2: f32,
    theta: f32,
}

impl LogicCalculator {
    fn new(input_y1: f32, input_y2: f32, input_theta: f32) -> Self {
        Self {
            y1: input_y1,
            y2: input_y2,
            theta: input_theta,
        }
    }
    fn eval(self, x1: f32, x2: f32) -> f32 {
        let input_array: Array1<f32> = arr1(&[x1, x2]);
        let para_array: Array1<f32> = arr1(&[self.y1, self.y2]);

        let tmp = (input_array * para_array).sum() + self.theta;
        if tmp <= 0.0 {
            0.
        } else {
            1.
        }
    }
}

struct Circuit {}

impl Circuit {
    fn and() -> LogicCalculator {
        LogicCalculator::new(0.5, 0.5, -0.7)
    }
    fn nand() -> LogicCalculator {
        LogicCalculator::new(-0.5, -0.5, 0.7)
    }
    fn or() -> LogicCalculator {
        LogicCalculator::new(0.5, 0.5, -0.2)
    }
}


struct LogicCircuit {}

impl LogicCircuit {
    fn and(x1: f32, x2: f32) -> f32 {
        let calculator = Circuit::and();
        calculator.eval(x1, x2)
    }
    fn nand(x1: f32, x2: f32) -> f32 {
        let calculator = Circuit::nand();
        calculator.eval(x1, x2)
    }
    fn or(x1: f32, x2: f32) -> f32 {
        let calculator = Circuit::or();
        calculator.eval(x1, x2)
    }
    fn xor(x1: f32, x2: f32) -> f32 {
        let or = Self::or(x1, x2);
        let nand = Self::nand(x1, x2);
        Self::and(or, nand)
    }
}

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
