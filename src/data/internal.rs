use scylla::FromRow;
use scylla::SerializeRow;
use uuid::Uuid;

#[derive(SerializeRow, FromRow)]
pub struct Internal {
    pub id: Uuid,
    pub epoch: i64,
}
