use serde::Serialize;
use chrono::{DateTime, Utc};

/// What we return to the client after scheduling each job
#[derive(Serialize)]
pub struct ScheduledJobLog {
    pub id: u64,
    pub follower_id: u64,
    pub scheduled_by: String, // staff or user or another job?
    pub scheduled_at: DateTime<Utc>,
}