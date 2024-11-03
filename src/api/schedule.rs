use crate::bad_request;
use crate::data::*;
use crate::SharedState;
use axum::extract::{Multipart, Query, State};
use axum::response::{ErrorResponse, IntoResponse, Result};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct ScheduleRequest {
    week: i32,
    group_id: i64,
}

#[utoipa::path(
    get,
    path = "/schedule",
    params(
        ("week", Query, description = "Week number"),
        ("group_id", Query, description = "Group ID"),
    ),
    responses(
        (status = 200, description = "Returns the schedule for the specified week", body = Vec<Vec<ScheduleEntry>>),
        (status = 400, description = "Incorrect query")
    ),
)]
async fn get_schedule(
    State(state): State<Arc<SharedState>>,
    request: Query<ScheduleRequest>,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    let group = state.storage.get_group(request.group_id).await?;
    let schedule = if let Some(schedule) = group.schedule {
        schedule
    } else {
        return Ok(Json(ApiResponse::Ok(vec![vec![]])));
    };

    let matching = schedule
        .iter()
        .filter(|e| {
            let even = e.even_odd == EvenOdd::Even;
            let odd = e.even_odd == EvenOdd::Odd;
            let mut even_odd_check = true;
            if even || odd {
                even_odd_check = (even && request.week % 2 == 0) || (odd && request.week % 2 != 0);
            }
            for range in &e.week_ranges {
                if request.week >= range.start && request.week <= range.end && even_odd_check {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<&Schedule>>();

    let mut days: Vec<Vec<ScheduleEntry>> = vec![];
    for i in 1..8 {
        let mut lesson_num = 1;
        let mut final_lessons: Vec<ScheduleEntry> = vec![];
        let mut lessons = matching
            .iter()
            .filter(|e| e.day == i)
            .collect::<Vec<&&Schedule>>();

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

    Ok(Json(ApiResponse::Ok(days)))
}

async fn post_import(
    State(state): State<Arc<SharedState>>,
    mut multipart: Multipart,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    let mut file: Option<Vec<u8>> = None;
    let mut group_id: Option<i64> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name();
        if name == Some("file") {
            file = Some(field.bytes().await?.to_vec());
        } else if name == Some("group_id") {
            group_id = Some(field.text().await?.parse::<i64>().unwrap());
        }
    }

    let Some(file) = file else {
        return Err(bad_request("form_field_empty_file"));
    };

    let Some(group_id) = group_id else {
        return Err(bad_request("form_field_empty_group_id"));
    };

    let mut reader = csv::Reader::from_reader(&*file);
    let pse: Vec<PortableScheduleEntry> = match reader.deserialize().collect::<Result<Vec<_>, _>>()
    {
        Ok(result) => result,
        Err(error) => return Err(error.to_string().into()),
    };

    let schedule: Vec<Schedule> = pse.into_iter().map(|v| v.into()).collect();

    state
        .storage
        .update_schedule(group_id, Some(sqlx::types::Json(schedule)))
        .await?;

    Ok(Json(ApiResponse::Ok(())))
}

pub async fn routes(state: Arc<SharedState>) -> Router<Arc<SharedState>> {
    let mut authorized_routes = state.authorized_routes.write().await;
    authorized_routes.push("/schedule/import");
    Router::new()
        .route("/", get(get_schedule))
        .route("/import", post(post_import))
}
