use scylla::Session;
use uuid::uuid;

use crate::data::{Internal, Queries};
use crate::{query_checked, query_one_checked};

pub async fn init(session: &Session, queries: &Queries) {
    let internal_record: Result<Internal, _> =
        query_one_checked!(session, &queries.get_internal, ());

    if let Ok(_) = internal_record {
        return;
    }

    let internal = Internal {
        id: uuid!("00000000-0000-0000-0000-000000000000"),
        epoch: 0,
    };

    let _ = query_checked!(session, &queries.add_internal, internal);
}
