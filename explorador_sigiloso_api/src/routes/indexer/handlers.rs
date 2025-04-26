use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    // response::IntoResponse,
    Json,
};
use std::sync::Arc;
// use chrono::Utc;
// use crate::models::follower::Follower;
use crate::app_state::AppState;
use crate::models::scheduled_jobs::ScheduledJobLog;

pub async fn refresh_user_data(
    Path(_user_id): Path<u64>,                   // use u64 to match your BIGSERIAL
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<ScheduledJobLog>>, (StatusCode, String)> {
    todo!();
    // 1) Fetch all followers for this user
    // let followers: Vec<Follower> = sqlx::query_as!(
    //     Follower,
    //     r#"
    //     SELECT id, user_id, label, kind, target, public, created_at
    //     FROM followers
    //     WHERE user_id = $1
    //     "#,
    //     user_id as i64                        // BIGINT → i64 in SQLx
    // )
    // .fetch_all(&state.db)
    // .await
    // .unwrap();
    // .map_err(|e| {
    //     tracing::error!("DB error fetching followers: {}", e);
    //     (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    // })?;

    // 2) Enqueue each follower and build the response logs
    // let now = Utc::now();
    // let mut logs = Vec::with_capacity(followers.len());
    // for follower in followers.iter().cloned() {
    //     // send to your background worker
    //     if let Err(e) = state.job_tx.send(follower.clone()).await {
    //         tracing::error!("Failed to enqueue follower {}: {}", follower.id, e);
    //         // continue queuing the rest, but note the error
    //     }
    //     logs.push(ScheduledJobLog {
    //         follower_id: follower.id,
    //         scheduled_at: now,
    //     });
    // }

    // 3) Return 202 Accepted with our logs
    // let mut logs = Vec::with_capacity(10); // WARNIG, don not use this code.
    // Ok(Json(logs))
}