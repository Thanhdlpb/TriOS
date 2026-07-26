use crate::ast::*;
use std::collections::HashMap;

pub struct KnowledgeBase {
    facts: Vec<(String, String, String)>, // (chủ, động, tân)
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self { facts: Vec::new() }
    }

    pub fn add_fact(&mut self, s: String, p: String, o: String) {
        self.facts.push((s, p, o));
    }

    pub fn query(&self, s: &str, p: &str, o: &str) -> Vec<(String, String, String)> {
        self.facts.iter().filter(|(fs, fp, fo)| {
            let s_match = s == "?" || s == fs;
            let p_match = p == "?" || p == fp;
            let o_match = o == "?" || o == fo;
            s_match && p_match && o_match
        }).cloned().collect()
    }
}

pub struct Interpreter {
    kb: KnowledgeBase,
    variables: HashMap<String, String>,
    output: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            kb: KnowledgeBase::new(),
            variables: HashMap::new(),
            output: Vec::new(),
        }
    }

    pub fn run(&mut self, statements: &[Statement]) -> Vec<String> {
        for stmt in statements {
            self.execute(stmt);
        }
        self.output.clone()
    }

    fn execute(&mut self, stmt: &Statement) {
        match stmt {
            Statement::PhatBieu(p) => {
                let s = &p.chu_ngu;
                if let Some(pred) = &p.dong_tu {
                    if let Some(obj) = &p.tan_ngu {
                        self.kb.add_fact(s.clone(), pred.clone(), obj.clone());
                    }
                }
            }
            Statement::Luat(l) => {
                // Lưu luật vào KB như một fact đặc biệt (tạm thời chưa suy luận)
                let s = format!("luat_{}", self.kb.facts.len());
                let p = "la_luat".to_string();
                let o = format!("{:?}", l);
                self.kb.add_fact(s, p, o);
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
                self.output.push(i.bieu_thuc.clone());
            }
            Statement::VongLap(v) => {
                for item in &v.danh_sach {
                    self.variables.insert(v.bien.clone(), item.clone());
                    for st in &v.than {
                        self.execute(st);
                    }
                }
            }
            Statement::Ham(_h) => {
                // Tạm thời chưa hỗ trợ gọi hàm
            }
            Statement::ChuongTrinh(c) => {
                for st in &c.than {
                    self.execute(st);
                }
            }
        }
    }
}
