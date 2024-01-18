use std::sync::Arc;

use axum::extract::Multipart;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Result;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use uuid::Uuid;

use crate::models::dogfood;
use crate::models::*;

pub async fn get_schedule(
    State(state): State<Arc<SharedState>>,
    request: Query<dogfood::ScheduleRequest>,
) -> Result<Json<Vec<Vec<ScheduleEntry>>>> {
    if let Ok(data) = state
        .session
        .query(
            "SELECT schedule FROM groups WHERE name = ?",
            (&request.group,),
        )
        .await
    {
        println!("{:#?}", data.rows);
    } else {
        return Err("Query failed".into());
    }

    Ok(Json(vec![vec![]]))
}

pub async fn post_import(
    State(state): State<Arc<SharedState>>,
    mut multipart: Multipart,
) -> Result<&'static str> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut group_data: Option<String> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name();
        if name == Some("file") {
            file_data = Some(field.bytes().await?.to_vec());
        } else if name == Some("group") {
            group_data = Some(field.text().await?);
        }
    }

    let file = if let Some(file) = file_data {
        file
    } else {
        return Err("Multipart field \"file\" not found".into());
    };

    let group = if let Some(group) = group_data {
        group
    } else {
        return Err("Multipart field \"group\" not found".into());
    };

    let mut reader = csv::Reader::from_reader(&*file);
    let pse: Vec<PortableScheduleEntry> = match reader.deserialize().collect::<Result<Vec<_>, _>>()
    {
        Ok(result) => result,
        Err(error) => return Err(error.to_string().into()),
    };

    let schedule: Vec<dogfood::ScheduleRow> = pse.into_iter().map(|v| v.into()).collect();

    let mut rows = match state
        .session
        .query(
            "SELECT id FROM groups_name_id_composite WHERE name = ?",
            (&group,),
        )
        .await
    {
        Ok(result) => match result.rows_typed::<(Uuid,)>() {
            Ok(result) => result,
            Err(error) => return Err(error.to_string().into()),
        },
        Err(error) => return Err(error.to_string().into()),
    };
    let uuid = rows.next().unwrap().unwrap().0;

    if let Err(error) = state
        .session
        .query(
            "UPDATE groups SET schedule = ? WHERE id = ?",
            (schedule, uuid),
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
