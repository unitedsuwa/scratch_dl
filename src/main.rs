extern crate scratch_dl;
// use scratch_dl::logic_circuit::LogicCircuit;
use scratch_dl::tensor::*;

fn main() {
    let vec1 = vec![2.0, 2.0];
    let vec2 = vec![1.0, 3.0];
    let ten1 = TensorAtom::new(vec1);
    let ten2 = TensorAtom::new(vec2);
    let ten_result = TensorAtom::dot(&ten1, &ten2);
    println!("Tensor: {:?}", ten_result);
    println!("ten1: {:?}", ten1);
    let ten3 = TensorAtom::new(vec![1.0, 0.0, -1.0, 1.0]);
    let step_ten = step_func(&ten3);
    println!("step_ten: {:?}", step_ten);
    let ten4 = TensorAtom::new(vec![-1.0, 1.0, 2.0]);
    let sig_ten = sigmoid(&ten4);
    print!("sig_ten: {:?}", sig_ten);
    let ten5 = TensorAtom::new(vec![-1.0, 1.0, 2.0, 0.0, -3.0, 3.0]);
    let relu_ten = relu(&ten5);
    print!("relu_ten: {:?}", relu_ten);
}
