use crate::ast::*;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs;

pub struct Interpreter {
    variables: HashMap<String, f64>,
    string_vars: HashMap<String, String>,
    facts: Vec<(String, String, String)>,
    rules: Vec<(Vec<Condition>, Condition, RuleRelation)>,
    output: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            string_vars: HashMap::new(),
            facts: Vec::new(),
            rules: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn run(&mut self, stmts: &[Statement]) -> Vec<String> {
        self.output.clear();
        for stmt in stmts {
            self.execute(stmt);
        }
        self.output.clone()
    }

    fn execute(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Fact {
                subject,
                predicate,
                object,
                ..
            } => {
                self.facts
                    .push((subject.clone(), predicate.clone(), object.clone()));
            }
            Statement::Rule {
                condition,
                conclusion,
                relation,
            } => {
                self.rules
                    .push((condition.clone(), conclusion.clone(), relation.clone()));
            }
            Statement::Query {
                subject,
                predicate,
                object,
                ..
            } => {
                let mut results = self.query(subject, predicate, object);
                results.sort();
                results.dedup();
                if results.is_empty() {
                    self.output.push("Không tìm thấy.".to_string());
                } else {
                    for r in results {
                        self.output.push(r);
                    }
                }
            }
            Statement::Print(expr) => {
                let val = self.eval_to_string(expr);
                self.output.push(val);
            }
            Statement::Assign { name, value } => match value {
                Expression::String(s) => {
                    self.string_vars.insert(name.clone(), s.clone());
                }
                _ => {
                    self.variables.insert(name.clone(), self.eval(value));
                }
            },
            Statement::IfElse {
                condition,
                then_body,
                else_body,
            } => {
                if self.eval(condition) != 0.0 {
                    for s in then_body {
                        self.execute(s);
                    }
                } else if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.execute(s);
                    }
                }
            }
            Statement::ForLoop {
                var,
                start,
                end,
                body,
            } => {
                let s = self.eval(start) as i64;
                let e = self.eval(end) as i64;
                for i in s..=e {
                    self.variables.insert(var.clone(), i as f64);
                    for stmt in body {
                        self.execute(stmt);
                    }
                }
            }
            Statement::ChuongTrinh { body, .. } => {
                for s in body {
                    self.execute(s);
                }
            }
            Statement::UseModule { path } => match fs::read_to_string(path) {
                Ok(source) => {
                    use crate::lexer::Lexer;
                    use crate::parser::Parser;
                    let mut lexer = Lexer::new(&source);
                    let mut tokens = Vec::new();
                    loop {
                        let token = lexer.next_token();
                        let is_eof = token.kind == crate::token::TokenKind::EOF;
                        tokens.push(token);
                        if is_eof {
                            break;
                        }
                    }
                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        Ok(module_stmts) => {
                            for s in module_stmts {
                                self.execute(&s);
                            }
                        }
                        Err(e) => self.output.push(format!("Lỗi module '{}': {}", path, e)),
                    }
                }
                Err(e) => self
                    .output
                    .push(format!("Không thể đọc module '{}': {}", path, e)),
            },
            _ => {}
        }
    }

    fn query(&self, subject: &str, predicate: &str, object: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut inferred = HashSet::new();

        for (s, p, o) in &self.facts {
            if self.match_term(subject, s)
                && self.match_term(predicate, p)
                && self.match_term(object, o)
            {
                results.push(format!("{} {} {}", s, p, o));
            }
        }

        for (conditions, conclusion, _) in &self.rules {
            if let Condition::Simple {
                subject: concl_s,
                predicate: concl_p,
                object: concl_o,
            } = conclusion
            {
                let bindings = self.find_bindings(conditions);
                for binding in bindings {
                    let inferred_s = self.apply_binding(concl_s, &binding);
                    let inferred_p = self.apply_binding(concl_p, &binding);
                    let inferred_o = self.apply_binding(concl_o, &binding);

                    if self.match_term(subject, &inferred_s)
                        && self.match_term(predicate, &inferred_p)
                        && self.match_term(object, &inferred_o)
                    {
                        let result = format!("{} {} {}", inferred_s, inferred_p, inferred_o);
                        if !inferred.contains(&result) {
                            inferred.insert(result.clone());
                            results.push(result);
                        }
                    }
                }
            }
        }

        if predicate == "là" || predicate == "?" {
            let mut ancestors: HashMap<String, HashSet<String>> = HashMap::new();
            for (s, p, o) in &self.facts {
                if p == "là" {
                    ancestors.entry(s.clone()).or_default().insert(o.clone());
                }
            }
            let mut changed = true;
            let mut iter = 0;
            while changed && iter < 1000 {
                changed = false;
                iter += 1;
                let keys: Vec<String> = ancestors.keys().cloned().collect();
                for key in keys {
                    let mut new_parents = HashSet::new();
                    if let Some(parents) = ancestors.get(&key) {
                        for parent in parents.clone() {
                            if let Some(grandparents) = ancestors.get(&parent) {
                                for gp in grandparents {
                                    if !ancestors.get(&key).map_or(true, |v| !v.contains(gp)) {
                                        new_parents.insert(gp.clone());
                                    }
                                }
                            }
                        }
                    }
                    for np in new_parents {
                        if ancestors.entry(key.clone()).or_default().insert(np.clone()) {
                            changed = true;
                        }
                    }
                }
            }
            for (entity, parents) in &ancestors {
                if self.match_term(subject, entity) {
                    for parent in parents {
                        let result = format!("{} là {}", entity, parent);
                        if !results.contains(&result) {
                            results.push(result);
                        }
                    }
                }
            }
        }

        results
    }

    fn find_bindings(&self, conditions: &Vec<Condition>) -> Vec<HashMap<String, String>> {
        let mut all_bindings: Vec<HashMap<String, String>> = vec![HashMap::new()];
        for condition in conditions {
            let mut new_bindings = Vec::new();
            match condition {
                Condition::Simple {
                    subject,
                    predicate,
                    object,
                } => {
                    for (s, p, o) in &self.facts {
                        if p == predicate {
                            for existing in &all_bindings {
                                let mut binding = existing.clone();
                                if self.unify(subject, s, &mut binding)
                                    && self.unify(object, o, &mut binding)
                                {
                                    new_bindings.push(binding);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            all_bindings = new_bindings;
        }
        all_bindings
    }

    fn unify(
        &self,
        var_or_value: &str,
        fact_value: &str,
        binding: &mut HashMap<String, String>,
    ) -> bool {
        if var_or_value
            .chars()
            .next()
            .map_or(false, |c| c.is_uppercase())
        {
            if let Some(existing) = binding.get(var_or_value) {
                existing == fact_value
            } else {
                binding.insert(var_or_value.to_string(), fact_value.to_string());
                true
            }
        } else {
            var_or_value == fact_value
        }
    }

    fn apply_binding(&self, term: &str, binding: &HashMap<String, String>) -> String {
        if term.chars().next().map_or(false, |c| c.is_uppercase()) {
            binding
                .get(term)
                .cloned()
                .unwrap_or_else(|| term.to_string())
        } else {
            term.to_string()
        }
    }

    fn match_term(&self, pattern: &str, value: &str) -> bool {
        pattern == "?"
            || pattern == value
            || (pattern.chars().next().map_or(false, |c| c.is_uppercase()))
    }

    fn eval(&self, expr: &Expression) -> f64 {
        match expr {
            Expression::Number(n) => *n,
            Expression::String(_) => 0.0,
            Expression::Variable(name) => self.variables.get(name).copied().unwrap_or(0.0),
            Expression::Add(a, b) => self.eval(a) + self.eval(b),
            Expression::Sub(a, b) => self.eval(a) - self.eval(b),
            Expression::Mul(a, b) => self.eval(a) * self.eval(b),
            Expression::Div(a, b) => {
                self.eval(a)
                    / if self.eval(b) == 0.0 {
                        1.0
                    } else {
                        self.eval(b)
                    }
            }
            Expression::Gt(a, b) => {
                if self.eval(a) > self.eval(b) {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Lt(a, b) => {
                if self.eval(a) < self.eval(b) {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Eq(a, b) => {
                if (self.eval(a) - self.eval(b)).abs() < 0.001 {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    fn eval_to_string(&self, expr: &Expression) -> String {
        match expr {
            Expression::String(s) => s.clone(),
            Expression::Variable(name) => {
                if let Some(s) = self.string_vars.get(name) {
                    return s.clone();
                }
                if let Some(v) = self.variables.get(name) {
                    return if *v == (*v as i64) as f64 {
                        format!("{}", *v as i64)
                    } else {
                        format!("{}", v)
                    };
                }
                "0".to_string()
            }
            Expression::Add(a, b) => {
                let left = self.eval_to_string(a);
                let right = self.eval_to_string(b);
                format!("{}{}", left, right)
            }
            _ => {
                let v = self.eval(expr);
                if v == (v as i64) as f64 {
                    format!("{}", v as i64)
                } else {
                    format!("{}", v)
                }
            }
        }
    }

    pub fn luu(&self, path: &str) -> Result<(), String> {
        let data = serde_json::json!({ "facts": self.facts });
        std::fs::write(path, data.to_string()).map_err(|e| e.to_string())
    }

    pub fn nap(&mut self, path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let data: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        self.facts.clear();
        if let Some(facts) = data["facts"].as_array() {
            for fact in facts {
                if let Some(arr) = fact.as_array() {
                    if arr.len() == 3 {
                        self.facts.push((
                            arr[0].as_str().unwrap_or("").to_string(),
                            arr[1].as_str().unwrap_or("").to_string(),
                            arr[2].as_str().unwrap_or("").to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
