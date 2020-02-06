use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

mod pagination;

pub use pagination::Paginate;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
        env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
        env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
        env::var("POSTGRES_DB").unwrap_or_else(|_| "graphqlapi".to_string())
    );
    info!("Connect to db at {}", db_url);
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
