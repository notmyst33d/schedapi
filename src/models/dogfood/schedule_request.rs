use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScheduleRequest {
    pub week: u8,
    pub group: String,
}
