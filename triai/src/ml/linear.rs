use ndarray::{Array1, Array2};
use rand::Rng;

pub struct LinearRegression {
    weights: Array1<f64>,
    bias: f64,
    learning_rate: f64,
}

impl LinearRegression {
    pub fn new(n_features: usize, learning_rate: f64) -> Self {
        let mut rng = rand::thread_rng();
        let weights = Array1::from_vec((0..n_features).map(|_| rng.gen_range(-1.0..1.0)).collect());
        Self { weights, bias: rng.gen_range(-1.0..1.0), learning_rate }
    }

    pub fn predict(&self, x: &Array2<f64>) -> Array1<f64> {
        x.dot(&self.weights) + self.bias
    }

    pub fn train(&mut self, x: &Array2<f64>, y: &Array1<f64>, epochs: usize) {
        let n_samples = x.nrows() as f64;
        for _ in 0..epochs {
            let y_pred = self.predict(x);
            let error = &y_pred - y;
            let dw = x.t().dot(&error) / n_samples;
            let db = error.sum() / n_samples;
            self.weights = &self.weights - self.learning_rate * dw;
            self.bias -= self.learning_rate * db;
        }
    }
}
