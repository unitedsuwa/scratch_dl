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
        let input_array = &[x1, x2];
        let para_array = &[self.y1, self.y2];
        let tmp = input_array.iter()
                             .zip(para_array)
                             .map(|(x, y)| x*y)
                             .fold(0., |acc, x| acc + x)
                             + self.theta;

        if tmp <= 0.0 {
            0.
        } else {
            1.
        }
    }
}

enum Logic {
    AND,
    NAND,
    OR
}

fn logic_type(logic: Logic) -> LogicCalculator {
    match logic {
        Logic::AND => LogicCalculator::new(0.5, 0.5, -0.7),
        Logic::NAND => LogicCalculator::new(-0.5, -0.5, 0.7),
        Logic::OR => LogicCalculator::new(0.5, 0.5, -0.2)
    }
}

pub struct LogicCircuit;
impl LogicCircuit {
    fn eval(logic: Logic, x1: f32, x2: f32) -> f32 {
        logic_type(logic).eval(x1, x2)
        }
    pub fn and(x1: f32, x2: f32) -> f32 {
        let logic = Logic::AND;
        Self::eval(logic, x1, x2)
    }
    pub fn nand(x1: f32, x2: f32) -> f32 {
        let logic = Logic::NAND;
        Self::eval(logic, x1, x2)
    }
    pub fn or(x1: f32, x2: f32) -> f32 {
        let logic = Logic::OR;
        Self::eval(logic, x1, x2)
    }
    pub fn xor(x1: f32, x2: f32) -> f32 {
        let or = Self::or(x1, x2);
        let nand = Self::nand(x1, x2);
        Self::and(or, nand)
    }
}