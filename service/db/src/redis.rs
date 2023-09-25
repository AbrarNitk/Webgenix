type Pool = r2d2::Pool<r2d2_redis::RedisConnectionManager>;

static REDIS_POOL: once_cell::sync::OnceCell<Pool> = once_cell::sync::OnceCell::new();

pub fn get_pool(redis_url: &str) -> Pool {
    let connection_manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
    r2d2::Pool::builder()
        .max_size(10)
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(connection_manager)
        .expect("error in setting the redis pool")
}

pub fn init_redis_pool() {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL env not found");
    let connection_manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
    REDIS_POOL
        .set(
            Pool::builder()
                .build(connection_manager)
                .expect("Error in creating redis connection pool"),
        )
        .expect("Error in setting the REDIS_POOL for postgres");
}
