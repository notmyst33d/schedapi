use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ScheduleEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lesson_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditorium: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
}
