use scylla::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct UserComposite {
    pub access_token: Uuid,
    pub username: String,
}
