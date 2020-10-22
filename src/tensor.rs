#[derive(Debug)]
pub struct TensorAtom {
    ten: Vec<f64>,
    len: usize,
}

impl TensorAtom {
    pub fn new(input_vec: Vec<f64>) -> Self {
        let len = input_vec.len();
        Self {
            ten: input_vec,
            len,
        }
    }

    pub fn dot(tensor_x: &TensorAtom, tensor_y: &TensorAtom) -> TensorAtom {
        assert_eq!(tensor_x.len, tensor_y.len); // TODO: Error handling with Result type

        let input_ten = &tensor_x.ten;
        let para_ten = &tensor_y.ten;
        assert!(input_ten.len() == para_ten.len());
        let result = input_ten
            .iter()
            .zip(para_ten)
            .map(|(x, y)| x * y)
            .fold(0., |acc, x| acc + x);
        Self::new(vec![result])
    }
}

pub fn step_func(input: &TensorAtom) -> TensorAtom {
    let result = input
        .ten
        .iter()
        .map(|&f| if f > 0. { 1. } else { 0. })
        .collect();
    TensorAtom::new(result)
}

pub fn sigmoid(input: &TensorAtom) -> TensorAtom {
    let result = input
        .ten
        .iter()
        .map(|&f| 1.0 / (1.0 + (-f).exp()))
        .collect();
    TensorAtom::new(result)
}

pub fn relu(input: &TensorAtom) -> TensorAtom {
    let result = input
        .ten
        .iter()
        .map(|&f| if f > 0. { f } else { 0. })
        .collect();
    TensorAtom::new(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tensoratom_construct() {
        let vec1 = vec![2.0, 2.0];
        let vec2 = vec1.clone();
        let ten_atom = TensorAtom::new(vec1);

        assert_eq!(ten_atom.ten, vec2);
        assert_eq!(ten_atom.len, 2);
    }

    #[test]
    fn tensoratom_dot() {
        let vec1 = vec![2.0, 2.0];
        let vec2 = vec![1.0, 3.0];
        let ten1 = TensorAtom::new(vec1);
        let ten2 = TensorAtom::new(vec2);

        let dot_result = TensorAtom::dot(&ten1, &ten2);

        assert_eq!(dot_result.ten, vec![8.0])
    }

    #[test]
    #[should_panic]
    fn tensoratom_dot_fail() {
        let vec1 = vec![2.0, 2.0, 4.0];
        let vec2 = vec![1.0, 3.0];
        let ten1 = TensorAtom::new(vec1);
        let ten2 = TensorAtom::new(vec2);

        TensorAtom::dot(&ten1, &ten2);
    }

    #[test]
    fn tensor_step() {
        let ten = TensorAtom::new(vec![1.0, 0.0, -1.0, 5.0]);
        let step_result = step_func(&ten);

        assert_eq!(step_result.ten, vec![1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn tensor_sigmoid() {
        let ten = TensorAtom::new(vec![-1.0, 1.0, 2.0]);
        let sig_result = sigmoid(&ten);

        assert_eq!(
            sig_result.ten,
            vec![0.2689414213699951, 0.7310585786300049, 0.8807970779778823]
        );
    }

    #[test]
    fn tensor_relu() {
        let ten = TensorAtom::new(vec![-1.0, 1.0, 2.0, 0.0, -3.0, 3.0]);
        let relu_result = relu(&ten);

        assert_eq!(relu_result.ten, vec![0.0, 1.0, 2.0, 0.0, 0.0, 3.0]);
    }
}
