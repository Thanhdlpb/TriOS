use std::fs;
use std::path::Path;

pub struct Source {
    ten: String,
    noi_dung: String,
}

impl Source {
    pub fn mo<P: AsRef<Path>>(duong_dan: P) -> std::io::Result<Self> {
        let noi_dung = fs::read_to_string(&duong_dan)?;
        Ok(Self {
            ten: duong_dan.as_ref().display().to_string(),
            noi_dung,
        })
    }

    pub fn tu_chuoi(ten: &str, noi_dung: &str) -> Self {
        Self {
            ten: ten.to_string(),
            noi_dung: noi_dung.to_string(),
        }
    }

    pub fn ten(&self) -> &str {
        &self.ten
    }

    pub fn noi_dung(&self) -> &str {
        &self.noi_dung
    }
}
