use std::sync::Arc;

use axum::extract::Multipart;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Result;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use scylla::serialize::row::SerializeRow;
use scylla::FromRow;
use scylla::Session;
use uuid::Uuid;

use crate::models::Group;
use crate::models::PortableScheduleEntry;
use crate::models::Schedule;
use crate::models::ScheduleEntry;
use crate::models::ScheduleRequest;
use crate::models::SharedState;
use crate::models::User;

// FIXED: I no longer want to kill myself
macro_rules! query {
    ($s:expr, $q:literal, $d:expr, $e:literal) => {{
        let result = if let Ok(result) = $s.query($q, $d).await {
            result
        } else {
            return Err($e.into());
        };

        let rows = if let Some(rows) = result.rows {
            rows
        } else {
            return Err($e.into());
        };

        let row = if let Some(row) = rows.into_iter().next() {
            row
        } else {
            return Err($e.into());
        };

        if let Ok(value) = row.into_typed() {
            value
        } else {
            return Err($e.into());
        }
    }};
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
pub async fn get_schedule(
    State(state): State<Arc<SharedState>>,
    request: Query<ScheduleRequest>,
) -> axum::response::Result<Json<Vec<Vec<ScheduleEntry>>>> {
    let group_id = if let Ok(group_id) = Uuid::parse_str(&request.group_id.clone()) {
        group_id
    } else {
        return Err("Group doesnt exist".into());
    };

    let group: Group = query!(
        state.session,
        "SELECT * FROM groups WHERE id = ?",
        (group_id,),
        "Group doesnt exist"
    );

    let matching = group
        .schedule
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

    Ok(Json(days))
}

pub async fn post_import(
    State(state): State<Arc<SharedState>>,
    mut multipart: Multipart,
) -> Result<&'static str> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut group_id_data: Option<String> = None;
    let mut access_token_data: Option<String> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name();
        if name == Some("file") {
            file_data = Some(field.bytes().await?.to_vec());
        } else if name == Some("group_id") {
            group_id_data = Some(field.text().await?);
        } else if name == Some("access_token") {
            access_token_data = Some(field.text().await?);
        }
    }

    let file = if let Some(file) = file_data {
        file
    } else {
        return Err("Multipart field \"file\" not found".into());
    };

    let group_id = if let Some(group) = group_id_data {
        if let Ok(group_id) = Uuid::parse_str(&group) {
            group_id
        } else {
            return Err("Invalid UUID".into());
        }
    } else {
        return Err("Multipart field \"group_id\" not found".into());
    };

    let access_token = if let Some(access_token) = access_token_data {
        access_token
    } else {
        return Err("Multipart field \"access_token\" not found".into());
    };

    let user: User = query!(
        state.session,
        "SELECT * FROM users WHERE access_token = ?",
        (access_token,),
        "Access token is not valid"
    );

    if !user.group_scope.contains(&group_id) {
        return Err("This group does not belong to your group scope".into());
    }

    let mut reader = csv::Reader::from_reader(&*file);
    let pse: Vec<PortableScheduleEntry> = match reader.deserialize().collect::<Result<Vec<_>, _>>()
    {
        Ok(result) => result,
        Err(error) => return Err(error.to_string().into()),
    };

    let schedule: Vec<Schedule> = pse.into_iter().map(|v| v.into()).collect();

    if let Err(error) = state
        .session
        .query(
            "UPDATE groups SET schedule = ? WHERE id = ?",
            (schedule, group_id),
        )
        .await
    {
        return Err(error.to_string().into());
    };

    Ok("Import successful")
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new()
        .route("/", get(get_schedule))
        .route("/import", post(post_import))
}
