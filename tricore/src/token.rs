#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Từ khóa
    ChuongTrinh, BatDau, KetThuc, InRa, Ham,
    Neu, Thi, Va, Hoi, Khi, La, Cho, Dung,
    Voi, Moi, Trong, Lam, NeuKhac, TrongKhi,
    TraVe, DungSai, BoQua,
    // Kiểu dữ liệu
    SoNguyen, SoThuc, ChuoiKyTu, DungS, Mang,
    // Toán tử
    LonHon, NhoHon, Bang, Khac,
    // Ký hiệu
    DauCham, DauPhay, DauHoi,
    DauNgoacTronMo, DauNgoacTronDong,
    DauNgoacVuongMo, DauNgoacVuongDong,
    Gan, Cong, Tru, Nhan, Chia,
    // Dữ liệu
    Ten(String),
    Chuoi(String),
    So(f64),
    SoNguyenVal(i64),
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
