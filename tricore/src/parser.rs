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

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    fn check_ten(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Ten(_))
    }

    fn check_chuoi(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Chuoi(_))
    }

    fn check_so(&self) -> bool {
        matches!(self.peek().kind, TokenKind::So(_))
    }

    fn check_kind(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    fn skip_dau_cham(&mut self) {
        if self.check_kind(&TokenKind::DauCham) {
            self.advance();
        }
    }

    fn skip_dau_hoi(&mut self) {
        if self.check_kind(&TokenKind::DauHoi) {
            self.advance();
        }
    }

    pub fn parse_chuong_trinh(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while !self.check_kind(&TokenKind::EOF) {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        self.skip_dau_cham();
        match &self.peek().kind {
            TokenKind::ChuongTrinh => Ok(Statement::ChuongTrinh(self.parse_full_chuong_trinh()?)),
            TokenKind::Ham => Ok(Statement::Ham(self.parse_ham()?)),
            TokenKind::Neu => Ok(Statement::Luat(self.parse_luat()?)),
            TokenKind::Hoi => Ok(Statement::TruyVan(self.parse_truy_van()?)),
            TokenKind::InRa => Ok(Statement::InRa(self.parse_in_ra()?)),
            TokenKind::Voi => Ok(Statement::VongLap(self.parse_vong_lap()?)),
            TokenKind::Ten(_) => Ok(Statement::PhatBieu(self.parse_phat_bieu()?)),
            _ => Err(format!("Token không mong đợi: {:?}", self.peek())),
        }
    }

    fn parse_full_chuong_trinh(&mut self) -> Result<ChuongTrinh, String> {
        self.advance(); // chương_trình
        let ten = if let TokenKind::Chuoi(s) = &self.advance().kind {
            s.clone()
        } else {
            return Err("Mong đợi tên chương trình dạng chuỗi".into());
        };
        if !self.check_kind(&TokenKind::BatDau) {
            return Err("Mong đợi 'bắt_đầu'".into());
        }
        self.advance(); // bắt_đầu
        let mut than = Vec::new();
        while !self.check_kind(&TokenKind::KetThuc) {
            than.push(self.parse_statement()?);
        }
        self.advance(); // kết_thúc
        Ok(ChuongTrinh { ten, than })
    }

    fn parse_phat_bieu(&mut self) -> Result<PhatBieu, String> {
        let chu_ngu = self.expect_ten()?;
        let dong_tu = if self.check_ten() {
            Some(self.expect_ten()?)
        } else if self.check_kind(&TokenKind::La) {
            self.advance();
            Some("là".to_string())
        } else {
            None
        };
        let tan_ngu = if self.check_ten() || self.check_chuoi() || self.check_so() {
            Some(match &self.advance().kind {
                TokenKind::Ten(s) => s.clone(),
                TokenKind::Chuoi(s) => s.clone(),
                TokenKind::So(n) => n.to_string(),
                _ => unreachable!(),
            })
        } else {
            None
        };
        self.skip_dau_cham();
        Ok(PhatBieu { chu_ngu, dong_tu, tan_ngu })
    }

    fn parse_luat(&mut self) -> Result<Luat, String> {
        self.advance(); // nếu
        let mut dieu_kien = Vec::new();
        while !self.check_kind(&TokenKind::Thi) {
            if self.check_kind(&TokenKind::Va) { self.advance(); }
            let s = self.expect_ten()?;
            let p = self.expect_ten()?;
            let o = self.expect_ten()?;
            dieu_kien.push((s, p, o));
        }
        self.advance(); // thì
        let s = self.expect_ten()?;
        let p = self.expect_ten()?;
        let o = self.expect_ten()?;
        let ket_luan = (s, p, o);
        self.skip_dau_cham();
        Ok(Luat { dieu_kien, ket_luan })
    }

    fn parse_truy_van(&mut self) -> Result<TruyVan, String> {
        self.advance(); // hỏi
        let s = self.expect_ten()?;
        let p = if self.check_kind(&TokenKind::La) {
            self.advance();
            "là".to_string()
        } else {
            self.expect_ten()?
        };
        let o = if self.check_ten() {
            let ten = self.expect_ten()?;
            if ten == "gì" {
                "?".to_string()
            } else {
                ten
            }
        } else if self.check_chuoi() {
            if let TokenKind::Chuoi(c) = &self.advance().kind {
                c.clone()
            } else {
                unreachable!()
            }
        } else if self.check_kind(&TokenKind::DauHoi) {
            "?".to_string()
        } else {
            return Err(format!("Mong đợi tân ngữ hoặc 'gì' nhưng gặp {:?}", self.peek()));
        };
        let muc_tieu = (s, p, o);
        let mut rang_buoc = None;
        if self.check_kind(&TokenKind::Khi) {
            self.advance();
            let mut constraints = Vec::new();
            while !self.check_kind(&TokenKind::DauHoi) && !self.check_kind(&TokenKind::EOF) {
                let cs = self.expect_ten()?;
                let cp = self.expect_ten()?;
                let co = self.expect_ten()?;
                constraints.push((cs, cp, co));
                self.skip_dau_cham();
            }
            rang_buoc = Some(constraints);
        }
        self.skip_dau_hoi();
        self.skip_dau_cham();
        Ok(TruyVan { muc_tieu, rang_buoc })
    }

    fn parse_in_ra(&mut self) -> Result<InRa, String> {
        self.advance(); // in_ra
        let bieu_thuc = match &self.advance().kind {
            TokenKind::Chuoi(s) => s.clone(),
            TokenKind::Ten(s) => s.clone(),
            _ => return Err("Mong đợi chuỗi hoặc biến".into()),
        };
        self.skip_dau_cham();
        Ok(InRa { bieu_thuc })
    }

    fn parse_vong_lap(&mut self) -> Result<VongLap, String> {
        self.advance(); // với
        self.advance(); // mỗi
        let bien = self.expect_ten()?;
        self.advance(); // trong
        let mut danh_sach = Vec::new();
        if self.check_kind(&TokenKind::DauNgoacVuongMo) {
            self.advance();
            while !self.check_kind(&TokenKind::DauNgoacVuongDong) {
                danh_sach.push(self.expect_ten()?);
                if self.check_kind(&TokenKind::DauPhay) { self.advance(); }
            }
            self.advance();
        }
        self.advance(); // làm
        let mut than = Vec::new();
        while !self.check_kind(&TokenKind::KetThuc) {
            than.push(self.parse_statement()?);
        }
        self.advance(); // kết_thúc
        self.skip_dau_cham();
        Ok(VongLap { bien, danh_sach, than })
    }

    fn parse_ham(&mut self) -> Result<Ham, String> {
        self.advance(); // hàm
        let ten = self.expect_ten()?;
        let mut tham_so = Vec::new();
        if self.check_kind(&TokenKind::DauNgoacTronMo) {
            self.advance();
            while !self.check_kind(&TokenKind::DauNgoacTronDong) {
                tham_so.push(self.expect_ten()?);
                if self.check_kind(&TokenKind::DauPhay) { self.advance(); }
            }
            self.advance();
        }
        self.advance(); // là
        let mut than = Vec::new();
        while !self.check_kind(&TokenKind::KetThuc) {
            than.push(self.parse_statement()?);
        }
        self.advance(); // kết_thúc
        self.skip_dau_cham();
        Ok(Ham { ten, tham_so, than })
    }

    fn expect_ten(&mut self) -> Result<String, String> {
        if let TokenKind::Ten(s) = &self.advance().kind {
            Ok(s.clone())
        } else {
            Err(format!("Mong đợi tên nhưng gặp {:?}", self.peek()))
        }
    }
}
