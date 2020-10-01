use actix_web::{HttpServer, App, };
use web_api::{database_actions, web_rount,};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");

    let bind = "127.0.0.1:8080";

    // connect data
    let pool = database_actions::connection_database_pool();

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web_rount::account_create)
            .service(web_rount::account_get_all)
            .service(web_rount::account_get)
            .service(web_rount::account_update)
            .service(web_rount::account_login_check)
            .service(web_rount::account_login)
            .service(web_rount::account_logout)
            .service(web_rount::account_validation_code)
            .service(web_rount::account_validation_code_check)
    })
    .bind(&bind)?
    .run()
    .await
}
