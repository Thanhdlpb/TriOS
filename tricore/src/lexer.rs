use crate::token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self { chars: source.chars().collect(), pos: 0, line: 1, col: 1 }
    }

    fn peek(&self) -> Option<char> { self.chars.get(self.pos).copied() }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if let Some(ch) = c {
            self.pos += 1;
            if ch == '\n' { self.line += 1; self.col = 1; }
            else { self.col += 1; }
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() { self.advance(); }
            else { break; }
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut s = String::new(); s.push(first);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' { s.push(c); self.advance(); }
            else { break; }
        }
        s
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '"' { self.advance(); break; }
            s.push(c);
            self.advance();
        }
        s
    }

    fn read_number(&mut self, first: char) -> TokenKind {
        let mut s = String::new(); s.push(first);
        let mut is_float = false;
        while let Some(c) = self.peek() {
            if c.is_digit(10) { s.push(c); self.advance(); }
            else if c == '.' { is_float = true; s.push(c); self.advance(); }
            else { break; }
        }
        if is_float { TokenKind::SoThuc(s.parse().unwrap_or(0.0)) }
        else { TokenKind::SoNguyen(s.parse().unwrap_or(0)) }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let line = self.line;
        let col = self.col;
        match self.advance() {
            None => Token::new(TokenKind::EOF, line, col),
            Some('.') => Token::new(TokenKind::DauCham, line, col),
            Some('?') => Token::new(TokenKind::DauHoi, line, col),
            Some('!') => Token::new(TokenKind::DauThan, line, col),
            Some(',') => Token::new(TokenKind::DauPhay, line, col),
            Some('(') => Token::new(TokenKind::DauNgoacTronMo, line, col),
            Some(')') => Token::new(TokenKind::DauNgoacTronDong, line, col),
            Some('[') => Token::new(TokenKind::DauNgoacVuongMo, line, col),
            Some(']') => Token::new(TokenKind::DauNgoacVuongDong, line, col),
            Some('=') => Token::new(TokenKind::Gan, line, col),
            Some('+') => Token::new(TokenKind::Cong, line, col),
            Some('-') => Token::new(TokenKind::Tru, line, col),
            Some('*') => Token::new(TokenKind::Nhan, line, col),
            Some('/') => Token::new(TokenKind::Chia, line, col),
            Some('>') => Token::new(TokenKind::LonHon, line, col),
            Some('<') => Token::new(TokenKind::NhoHon, line, col),
            Some('"') => Token::new(TokenKind::Chuoi(self.read_string()), line, col),
            Some(c) if c.is_digit(10) => Token::new(self.read_number(c), line, col),
            Some(c) if c.is_alphabetic() || c == '_' => {
                let ident = self.read_identifier(c);
                match ident.as_str() {
                    "chương_trình" => Token::new(TokenKind::ChuongTrinh, line, col),
                    "bắt_đầu" => Token::new(TokenKind::BatDau, line, col),
                    "kết_thúc" => Token::new(TokenKind::KetThuc, line, col),
                    "in" => Token::new(TokenKind::In, line, col),
                    "in_ra" => Token::new(TokenKind::In, line, col),
                    "hàm" => Token::new(TokenKind::Ham, line, col),
                    "nếu" => Token::new(TokenKind::Neu, line, col),
                    "thì" => Token::new(TokenKind::Thi, line, col),
                    "không_thì" => Token::new(TokenKind::KhongThi, line, col),
                    "lặp" => Token::new(TokenKind::Lap, line, col),
                    "từ" => Token::new(TokenKind::Tu, line, col),
                    "đến" => Token::new(TokenKind::Den, line, col),
                    "hỏi" => Token::new(TokenKind::Hoi, line, col),
                    "có_phải" => Token::new(TokenKind::CoPhai, line, col),
                    "hãy" => Token::new(TokenKind::Hay, line, col),
                    "đừng" => Token::new(TokenKind::DungLenh, line, col),
                    "là" => Token::new(TokenKind::La, line, col),
                    "và" => Token::new(TokenKind::Va, line, col),
                    "hoặc" => Token::new(TokenKind::Hoac, line, col),
                    "đúng" => Token::new(TokenKind::Dung, line, col),
                    "sai" => Token::new(TokenKind::Sai, line, col),
                    "suy_ra" => Token::new(TokenKind::SuyRa, line, col),
                    "tương_đương" => Token::new(TokenKind::TuongDuong, line, col),
                    "dùng" => Token::new(TokenKind::DungModule, line, col), // <<< THÊM "dùng"
                    "gì" => Token::new(TokenKind::Gi, line, col),
                    "đâu" => Token::new(TokenKind::Dau, line, col),
                    "của" => Token::new(TokenKind::Cua, line, col),
                    "bằng" => Token::new(TokenKind::BangQuanHe, line, col),
                    "với" => Token::new(TokenKind::Voi, line, col),
                    "về" => Token::new(TokenKind::Ve, line, col),
                    "cho" => Token::new(TokenKind::Cho, line, col),
                    "tại" => Token::new(TokenKind::Tai, line, col),
                    "trong" => Token::new(TokenKind::Trong, line, col),
                    "trên" => Token::new(TokenKind::Tren, line, col),
                    "dưới" => Token::new(TokenKind::Duoi, line, col),
                    "đã" => Token::new(TokenKind::Da, line, col),
                    "đang" => Token::new(TokenKind::Dang, line, col),
                    "sẽ" => Token::new(TokenKind::Se, line, col),
                    "vừa" => Token::new(TokenKind::Vua, line, col),
                    "sắp" => Token::new(TokenKind::Sap, line, col),
                    "không" => Token::new(TokenKind::Khong, line, col),
                    "chưa" => Token::new(TokenKind::Chua, line, col),
                    "chẳng" => Token::new(TokenKind::Chang, line, col),
                    "hay_không" => Token::new(TokenKind::HayKhong, line, col),
                    "đã_chưa" => Token::new(TokenKind::DaChua, line, col),
                    _ => Token::new(TokenKind::Ten(ident), line, col),
                }
            }
            Some(c) => panic!("Ký tự không hợp lệ '{}' tại dòng {} cột {}", c, line, col),
        }
    }
}
