use scylla::FromRow;
use scylla::SerializeRow;
use uuid::Uuid;

#[derive(SerializeRow, FromRow)]
pub struct User {
    pub access_token: String,
    pub group_scope: Vec<Uuid>,
}
