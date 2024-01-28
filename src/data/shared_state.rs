use scylla::Session;

use crate::data::Queries;

pub struct SharedState {
    pub session: Session,
    pub queries: Queries,
    pub single_user: bool,
    pub product_name: &'static str,
    pub product_logo: &'static [u8],
}
