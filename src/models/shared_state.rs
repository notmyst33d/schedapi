use scylla::Session;

use crate::models::PortableScheduleEntry;

pub struct SharedState {
    pub data: Vec<PortableScheduleEntry>,
    pub session: Session,
}
