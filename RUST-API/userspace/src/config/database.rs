use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::pg::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool!")
}
