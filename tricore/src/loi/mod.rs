use crate::vi_tri::ViTri;

#[derive(Clone, Debug)]
pub struct LoiTri {
    pub thong_diep: String,
    pub vi_tri: Option<ViTri>,
}
