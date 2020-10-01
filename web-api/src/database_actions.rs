use diesel::r2d2;
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;

pub fn connection_database_pool() -> r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("database_url: {}",database_url);

    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Faild to create pool");

    pool
}