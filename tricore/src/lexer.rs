use crate::token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if let Some(ch) = c {
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
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
        let mut s = String::new();
        s.push(first);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else { break; }
        }
        s
    }

    fn read_string(&mut self) -> String {
        self.advance(); // bỏ dấu "
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '"' { self.advance(); break; }
            s.push(c);
            self.advance();
        }
        s
    }

    fn read_number(&mut self, first: char) -> (f64, bool) {
        let mut s = String::new();
        s.push(first);
        let mut is_integer = true;
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                s.push(c);
                self.advance();
            } else if c == '.' {
                is_integer = false;
                s.push(c);
                self.advance();
            } else { break; }
        }
        (s.parse().unwrap_or(0.0), is_integer)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let line = self.line;
        let col = self.col;
        match self.advance() {
            None => Token::new(TokenKind::EOF, line, col),
            Some('.') => Token::new(TokenKind::DauCham, line, col),
            Some(',') => Token::new(TokenKind::DauPhay, line, col),
            Some('?') => Token::new(TokenKind::DauHoi, line, col),
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
            Some('"') => {
                let s = self.read_string();
                Token::new(TokenKind::Chuoi(s), line, col)
            }
            Some(c) if c.is_digit(10) => {
                let (n, is_int) = self.read_number(c);
                if is_int && n == (n as i64) as f64 {
                    Token::new(TokenKind::SoNguyenVal(n as i64), line, col)
                } else {
                    Token::new(TokenKind::So(n), line, col)
                }
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let ident = self.read_identifier(c);
                match ident.as_str() {
                    "chương_trình" => Token::new(TokenKind::ChuongTrinh, line, col),
                    "bắt_đầu" => Token::new(TokenKind::BatDau, line, col),
                    "kết_thúc" => Token::new(TokenKind::KetThuc, line, col),
                    "in_ra" => Token::new(TokenKind::InRa, line, col),
                    "hàm" => Token::new(TokenKind::Ham, line, col),
                    "nếu" => Token::new(TokenKind::Neu, line, col),
                    "thì" => Token::new(TokenKind::Thi, line, col),
                    "và" => Token::new(TokenKind::Va, line, col),
                    "hỏi" => Token::new(TokenKind::Hoi, line, col),
                    "khi" => Token::new(TokenKind::Khi, line, col),
                    "là" => Token::new(TokenKind::La, line, col),
                    "cho" => Token::new(TokenKind::Cho, line, col),
                    "dùng" => Token::new(TokenKind::Dung, line, col),
                    "với" => Token::new(TokenKind::Voi, line, col),
                    "mỗi" => Token::new(TokenKind::Moi, line, col),
                    "trong" => Token::new(TokenKind::Trong, line, col),
                    "làm" => Token::new(TokenKind::Lam, line, col),
                    "nếu_khác" => Token::new(TokenKind::NeuKhac, line, col),
                    "trong_khi" => Token::new(TokenKind::TrongKhi, line, col),
                    "trả_về" => Token::new(TokenKind::TraVe, line, col),
                    "đúng_sai" => Token::new(TokenKind::DungSai, line, col),
                    "bỏ_qua" => Token::new(TokenKind::BoQua, line, col),
                    "số_nguyên" => Token::new(TokenKind::SoNguyen, line, col),
                    "số_thực" => Token::new(TokenKind::SoThuc, line, col),
                    "chuỗi" => Token::new(TokenKind::ChuoiKyTu, line, col),
                    "đúng" => Token::new(TokenKind::DungS, line, col),
                    "mảng" => Token::new(TokenKind::Mang, line, col),
                    _ => Token::new(TokenKind::Ten(ident), line, col),
                }
            }
            Some(c) => panic!("Ký tự không hợp lệ '{}' tại dòng {} cột {}", c, line, col),
        }
    }
}
