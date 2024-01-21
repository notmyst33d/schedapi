use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScheduleRequest {
    pub week: i32,
    pub group_id: String,
}
