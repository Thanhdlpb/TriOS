#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Từ khóa
    ChuongTrinh, BatDau, KetThuc, InRa, Ham,
    Neu, Thi, Va, Hoi, Khi, La, Cho, Dung,
    Voi, Moi, Trong, Lam,  // <<< Thêm Lam
    // Ký hiệu
    DauCham, DauPhay, DauHoi,
    DauNgoacTronMo, DauNgoacTronDong,
    DauNgoacVuongMo, DauNgoacVuongDong,
    Gan, Cong, Tru, Nhan, Chia,
    // Dữ liệu
    Ten(String),
    Chuoi(String),
    So(f64),
    // Đặc biệt
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}
