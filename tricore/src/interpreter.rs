use crate::ast::*;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum TruthValue { True, False, Unknown }

pub struct Fact {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub truth: TruthValue,
}

impl Fact {
    pub fn new(subject: String, predicate: String, object: String) -> Self {
        Self { subject, predicate, object, truth: TruthValue::True }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableKB {
    facts: Vec<(String, String, String)>,
    rules: Vec<(Vec<(String, String, String)>, (String, String, String))>,
}

pub struct KnowledgeBase {
    facts: Vec<Fact>,
    rules: Vec<(Vec<(String, String, String)>, (String, String, String))>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self { facts: Vec::new(), rules: Vec::new() }
    }

    pub fn add_fact(&mut self, s: String, p: String, o: String) {
        let (truth, pred) = if p.starts_with("không_") {
            (TruthValue::False, p[6..].to_string())
        } else {
            (TruthValue::True, p)
        };
        self.facts.push(Fact { subject: s, predicate: pred, object: o, truth });
    }

    pub fn add_rule(&mut self, conditions: Vec<(String, String, String)>, conclusion: (String, String, String)) {
        self.rules.push((conditions, conclusion));
    }

    pub fn query(&self, s: &str, p: &str, o: &str) -> Vec<(String, String, String)> {
        let mut results = Vec::new();
        for fact in &self.facts {
            if fact.truth == TruthValue::True && self.match_fact(fact, s, p, o) {
                results.push((fact.subject.clone(), fact.predicate.clone(), fact.object.clone()));
            }
        }
        if p == "là" || p == "?" {
            let mut ancestors: HashMap<String, HashSet<String>> = HashMap::new();
            for fact in &self.facts {
                if fact.predicate == "là" && fact.truth == TruthValue::True {
                    ancestors.entry(fact.subject.clone()).or_default().insert(fact.object.clone());
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
                if s == "?" || s == entity {
                    for parent in parents {
                        let inherited = (entity.clone(), "là".to_string(), parent.clone());
                        if !results.contains(&inherited) {
                            results.push(inherited);
                        }
                    }
                }
            }
        }
        for (conditions, conclusion) in &self.rules {
            if conditions.len() == 1 {
                let (var, pred, value) = &conditions[0];
                if pred == "là" && var.starts_with(|c: char| c.is_uppercase()) {
                    let mut matching = Vec::new();
                    for fact in &self.facts {
                        if fact.predicate == "là" && fact.object == *value && fact.truth == TruthValue::True {
                            matching.push(fact.subject.clone());
                        }
                    }
                    for entity in matching {
                        let inferred_s = entity;
                        let inferred_p = conclusion.1.clone();
                        let inferred_o = conclusion.2.clone();
                        let inferred = (inferred_s, inferred_p, inferred_o);
                        if self.match_triple(&inferred, s, p, o) && !results.contains(&inferred) {
                            results.push(inferred);
                        }
                    }
                }
            }
        }
        results
    }

    pub fn check_condition(&self, condition: &(String, String, String)) -> bool {
        let results = self.query(&condition.0, &condition.1, &condition.2);
        !results.is_empty()
    }

    fn match_fact(&self, fact: &Fact, s: &str, p: &str, o: &str) -> bool {
        (s == "?" || s == fact.subject) &&
        (p == "?" || p == fact.predicate) &&
        (o == "?" || o == fact.object) &&
        fact.truth == TruthValue::True
    }

    fn match_triple(&self, triple: &(String, String, String), s: &str, p: &str, o: &str) -> bool {
        (s == "?" || s == triple.0) &&
        (p == "?" || p == triple.1) &&
        (o == "?" || o == triple.2)
    }

    pub fn luu(&self, path: &str) -> Result<(), String> {
        let facts: Vec<(String, String, String)> = self.facts.iter()
            .filter(|f| f.truth == TruthValue::True)
            .map(|f| (f.subject.clone(), f.predicate.clone(), f.object.clone()))
            .collect();
        let skb = SerializableKB { facts, rules: self.rules.clone() };
        let json = serde_json::to_string_pretty(&skb).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn nap(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let skb: SerializableKB = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        self.facts.clear();
        for (s, p, o) in skb.facts {
            self.add_fact(s, p, o);
        }
        self.rules = skb.rules;
        Ok(())
    }
}

pub struct Interpreter {
    pub kb: KnowledgeBase,
    pub output: Vec<String>,
    pub variables: HashMap<String, String>,
    pub functions: HashMap<String, Ham>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            kb: KnowledgeBase::new(),
            output: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn run(&mut self, statements: &[Statement]) -> Vec<String> {
        self.output.clear();
        for stmt in statements {
            self.execute(stmt);
        }
        self.output.clone()
    }

    fn execute(&mut self, stmt: &Statement) {
        match stmt {
            Statement::PhatBieu(p) => {
                if let Some(pred) = &p.dong_tu {
                    let o = p.tan_ngu.as_deref().unwrap_or("đúng");
                    self.kb.add_fact(p.chu_ngu.clone(), pred.clone(), o.to_string());
                }
            }
            Statement::Gan(g) => {
                let value = self.eval_bieu_thuc(&g.bieu_thuc);
                self.variables.insert(g.bien.clone(), value);
            }
            Statement::TraVe(t) => {
                let value = self.eval_bieu_thuc(&t.bieu_thuc);
                self.output.push(value);
            }
            Statement::Luat(l) => {
                self.kb.add_rule(l.dieu_kien.clone(), l.ket_luan.clone());
            }
            Statement::TruyVan(t) => {
                let (s, p, o) = &t.muc_tieu;
                let results = self.kb.query(s, p, o);
                if results.is_empty() {
                    self.output.push("Không tìm thấy.".into());
                } else {
                    for (rs, rp, ro) in results {
                        self.output.push(format!("{} {} {}.", rs, rp, ro));
                    }
                }
            }
            Statement::InRa(i) => {
                self.output.push(self.eval_bieu_thuc(&i.bieu_thuc));
            }
            Statement::IfElse(ie) => {
                let condition_met = self.kb.check_condition(&ie.dieu_kien);
                if condition_met {
                    for s in &ie.dung {
                        self.execute(s);
                    }
                } else if let Some(sai) = &ie.sai {
                    for s in sai {
                        self.execute(s);
                    }
                }
            }
            Statement::WhileLoop(wl) => {
                let mut iteration = 0;
                while self.kb.check_condition(&wl.dieu_kien) && iteration < 1000 {
                    for s in &wl.than {
                        self.execute(s);
                    }
                    iteration += 1;
                }
            }
            Statement::Ham(h) => {
                self.functions.insert(h.ten.clone(), h.clone());
            }
            Statement::ChuongTrinh(c) => {
                for s in &c.than {
                    self.execute(s);
                }
            }
            _ => {}
        }
    }

    fn eval_bieu_thuc(&self, expr: &str) -> String {
        // Nếu là biến, tra cứu
        if let Some(val) = self.variables.get(expr.trim()) {
            return val.clone();
        }
        // Nếu là chuỗi (có dấu ngoặc kép)
        if expr.starts_with('"') && expr.ends_with('"') {
            return expr[1..expr.len()-1].to_string();
        }
        // Mặc định trả về biểu thức như cũ
        expr.to_string()
    }
}
