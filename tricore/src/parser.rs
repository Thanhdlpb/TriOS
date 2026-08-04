use crate::ast::*;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn skip_dot(&mut self) {
        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::DauCham
                | TokenKind::DauHoi
                | TokenKind::DauThan
                | TokenKind::DauPhay => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn is_action_token(&self) -> bool {
        match self.peek() {
            Some(tok) => matches!(
                tok.kind,
                TokenKind::Ten(_)
                    | TokenKind::In
                    | TokenKind::Hoi
                    | TokenKind::Lap
                    | TokenKind::Ham
                    | TokenKind::Hay
                    | TokenKind::DungLenh
            ),
            None => false,
        }
    }

    fn expect_action(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                kind: TokenKind::Ten(s),
                ..
            }) => Ok(s),
            Some(Token {
                kind: TokenKind::In,
                ..
            }) => Ok("in".to_string()),
            Some(Token {
                kind: TokenKind::Hoi,
                ..
            }) => Ok("hỏi".to_string()),
            Some(Token {
                kind: TokenKind::Lap,
                ..
            }) => Ok("lặp".to_string()),
            Some(Token {
                kind: TokenKind::Ham,
                ..
            }) => Ok("hàm".to_string()),
            Some(Token {
                kind: TokenKind::Hay,
                ..
            }) => Ok("hãy".to_string()),
            Some(Token {
                kind: TokenKind::DungLenh,
                ..
            }) => Ok("đừng".to_string()),
            other => Err(format!("Mong đợi hành động nhưng gặp {:?}", other)),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmts = Vec::new();
        while self.peek().is_some() && !matches!(self.peek().unwrap().kind, TokenKind::EOF) {
            self.skip_dot();
            if self.peek().is_none() || matches!(self.peek().unwrap().kind, TokenKind::EOF) {
                break;
            }
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        self.skip_dot();
        let tok = self.peek().ok_or("Kết thúc file")?;
        match &tok.kind {
            TokenKind::DungModule => self.parse_use_module(), // <<< THÊM "dùng"
            TokenKind::ChuongTrinh => self
                .parse_chuong_trinh()
                .map(|(name, body)| Statement::ChuongTrinh { name, body }),
            TokenKind::In => self.parse_print(),
            TokenKind::Hay | TokenKind::DungLenh => self.parse_command(),
            TokenKind::Neu => self.parse_if_or_rule(),
            TokenKind::Lap => self.parse_for(),
            TokenKind::Ham => self.parse_function(),
            TokenKind::Hoi | TokenKind::CoPhai => self.parse_query(),
            TokenKind::Ten(ref s) if s == "in_ra" => self.parse_print(),
            TokenKind::Ten(_) => {
                if let Some(next) = self.peek_next() {
                    if next.kind == TokenKind::Gan {
                        return self.parse_assign();
                    }
                }
                self.parse_fact()
            }
            _ => Err(format!("Token không mong đợi: {:?}", tok)),
        }
    }

    fn parse_use_module(&mut self) -> Result<Statement, String> {
        self.advance(); // dùng
        let path = if let Some(Token {
            kind: TokenKind::Chuoi(s),
            ..
        }) = self.advance()
        {
            s
        } else {
            return Err("Mong đợi đường dẫn module dạng chuỗi".to_string());
        };
        self.skip_dot();
        Ok(Statement::UseModule { path })
    }

    fn parse_chuong_trinh(&mut self) -> Result<(String, Vec<Statement>), String> {
        self.advance(); // chương_trình
        let name = if let Some(Token {
            kind: TokenKind::Chuoi(s),
            ..
        }) = self.advance()
        {
            s
        } else {
            return Err("Mong đợi tên chương trình dạng chuỗi".to_string());
        };
        self.expect_kind(&TokenKind::BatDau)?;
        let mut body = Vec::new();
        loop {
            self.skip_dot();
            if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                break;
            }
            body.push(self.parse_statement()?);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        Ok((name, body))
    }

    // ... (giữ nguyên các hàm parse còn lại)
    fn parse_fact(&mut self) -> Result<Statement, String> {
        let mut negation = false;
        let mut tense = None;

        if let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::Khong | TokenKind::Chua | TokenKind::Chang => {
                    negation = true;
                    self.advance();
                }
                TokenKind::Da
                | TokenKind::Dang
                | TokenKind::Se
                | TokenKind::Vua
                | TokenKind::Sap => {
                    tense = Some(self.advance().unwrap().kind.clone());
                }
                _ => {}
            }
        }

        let subject = self.expect_action()?;

        if let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::Khong | TokenKind::Chua | TokenKind::Chang => {
                    negation = true;
                    self.advance();
                }
                TokenKind::Da
                | TokenKind::Dang
                | TokenKind::Se
                | TokenKind::Vua
                | TokenKind::Sap => {
                    tense = Some(self.advance().unwrap().kind.clone());
                }
                _ => {}
            }
        }

        let predicate = if self.check_kind(&TokenKind::La) {
            self.advance();
            "là".to_string()
        } else if self.is_action_token() {
            self.expect_action()?
        } else if self.peek().map_or(true, |t| {
            matches!(
                t.kind,
                TokenKind::DauCham | TokenKind::DauHoi | TokenKind::DauThan | TokenKind::EOF
            )
        }) {
            "là".to_string()
        } else {
            self.expect_action()?
        };

        let object = if self.peek().map_or(true, |t| {
            matches!(
                t.kind,
                TokenKind::DauCham | TokenKind::DauHoi | TokenKind::DauThan | TokenKind::EOF
            )
        }) {
            "đúng".to_string()
        } else {
            self.expect_value()?
        };

        let tense_str = tense.map(|t| format!("{:?}", t));
        self.skip_dot();
        Ok(Statement::Fact {
            subject,
            predicate,
            object,
            tense: tense_str,
            negation,
        })
    }

    fn parse_assign(&mut self) -> Result<Statement, String> {
        let name = self.expect_action()?;
        self.expect_kind(&TokenKind::Gan)?;
        let value = self.parse_expression()?;
        self.skip_dot();
        Ok(Statement::Assign { name, value })
    }

    fn parse_print(&mut self) -> Result<Statement, String> {
        self.advance(); // in
        let expr = self.parse_expression()?;
        self.skip_dot();
        Ok(Statement::Print(expr))
    }

    fn parse_query(&mut self) -> Result<Statement, String> {
        let mut question_type = QuestionType::What;
        if self.check_kind(&TokenKind::CoPhai) {
            question_type = QuestionType::YesNo;
            self.advance();
        } else if self.check_kind(&TokenKind::Hoi) {
            self.advance();
        }

        let subject = self.expect_action()?;
        let predicate = if self.check_kind(&TokenKind::La) {
            self.advance();
            "là".to_string()
        } else {
            self.expect_action()?
        };

        let object = if self.check_kind(&TokenKind::Gi) {
            question_type = QuestionType::What;
            self.advance();
            "?".to_string()
        } else if self.check_kind(&TokenKind::Dau) {
            question_type = QuestionType::Where;
            self.advance();
            "?".to_string()
        } else if self.is_action_token() {
            let t = self.expect_action()?;
            match t.as_str() {
                "gì" => {
                    question_type = QuestionType::What;
                    "?".to_string()
                }
                "đâu" => {
                    question_type = QuestionType::Where;
                    "?".to_string()
                }
                "ai" => {
                    question_type = QuestionType::Who;
                    "?".to_string()
                }
                "nào" => {
                    question_type = QuestionType::When;
                    "?".to_string()
                }
                _ => t,
            }
        } else if self.check_kind(&TokenKind::HayKhong) {
            question_type = QuestionType::YesNo;
            self.advance();
            "?".to_string()
        } else {
            "?".to_string()
        };

        self.skip_dot();
        Ok(Statement::Query {
            question_type,
            subject,
            predicate,
            object,
        })
    }

    fn parse_command(&mut self) -> Result<Statement, String> {
        let command_type = match self.peek().unwrap().kind {
            TokenKind::Hay => {
                self.advance();
                CommandType::Do
            }
            TokenKind::DungLenh => {
                self.advance();
                CommandType::Dont
            }
            _ => return Err("Lỗi lệnh".to_string()),
        };
        let action = self.expect_action()?;
        let target = if self.peek().is_some()
            && (self.is_action_token() || matches!(self.peek().unwrap().kind, TokenKind::Chuoi(_)))
        {
            let tok = self.advance().unwrap();
            match tok.kind {
                TokenKind::Ten(s) => Some(s),
                TokenKind::Chuoi(s) => Some(s),
                TokenKind::In => Some("in".to_string()),
                _ => None,
            }
        } else {
            None
        };
        self.skip_dot();
        Ok(Statement::Command {
            command_type,
            action,
            target,
        })
    }

    fn parse_if_or_rule(&mut self) -> Result<Statement, String> {
        self.advance(); // nếu

        if self.is_action_token() {
            let first = self.expect_action()?;
            if self.check_kind(&TokenKind::La) {
                self.advance(); // là
                let second = self.expect_action()?;
                let mut condition = Condition::Simple {
                    subject: first,
                    predicate: "là".to_string(),
                    object: second,
                };

                while self.check_kind(&TokenKind::Va) {
                    self.advance();
                    let s = self.expect_action()?;
                    let p = if self.check_kind(&TokenKind::La) {
                        self.advance();
                        "là".to_string()
                    } else {
                        self.expect_action()?
                    };
                    let o = self.expect_action()?;
                    condition = Condition::Conjunction(
                        Box::new(condition),
                        Box::new(Condition::Simple {
                            subject: s,
                            predicate: p,
                            object: o,
                        }),
                    );
                }

                let relation = if self.check_kind(&TokenKind::SuyRa) {
                    self.advance();
                    RuleRelation::Implication
                } else if self.check_kind(&TokenKind::TuongDuong) {
                    self.advance();
                    RuleRelation::Equivalence
                } else {
                    self.expect_kind(&TokenKind::Thi)?;
                    RuleRelation::Implication
                };

                let s = self.expect_action()?;
                let p = if self.check_kind(&TokenKind::La) {
                    self.advance();
                    "là".to_string()
                } else {
                    self.expect_action()?
                };
                let o = if self.peek().map_or(true, |t| {
                    matches!(
                        t.kind,
                        TokenKind::DauCham
                            | TokenKind::DauHoi
                            | TokenKind::DauThan
                            | TokenKind::EOF
                    )
                }) {
                    "đúng".to_string()
                } else {
                    self.expect_value()?
                };

                let conclusion = Condition::Simple {
                    subject: s,
                    predicate: p,
                    object: o,
                };
                self.skip_dot();
                return Ok(Statement::Rule {
                    condition: vec![condition],
                    conclusion,
                    relation,
                });
            } else {
                self.pos -= 1;
            }
        }

        let condition = self.parse_expression()?;
        self.expect_kind(&TokenKind::Thi)?;
        let mut then_body = Vec::new();
        while !self.check_kind(&TokenKind::KhongThi)
            && !self.check_kind(&TokenKind::KetThuc)
            && self.peek().is_some()
        {
            self.skip_dot();
            if self.check_kind(&TokenKind::KhongThi)
                || self.check_kind(&TokenKind::KetThuc)
                || self.peek().is_none()
            {
                break;
            }
            then_body.push(self.parse_statement()?);
        }
        let mut else_body = None;
        if self.check_kind(&TokenKind::KhongThi) {
            self.advance();
            self.expect_kind(&TokenKind::Thi)?;
            let mut else_stmts = Vec::new();
            while !self.check_kind(&TokenKind::KetThuc) && self.peek().is_some() {
                self.skip_dot();
                if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                    break;
                }
                else_stmts.push(self.parse_statement()?);
            }
            else_body = Some(else_stmts);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        self.skip_dot();
        Ok(Statement::IfElse {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_for(&mut self) -> Result<Statement, String> {
        self.advance(); // lặp
        let var = self.expect_action()?;
        self.expect_kind(&TokenKind::Tu)?;
        let start = self.parse_expression()?;
        self.expect_kind(&TokenKind::Den)?;
        let end = self.parse_expression()?;
        self.expect_kind(&TokenKind::Thi)?;
        let mut body = Vec::new();
        while !self.check_kind(&TokenKind::KetThuc) && self.peek().is_some() {
            self.skip_dot();
            if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                break;
            }
            body.push(self.parse_statement()?);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        self.skip_dot();
        Ok(Statement::ForLoop {
            var,
            start,
            end,
            body,
        })
    }

    fn parse_function(&mut self) -> Result<Statement, String> {
        self.advance(); // hàm
        let name = self.expect_action()?;
        let mut params = Vec::new();
        if self.check_kind(&TokenKind::DauNgoacTronMo) {
            self.advance();
            while !self.check_kind(&TokenKind::DauNgoacTronDong) && self.peek().is_some() {
                params.push(self.expect_action()?);
                if self.check_kind(&TokenKind::DauPhay) {
                    self.advance();
                }
            }
            self.expect_kind(&TokenKind::DauNgoacTronDong)?;
        }
        self.expect_kind(&TokenKind::Thi)?;
        let mut body = Vec::new();
        while !self.check_kind(&TokenKind::KetThuc) && self.peek().is_some() {
            self.skip_dot();
            if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                break;
            }
            body.push(self.parse_statement()?);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        self.skip_dot();
        Ok(Statement::Function { name, params, body })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_term()?;
        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::LonHon | TokenKind::NhoHon | TokenKind::Bang => {
                    let op = self.advance().unwrap();
                    let right = self.parse_term()?;
                    left = match op.kind {
                        TokenKind::LonHon => Expression::Gt(Box::new(left), Box::new(right)),
                        TokenKind::NhoHon => Expression::Lt(Box::new(left), Box::new(right)),
                        TokenKind::Bang => Expression::Eq(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;
        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::Cong | TokenKind::Tru => {
                    let op = self.advance().unwrap();
                    let right = self.parse_factor()?;
                    left = match op.kind {
                        TokenKind::Cong => Expression::Add(Box::new(left), Box::new(right)),
                        TokenKind::Tru => Expression::Sub(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let tok = self.advance().ok_or("Kết thúc file")?;
        match tok.kind {
            TokenKind::SoNguyen(n) => Ok(Expression::Number(n as f64)),
            TokenKind::SoThuc(n) => Ok(Expression::Number(n)),
            TokenKind::Chuoi(s) => Ok(Expression::String(s)),
            TokenKind::Ten(s) => Ok(Expression::Variable(s)),
            TokenKind::In => Ok(Expression::Variable("in".to_string())),
            TokenKind::Hoi => Ok(Expression::Variable("hỏi".to_string())),
            TokenKind::DauNgoacTronMo => {
                let expr = self.parse_expression()?;
                self.expect_kind(&TokenKind::DauNgoacTronDong)?;
                Ok(expr)
            }
            _ => Err(format!("Token không mong đợi trong biểu thức: {:?}", tok)),
        }
    }

    fn expect_value(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(tok) => match tok.kind {
                TokenKind::Ten(s) => Ok(s),
                TokenKind::Chuoi(s) => Ok(s),
                TokenKind::SoNguyen(n) => Ok(n.to_string()),
                TokenKind::SoThuc(n) => Ok(n.to_string()),
                TokenKind::Dung => Ok("đúng".to_string()),
                TokenKind::Sai => Ok("sai".to_string()),
                _ => Err(format!("Mong đợi giá trị nhưng gặp {:?}", tok)),
            },
            None => Err("Kết thúc file".to_string()),
        }
    }

    fn check_kind(&self, kind: &TokenKind) -> bool {
        match self.peek() {
            Some(tok) => std::mem::discriminant(&tok.kind) == std::mem::discriminant(kind),
            None => false,
        }
    }

    fn expect_kind(&mut self, kind: &TokenKind) -> Result<(), String> {
        if self.check_kind(kind) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Mong đợi {:?} nhưng gặp {:?}", kind, self.peek()))
        }
    }
}
