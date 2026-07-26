use crate::token::LoaiToken;

pub fn tim_tu_khoa(text: &str) -> Option<LoaiToken> {
    match text {
        "biến" => Some(LoaiToken::Bien),
        "hằng" => Some(LoaiToken::Hang),
        "hàm" => Some(LoaiToken::Ham),
        "nếu" => Some(LoaiToken::Neu),
        "thì" => Some(LoaiToken::Thi),
        "ngược_lại" => Some(LoaiToken::NguocLai),
        "trả_về" => Some(LoaiToken::TraVe),
        "in_ra" => Some(LoaiToken::InRa),
        _ => None,
    }
}
