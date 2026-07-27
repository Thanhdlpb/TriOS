#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Từ khóa cấu trúc
    Neu, Thi, KhongThi, KetThuc, Lap, Tu, Den, Ham, In, La, Va, Hoac, Dung, Sai,
    Hoi, Gi, Dau, Nao,
    ChuongTrinh, BatDau,
    // Quan hệ từ
    Cua, BangQuanHe, Voi, Ve, Cho, Tai, Trong, Tren, Duoi,
    // Thời gian
    Da, Dang, Se, Vua, Sap,
    // Phủ định
    Khong, Chua, Chang,
    // Nghi vấn
    CoPhai, HayKhong, DaChua,
    // Mệnh lệnh
    Hay, DungLenh,
    // Logic
    SuyRa, TuongDuong,
    // Ký hiệu
    DauCham, DauHoi, DauPhay, DauThan,
    DauNgoacTronMo, DauNgoacTronDong,
    DauNgoacVuongMo, DauNgoacVuongDong,
    Gan, Cong, Tru, Nhan, Chia,
    LonHon, NhoHon, Bang, Khac,
    // Dữ liệu
    Ten(String),
    Chuoi(String),
    SoNguyen(i64),
    SoThuc(f64),
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
