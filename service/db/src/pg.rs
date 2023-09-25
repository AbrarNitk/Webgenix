pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub fn get_connection_pool(url: &str) -> DbPool {
    let connection_manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(url);
    diesel::r2d2::Pool::builder()
        .max_size(10)
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(connection_manager)
        .expect("Error in building the connection pool for postgres")
}
