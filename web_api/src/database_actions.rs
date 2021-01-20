use diesel::r2d2;
use dotenv::dotenv;
//use std::env;
use diesel::prelude::*;

pub fn connection_database_pool() -> r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    //let database_url = env::var("DATABASE_URL")
    //    .expect("DATABASE_URL must be set");
    //println!("database_url: {}",database_url);
    let database_url = read_env();

    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Faild to create pool");

    pool
}

fn read_env() -> String {
    //use std::fs::File;
    //use std::io::Read;
    println!("Please write Mysql address:");
    //let mut file = File::open("static/.env").unwrap();
    let mut env = String::new();
    std::io::stdin().read_line(&mut env).unwrap();
    //file.read_to_string(&mut env).unwrap();

    println!("read env url: {}",env);

    env

}