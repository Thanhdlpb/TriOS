use crate::vi_tri::ViTri;

#[derive(Clone, Debug, PartialEq)]
pub enum LoaiToken {
    EOF,

    DinhDanh,
    So,
    Chuoi,

    Bien,
    Hang,
    Ham,
    Neu,
    Thi,
    NguocLai,
    TraVe,
    InRa,

    Gan,

    Cong,
    Tru,
    Nhan,
    Chia,

    Bang,
    Khac,
    Lon,
    LonBang,
    Nho,
    NhoBang,

    MoNgoac,
    DongNgoac,

    MoNgoacNhan,
    DongNgoacNhan,

    Phay,
    ChamPhay,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub loai: LoaiToken,
    pub gia_tri: String,
    pub vi_tri: ViTri,
}
