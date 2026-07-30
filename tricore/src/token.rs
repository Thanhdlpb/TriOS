#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Từ khóa cấu trúc
    ChuongTrinh, BatDau, KetThuc, In, Ham,
    Neu, Thi, KhongThi, Lap, Tu, Den,
    Hoi, CoPhai, Hay, DungLenh,
    La, Va, Hoac, Dung, Sai,
    SuyRa, TuongDuong,
    DungModule, // <<< THÊM TỪ KHÓA "dùng"
    // Quan hệ từ
    Cua, BangQuanHe, Voi, Ve, Cho, Tai, Trong, Tren, Duoi,
    // Thời gian
    Da, Dang, Se, Vua, Sap,
    // Phủ định
    Khong, Chua, Chang,
    // Nghi vấn
    Gi, Dau, Nao, HayKhong, DaChua,
    // Ký hiệu
    DauCham, DauHoi, DauThan, DauPhay,
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
