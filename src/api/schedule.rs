use std::sync::Arc;

use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use axum::Json;
use axum::Router;

use crate::models::PortableScheduleEntry;
use crate::models::ScheduleEntry;
use crate::models::ScheduleRequest;
use crate::models::SharedState;

#[utoipa::path(
    get,
    path = "/schedule",
    params(
        ("week", Query, description = "Week number"),
    ),
    responses(
        (status = 200, description = "Returns the schedule for the specified week", body = Vec<Vec<ScheduleEntry>>),
        (status = 400, description = "Incorrect query")
    ),
)]
pub async fn get_schedule(
    State(state): State<Arc<SharedState>>,
    request: Query<ScheduleRequest>,
) -> Json<Vec<Vec<ScheduleEntry>>> {
    let matching = state
        .data
        .iter()
        .filter(|e| {
            let even = e.even.unwrap_or(false);
            let odd = e.odd.unwrap_or(false);
            let mut even_odd_check = true;
            if even || odd {
                even_odd_check = (even && request.week % 2 == 0) || (odd && request.week % 2 != 0);
            }
            request.week >= e.week_range.start && request.week <= e.week_range.end && even_odd_check
        })
        .collect::<Vec<&PortableScheduleEntry>>();

    let mut days: Vec<Vec<ScheduleEntry>> = vec![];
    for i in 1..8 {
        let mut lesson_num = 1;
        let mut final_lessons: Vec<ScheduleEntry> = vec![];
        let mut lessons = matching
            .iter()
            .filter(|e| e.day == i)
            .collect::<Vec<&&PortableScheduleEntry>>();

        lessons.sort_by_key(|e| e.num);

        for lesson in lessons.iter() {
            if lesson_num < lesson.num {
                for _ in 0..lesson.num - lesson_num {
                    final_lessons.push(ScheduleEntry {
                        name: None,
                        lesson_type: None,
                        teacher: None,
                        auditorium: None,
                        empty: Some(true),
                    });
                    lesson_num += 1;
                }
            }

            final_lessons.push(ScheduleEntry {
                name: Some(lesson.name.clone()),
                lesson_type: lesson.lesson_type.clone(),
                teacher: lesson.teacher.clone(),
                auditorium: Some(lesson.auditorium.clone()),
                empty: None,
            });

            lesson_num += 1;
        }

        days.push(final_lessons);
    }

    Json(days)
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new().route("/", get(get_schedule))
}
