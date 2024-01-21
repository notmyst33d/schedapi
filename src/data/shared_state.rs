use scylla::Session;

use crate::data::Queries;

pub struct SharedState {
    pub session: Session,
    pub queries: Queries,
    pub single_user: bool,
}
