use std::env;
use std::sync::Arc;
use sqlx::Postgres;
use sqlx::Pool;

mod types;
pub mod handlers;

pub async fn try_connect_postgres() -> Option<Arc<Pool<Postgres>>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = Pool::<Postgres>::connect(&url)
        .await
        .ok()
        .map(Arc::new);

    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    pool
}
