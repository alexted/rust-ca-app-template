use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub fn db_pool(db_dsn: String) -> DBConn {
    let manager = ConnectionManager::<PgConnection>::new(db_dsn);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
