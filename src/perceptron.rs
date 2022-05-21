use ndarray::Array1;

pub struct Perceptron {
    weight: Array1<f64>,
    bias: f64,
}

impl Perceptron {
    fn check_rep(&self) {}

    pub fn new(input_len: usize, weight: Option<Array1<f64>>, bias: Option<f64>) -> Self {
        let weight = match weight {
            Some(x) => {
                assert_eq!(x.shape()[0], input_len);
                x
            }
            None => Array1::<f64>::zeros(input_len),
        };
        let bias = match bias {
            Some(x) => x,
            None => 0.0,
        };
        let this = Perceptron { weight, bias };
        this.check_rep();
        this
    }

    pub fn predict(&self, input: &Array1<f64>) -> bool {
        input.dot(&self.weight) > self.bias
    }

    pub fn train_and_is_correct(&mut self, input: Array1<f64>, label: bool) -> bool {
        let predict = self.predict(&input);
        match (predict, label) {
            (true, false) => {
                self.weight -= &input;
                false
            }
            (false, true) => {
                self.weight += &input;
                false
            }
            _ => true,
        }
    }

    #[inline]
    pub fn weight(&self) -> &Array1<f64> {
        &self.weight
    }

    #[inline]
    pub fn bias(&self) -> f64 {
        self.bias
    }
}
