use super::result::DoctorResult;

pub trait DoctorCheck {
    fn name(&self) -> &'static str;

    fn run(&self) -> DoctorResult;
}
