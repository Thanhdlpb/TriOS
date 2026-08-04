#[derive(Debug, Clone)]
pub enum Status {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone)]
pub struct DoctorResult {
    pub name: String,
    pub status: Status,
    pub message: String,
}

impl DoctorResult {
    pub fn pass(name: &str, msg: &str) -> Self {
        Self {
            name: name.into(),
            status: Status::Pass,
            message: msg.into(),
        }
    }

    pub fn warn(name: &str, msg: &str) -> Self {
        Self {
            name: name.into(),
            status: Status::Warn,
            message: msg.into(),
        }
    }

    pub fn fail(name: &str, msg: &str) -> Self {
        Self {
            name: name.into(),
            status: Status::Fail,
            message: msg.into(),
        }
    }
}
