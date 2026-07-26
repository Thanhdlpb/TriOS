use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    PhatBieu(PhatBieu),
    Luat(Luat),
    TruyVan(TruyVan),
    InRa(InRa),
    VongLap(VongLap),
    Ham(Ham),
    ChuongTrinh(ChuongTrinh),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhatBieu {
    pub chu_ngu: String,
    pub dong_tu: Option<String>,
    pub tan_ngu: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Luat {
    pub dieu_kien: Vec<(String, String, String)>,
    pub ket_luan: (String, String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TruyVan {
    pub muc_tieu: (String, String, String),
    pub rang_buoc: Option<Vec<(String, String, String)>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InRa {
    pub bieu_thuc: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VongLap {
    pub bien: String,
    pub danh_sach: Vec<String>,
    pub than: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ham {
    pub ten: String,
    pub tham_so: Vec<String>,
    pub than: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChuongTrinh {
    pub ten: String,
    pub than: Vec<Statement>,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::PhatBieu(p) => write!(f, "{} {:?} {:?}", p.chu_ngu, p.dong_tu, p.tan_ngu),
            Statement::Luat(l) => write!(f, "luật: {:?} -> {:?}", l.dieu_kien, l.ket_luan),
            Statement::TruyVan(t) => write!(f, "truy vấn: {:?}", t.muc_tieu),
            Statement::InRa(i) => write!(f, "in ra: {}", i.bieu_thuc),
            Statement::VongLap(v) => write!(f, "vòng lặp {} trong {:?}", v.bien, v.danh_sach),
            Statement::Ham(h) => write!(f, "hàm {}({:?})", h.ten, h.tham_so),
            Statement::ChuongTrinh(c) => write!(f, "chương trình '{}'", c.ten),
        }
    }
}
