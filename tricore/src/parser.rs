use crate::token::{Token, TokenKind};
use crate::ast::*;

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

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn skip_punctuation(&mut self) {
        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::DauCham | TokenKind::DauHoi | TokenKind::DauPhay => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    pub fn parse_chuong_trinh(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        loop {
            self.skip_punctuation();
            if self.peek().is_none() || matches!(self.peek(), Some(Token { kind: TokenKind::EOF, .. })) {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        self.skip_punctuation();
        let tok = self.peek().cloned();
        match tok {
            None => Err("Kết thúc file bất ngờ".to_string()),
            Some(Token { kind: TokenKind::ChuongTrinh, .. }) => self.parse_full_chuong_trinh().map(Statement::ChuongTrinh),
            Some(Token { kind: TokenKind::Ham, .. }) => self.parse_ham().map(Statement::Ham),
            Some(Token { kind: TokenKind::Neu, .. }) => self.parse_if_else().map(Statement::IfElse),
            Some(Token { kind: TokenKind::TrongKhi, .. }) => self.parse_while().map(Statement::WhileLoop),
            Some(Token { kind: TokenKind::TraVe, .. }) => self.parse_tra_ve().map(Statement::TraVe),
            Some(Token { kind: TokenKind::Hoi, .. }) => self.parse_truy_van().map(Statement::TruyVan),
            Some(Token { kind: TokenKind::InRa, .. }) => self.parse_in_ra().map(Statement::InRa),
            Some(Token { kind: TokenKind::Voi, .. }) => self.parse_vong_lap().map(Statement::VongLap),
            Some(Token { kind: TokenKind::Ten(_), .. }) => {
                // Kiểm tra xem có phải phép gán không (X = ...)
                if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1].kind == TokenKind::Gan {
                    self.parse_gan().map(Statement::Gan)
                } else {
                    self.parse_phat_bieu().map(Statement::PhatBieu)
                }
            }
            _ => Err(format!("Token không mong đợi: {:?}", tok)),
        }
    }

    fn parse_gan(&mut self) -> Result<Gan, String> {
        let bien = self.expect_ten()?;
        self.expect_kind(&TokenKind::Gan)?;
        let bieu_thuc = self.parse_bieu_thuc_string()?;
        self.skip_punctuation();
        Ok(Gan { bien, bieu_thuc })
    }

    fn parse_tra_ve(&mut self) -> Result<TraVe, String> {
        self.advance(); // trả_về
        let bieu_thuc = self.parse_bieu_thuc_string()?;
        self.skip_punctuation();
        Ok(TraVe { bieu_thuc })
    }

    fn parse_bieu_thuc_string(&mut self) -> Result<String, String> {
        let mut result = String::new();
        while !self.check_kind(&TokenKind::DauCham) && !self.check_kind(&TokenKind::KetThuc) && self.peek().is_some() {
            if let Some(tok) = self.advance() {
                match &tok.kind {
                    TokenKind::Ten(s) => result.push_str(s),
                    TokenKind::So(n) => result.push_str(&n.to_string()),
                    TokenKind::SoNguyenVal(i) => result.push_str(&i.to_string()),
                    TokenKind::Chuoi(s) => result.push_str(&format!("\"{}\"", s)),
                    TokenKind::Cong => result.push_str(" + "),
                    TokenKind::Tru => result.push_str(" - "),
                    TokenKind::Nhan => result.push_str(" * "),
                    TokenKind::Chia => result.push_str(" / "),
                    TokenKind::DauNgoacTronMo => result.push('('),
                    TokenKind::DauNgoacTronDong => result.push(')'),
                    TokenKind::LonHon => result.push_str(" > "),
                    TokenKind::NhoHon => result.push_str(" < "),
                    TokenKind::Bang => result.push_str(" == "),
                    _ => break,
                }
            }
        }
        Ok(result.trim().to_string())
    }

    fn parse_if_else(&mut self) -> Result<IfElse, String> {
        self.advance(); // nếu
        let s = self.expect_ten()?;
        let p = self.parse_predicate()?;
        let o = self.expect_ten()?;
        let dieu_kien = (s, p, o);
        self.expect_kind(&TokenKind::Thi)?;
        let mut dung = Vec::new();
        while !self.check_kind(&TokenKind::NeuKhac) && !self.check_kind(&TokenKind::KetThuc) && self.peek().is_some() {
            self.skip_punctuation();
            if self.check_kind(&TokenKind::NeuKhac) || self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                break;
            }
            dung.push(self.parse_statement()?);
        }
        let mut sai = None;
        if self.check_kind(&TokenKind::NeuKhac) {
            self.advance(); // nếu_khác
            self.expect_kind(&TokenKind::Thi)?;
            let mut sai_vec = Vec::new();
            while !self.check_kind(&TokenKind::KetThuc) && self.peek().is_some() {
                self.skip_punctuation();
                if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                    break;
                }
                sai_vec.push(self.parse_statement()?);
            }
            sai = Some(sai_vec);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        Ok(IfElse { dieu_kien, dung, sai })
    }

    fn parse_while(&mut self) -> Result<WhileLoop, String> {
        self.advance(); // trong_khi
        let s = self.expect_ten()?;
        let p = self.parse_predicate()?;
        let o = self.expect_ten()?;
        let dieu_kien = (s, p, o);
        self.expect_kind(&TokenKind::Lam)?;
        let mut than = Vec::new();
        while !self.check_kind(&TokenKind::KetThuc) && self.peek().is_some() {
            self.skip_punctuation();
            if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                break;
            }
            than.push(self.parse_statement()?);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        Ok(WhileLoop { dieu_kien, than })
    }

    fn parse_full_chuong_trinh(&mut self) -> Result<ChuongTrinh, String> {
        self.advance(); // chương_trình
        let ten = self.expect_chuoi()?;
        self.expect_kind(&TokenKind::BatDau)?;
        let mut than = Vec::new();
        loop {
            self.skip_punctuation();
            if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                break;
            }
            than.push(self.parse_statement()?);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        Ok(ChuongTrinh { ten, than })
    }

    fn parse_phat_bieu(&mut self) -> Result<PhatBieu, String> {
        let chu_ngu = self.expect_ten()?;
        let dong_tu = if self.check_kind(&TokenKind::La) {
            self.advance();
            Some("là".to_string())
        } else if self.check_ten() {
            Some(self.expect_ten()?)
        } else {
            None
        };
        let tan_ngu = if self.check_ten() || self.check_chuoi() || self.check_so() {
            Some(self.advance_value()?)
        } else if self.check_kind(&TokenKind::DauHoi) {
            self.advance();
            Some("?".to_string())
        } else {
            Some("đúng".to_string())
        };
        self.skip_punctuation();
        Ok(PhatBieu { chu_ngu, dong_tu, tan_ngu })
    }

    fn parse_luat(&mut self) -> Result<Luat, String> {
        self.advance(); // nếu
        let mut dieu_kien = Vec::new();
        loop {
            self.skip_punctuation();
            if self.check_kind(&TokenKind::Thi) || self.peek().is_none() {
                break;
            }
            if self.check_kind(&TokenKind::Va) {
                self.advance();
            }
            let s = self.expect_ten()?;
            let p = self.parse_predicate()?;
            let o = self.parse_optional_object().unwrap_or_else(|| "đúng".to_string());
            dieu_kien.push((s, p, o));
        }
        self.expect_kind(&TokenKind::Thi)?;
        let s = self.expect_ten()?;
        let p = self.parse_predicate()?;
        let o = self.parse_optional_object().unwrap_or_else(|| "đúng".to_string());
        let ket_luan = (s, p, o);
        self.skip_punctuation();
        Ok(Luat { dieu_kien, ket_luan })
    }

    fn parse_truy_van(&mut self) -> Result<TruyVan, String> {
        self.advance(); // hỏi
        self.skip_punctuation();
        let s = self.expect_ten()?;
        let p = self.parse_predicate()?;
        let o = if self.check_ten() {
            let ten = self.expect_ten()?;
            if ten == "gì" { "?".to_string() } else { ten }
        } else if self.check_chuoi() || self.check_so() {
            self.advance_value()?
        } else {
            "?".to_string()
        };
        self.skip_punctuation();
        Ok(TruyVan { muc_tieu: (s, p, o), rang_buoc: None })
    }

    fn parse_in_ra(&mut self) -> Result<InRa, String> {
        self.advance(); // in_ra
        let val = self.advance_value()?;
        self.skip_punctuation();
        Ok(InRa { bieu_thuc: val })
    }

    fn parse_vong_lap(&mut self) -> Result<VongLap, String> {
        self.advance(); // với
        self.advance(); // mỗi
        let bien = self.expect_ten()?;
        self.advance(); // trong
        let mut danh_sach = Vec::new();
        if self.check_kind(&TokenKind::DauNgoacVuongMo) {
            self.advance();
            while !self.check_kind(&TokenKind::DauNgoacVuongDong) && self.peek().is_some() {
                danh_sach.push(self.expect_ten()?);
                if self.check_kind(&TokenKind::DauPhay) { self.advance(); }
            }
            self.expect_kind(&TokenKind::DauNgoacVuongDong)?;
        }
        self.expect_kind(&TokenKind::Lam)?;
        let mut than = Vec::new();
        loop {
            self.skip_punctuation();
            if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                break;
            }
            than.push(self.parse_statement()?);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        self.skip_punctuation();
        Ok(VongLap { bien, danh_sach, than })
    }

    fn parse_ham(&mut self) -> Result<Ham, String> {
        self.advance(); // hàm
        let ten = self.expect_ten()?;
        let mut tham_so = Vec::new();
        if self.check_kind(&TokenKind::DauNgoacTronMo) {
            self.advance();
            while !self.check_kind(&TokenKind::DauNgoacTronDong) && self.peek().is_some() {
                tham_so.push(self.expect_ten()?);
                if self.check_kind(&TokenKind::DauPhay) { self.advance(); }
            }
            self.expect_kind(&TokenKind::DauNgoacTronDong)?;
        }
        self.expect_kind(&TokenKind::La)?;
        let mut than = Vec::new();
        loop {
            self.skip_punctuation();
            if self.check_kind(&TokenKind::KetThuc) || self.peek().is_none() {
                break;
            }
            than.push(self.parse_statement()?);
        }
        self.expect_kind(&TokenKind::KetThuc)?;
        self.skip_punctuation();
        Ok(Ham { ten, tham_so, than })
    }

    fn parse_predicate(&mut self) -> Result<String, String> {
        if self.check_kind(&TokenKind::La) {
            self.advance();
            Ok("là".to_string())
        } else {
            self.expect_ten()
        }
    }

    fn parse_optional_object(&mut self) -> Option<String> {
        if self.check_ten() || self.check_chuoi() || self.check_so() {
            self.advance_value().ok()
        } else if self.check_kind(&TokenKind::DauHoi) {
            self.advance();
            Some("?".to_string())
        } else {
            None
        }
    }

    fn expect_ten(&mut self) -> Result<String, String> {
        self.skip_punctuation();
        match self.advance() {
            Some(Token { kind: TokenKind::Ten(s), .. }) => Ok(s),
            other => Err(format!("Mong đợi tên nhưng gặp {:?}", other)),
        }
    }

    fn expect_chuoi(&mut self) -> Result<String, String> {
        self.skip_punctuation();
        match self.advance() {
            Some(Token { kind: TokenKind::Chuoi(s), .. }) => Ok(s),
            other => Err(format!("Mong đợi chuỗi nhưng gặp {:?}", other)),
        }
    }

    fn advance_value(&mut self) -> Result<String, String> {
        self.skip_punctuation();
        match self.advance() {
            Some(tok) => match tok.kind {
                TokenKind::Ten(s) => Ok(s),
                TokenKind::Chuoi(s) => Ok(s),
                TokenKind::So(n) => Ok(n.to_string()),
                TokenKind::SoNguyenVal(i) => Ok(i.to_string()),
                _ => Err(format!("Mong đợi giá trị nhưng gặp {:?}", tok)),
            },
            None => Err("Kết thúc file bất ngờ".to_string()),
        }
    }

    fn check_ten(&self) -> bool { matches!(self.peek(), Some(Token { kind: TokenKind::Ten(_), .. })) }
    fn check_chuoi(&self) -> bool { matches!(self.peek(), Some(Token { kind: TokenKind::Chuoi(_), .. })) }
    fn check_so(&self) -> bool { matches!(self.peek(), Some(Token { kind: TokenKind::So(_), .. })) || matches!(self.peek(), Some(Token { kind: TokenKind::SoNguyenVal(_), .. })) }

    fn check_kind(&self, kind: &TokenKind) -> bool {
        match self.peek() {
            Some(tok) => std::mem::discriminant(&tok.kind) == std::mem::discriminant(kind),
            None => false,
        }
    }

    fn expect_kind(&mut self, kind: &TokenKind) -> Result<(), String> {
        self.skip_punctuation();
        if self.check_kind(kind) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Mong đợi {:?} nhưng gặp {:?}", kind, self.peek()))
        }
    }
}
