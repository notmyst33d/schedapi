use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::api::schedule::get_schedule),
    components(schemas(crate::data::ScheduleEntry)),
    info(
        title = "Schedule API",
        description = "Open source API for college schedules",
    )
)]
pub struct Docs;
