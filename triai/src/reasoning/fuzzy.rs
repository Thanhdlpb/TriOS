use std::collections::HashMap;

pub struct FuzzySystem {
    sets: HashMap<String, fn(f64) -> f64>,
    rules: Vec<(Vec<(String, String)>, (String, String))>,
}

impl FuzzySystem {
    pub fn new() -> Self {
        Self {
            sets: HashMap::new(),
            rules: Vec::new(),
        }
    }

    pub fn add_set(&mut self, name: &str, membership: fn(f64) -> f64) {
        self.sets.insert(name.to_string(), membership);
    }

    pub fn add_rule(&mut self, conditions: Vec<(String, String)>, conclusion: (String, String)) {
        self.rules.push((conditions, conclusion));
    }

    pub fn evaluate(&self, inputs: &HashMap<String, f64>) -> HashMap<String, f64> {
        let mut outputs = HashMap::new();
        for (conditions, conclusion) in &self.rules {
            let mut min_degree = 1.0f64;
            for (var, set_name) in conditions {
                if let Some(value) = inputs.get(var) {
                    if let Some(membership_fn) = self.sets.get(set_name) {
                        min_degree = min_degree.min(membership_fn(*value));
                    }
                }
            }
            if min_degree > 0.0 {
                outputs.insert(conclusion.0.clone(), min_degree);
            }
        }
        outputs
    }
}
