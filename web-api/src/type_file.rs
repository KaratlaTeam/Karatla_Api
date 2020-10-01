use diesel::{r2d2::ConnectionManager, r2d2, MysqlConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;