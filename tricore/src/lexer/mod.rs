pub mod tu_khoa;
pub mod dinh_danh;
pub mod so;
pub mod chuoi;
pub mod toan_tu;
pub mod ky_tu;

use crate::reader::Reader;
use crate::token::{LoaiToken, Token};
use crate::vi_tri::ViTri;

pub struct Lexer {
    reader: Reader,
    da_tra_eof: bool,
}

impl Lexer {
    pub fn moi(text: &str) -> Self {
        Self {
            reader: Reader::moi(text),
            da_tra_eof: false,
        }
    }

    pub fn token_tiep(&mut self) -> Option<Token> {
        if self.da_tra_eof {
            return None;
        }

        self.reader.bo_qua_khoang_trang();

        if self.reader.eof() {
            self.da_tra_eof = true;
            return Some(Token {
                loai: LoaiToken::EOF,
                gia_tri: String::new(),
                vi_tri: ViTri { dong: 0, cot: 0 },
            });
        }

        let c = self.reader.peek()?;

        if c.is_alphabetic() || c == '_' {
            let text = dinh_danh::doc_dinh_danh(&mut self.reader);

            let loai = tu_khoa::tim_tu_khoa(&text)
                .unwrap_or(LoaiToken::DinhDanh);

            return Some(Token {
                loai,
                gia_tri: text,
                vi_tri: ViTri { dong: 0, cot: 0 },
            });
        }

        if c.is_ascii_digit() {
            let so = so::doc_so(&mut self.reader);

            return Some(Token {
                loai: LoaiToken::So,
                gia_tri: so,
                vi_tri: ViTri { dong: 0, cot: 0 },
            });
        }

        if c == '=' {
            self.reader.next();

            return Some(Token {
                loai: LoaiToken::Gan,
                gia_tri: "=".to_string(),
                vi_tri: ViTri { dong: 0, cot: 0 },
            });
        }

        self.reader.next();

        self.token_tiep()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::token::LoaiToken;

    #[test]
    fn kiem_tra_bien_va_so() {

        let mut lexer = Lexer::moi("biến tuổi = 20");

        let t1 = lexer.token_tiep().unwrap();
        assert_eq!(t1.loai, LoaiToken::Bien);

        let t2 = lexer.token_tiep().unwrap();
        assert_eq!(t2.loai, LoaiToken::DinhDanh);
        assert_eq!(t2.gia_tri, "tuổi");

        let t3 = lexer.token_tiep().unwrap();
        assert_eq!(t3.loai, LoaiToken::Gan);

        let t4 = lexer.token_tiep().unwrap();
        assert_eq!(t4.loai, LoaiToken::So);
        assert_eq!(t4.gia_tri, "20");

        let t5 = lexer.token_tiep().unwrap();
        assert_eq!(t5.loai, LoaiToken::EOF);
    }

}
