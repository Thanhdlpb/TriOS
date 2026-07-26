use crate::ml::linear::LinearRegression;
use ndarray::Array2;

pub struct AIRuntime {
    models: Vec<Box<dyn AIModel>>,
}

pub trait AIModel {
    fn train(&mut self, data: &str) -> Result<(), String>;
    fn predict(&self, input: &str) -> Result<String, String>;
    fn name(&self) -> &str;
}

impl AIRuntime {
    pub fn new() -> Self { Self { models: Vec::new() } }
    pub fn register_model(&mut self, model: Box<dyn AIModel>) { self.models.push(model); }
    pub fn train(&mut self, model_name: &str, data: &str) -> Result<(), String> {
        for model in &mut self.models {
            if model.name() == model_name { return model.train(data); }
        }
        Err(format!("Không tìm thấy mô hình '{}'", model_name))
    }
    pub fn predict(&self, model_name: &str, input: &str) -> Result<String, String> {
        for model in &self.models {
            if model.name() == model_name { return model.predict(input); }
        }
        Err(format!("Không tìm thấy mô hình '{}'", model_name))
    }
}

pub struct SimpleLinearModel {
    name: String,
    model: LinearRegression,
}

impl SimpleLinearModel {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), model: LinearRegression::new(1, 0.01) }
    }
}

impl AIModel for SimpleLinearModel {
    fn train(&mut self, data: &str) -> Result<(), String> {
        let points: Vec<Vec<f64>> = data.split(';')
            .map(|s| s.split(',').filter_map(|v| v.trim().parse().ok()).collect())
            .collect();
        let mut x_vals = Vec::new();
        let mut y_vals = Vec::new();
        for p in &points {
            if p.len() >= 2 { x_vals.push(p[0]); y_vals.push(p[1]); }
        }
        let x = Array2::from_shape_vec((points.len(), 1), x_vals).map_err(|e| e.to_string())?;
        let y = ndarray::Array1::from_vec(y_vals);
        self.model.train(&x, &y, 1000);
        Ok(())
    }
    fn predict(&self, input: &str) -> Result<String, String> {
        let x: f64 = input.trim().parse().map_err(|_| "Đầu vào không hợp lệ".to_string())?;
        let x_arr = Array2::from_shape_vec((1, 1), vec![x]).map_err(|e| e.to_string())?;
        Ok(format!("{}", self.model.predict(&x_arr)[0]))
    }
    fn name(&self) -> &str { &self.name }
}

impl AIRuntime {
    pub fn has_model(&self, name: &str) -> bool {
        self.models.iter().any(|m| m.name() == name)
    }
}
