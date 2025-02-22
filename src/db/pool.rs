use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.")
}
