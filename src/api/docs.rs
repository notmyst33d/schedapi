use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::schedule::get_schedule,
    ),
    components(
        schemas(
            crate::models::ScheduleEntry,
        ),
    ),
    info(
        title = "Schedule API",
        description = "Open source API for college schedules",
    ),
)]
pub struct Docs;

