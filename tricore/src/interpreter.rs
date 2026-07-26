use crate::ast::*;
use std::collections::{HashMap, HashSet};

pub struct KnowledgeBase {
    facts: Vec<(String, String, String)>,       // (chủ, động, tân)
    rules: Vec<(Vec<(String, String, String)>, (String, String, String))>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self { facts: Vec::new(), rules: Vec::new() }
    }

    pub fn add_fact(&mut self, s: String, p: String, o: String) {
        self.facts.push((s, p, o));
    }

    pub fn add_rule(&mut self, conditions: Vec<(String, String, String)>, conclusion: (String, String, String)) {
        self.rules.push((conditions, conclusion));
    }

    // Truy vấn an toàn, không đệ quy phức tạp
    pub fn query(&self, s: &str, p: &str, o: &str) -> Vec<(String, String, String)> {
        let mut results = Vec::new();

        // 1. Truy vấn trực tiếp từ facts
        for fact in &self.facts {
            if self.match_triple(fact, s, p, o) {
                results.push(fact.clone());
            }
        }

        // 2. Kế thừa: nếu X là Y và Y là Z thì X là Z (dùng tập hợp để tránh lặp)
        if p == "là" || p == "?" {
            let mut ancestors: HashMap<String, HashSet<String>> = HashMap::new();
            // Thu thập tất cả quan hệ "là"
            for (sub, pred, obj) in &self.facts {
                if pred == "là" {
                    ancestors.entry(sub.clone()).or_default().insert(obj.clone());
                }
            }
            // Đóng bắc cầu (transitive closure) an toàn bằng vòng lặp giới hạn
            let mut changed = true;
            let mut iteration = 0;
            while changed && iteration < 1000 {
                changed = false;
                iteration += 1;
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

            // Trả về kết quả kế thừa
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

        // 3. Áp dụng luật một cách an toàn: với mỗi luật, tìm các thực thể thỏa mãn điều kiện
        //    và thêm trực tiếp fact kết luận vào kết quả nếu khớp truy vấn.
        for (conditions, conclusion) in &self.rules {
            // Chỉ hỗ trợ luật đơn giản: 1 điều kiện dạng X là Y
            if conditions.len() == 1 {
                let (var, pred, value) = &conditions[0];
                if pred == "là" && var.starts_with(|c: char| c.is_uppercase()) {
                    // var là biến (ví dụ X)
                    // Tìm tất cả các thực thể thỏa mãn "là value"
                    let mut matching_entities = Vec::new();
                    for (sub, p, obj) in &self.facts {
                        if p == "là" && obj == value {
                            matching_entities.push(sub.clone());
                        }
                    }
                    // Cũng tìm qua kế thừa (tổ tiên)
                    // Để đơn giản, chỉ dùng facts trực tiếp
                    for entity in matching_entities {
                        let inferred_s = entity.clone();
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

    fn match_triple(&self, triple: &(String, String, String), s: &str, p: &str, o: &str) -> bool {
        (s == "?" || s == triple.0) &&
        (p == "?" || p == triple.1) &&
        (o == "?" || o == triple.2)
    }
}

pub struct Interpreter {
    kb: KnowledgeBase,
    output: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { kb: KnowledgeBase::new(), output: Vec::new() }
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
                if let Some(pred) = &p.dong_tu {
                    let o = p.tan_ngu.as_deref().unwrap_or("đúng");
                    self.kb.add_fact(p.chu_ngu.clone(), pred.clone(), o.to_string());
                }
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
            Statement::InRa(i) => { self.output.push(i.bieu_thuc.clone()); }
            Statement::VongLap(_) => {} // chưa hỗ trợ
            Statement::Ham(_) => {}
            Statement::ChuongTrinh(c) => {
                for st in &c.than {
                    self.execute(st);
                }
            }
        }
    }
}
