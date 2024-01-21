use scylla::FromRow;
use scylla::SerializeRow;
use uuid::Uuid;

#[derive(SerializeRow, FromRow)]
pub struct User {
    pub username: String,
    pub access_token: Uuid,
    pub group_scope: Option<Vec<Uuid>>,
    pub password: String,
}
