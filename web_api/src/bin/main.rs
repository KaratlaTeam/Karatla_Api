use actix_web::{HttpServer, App, web};
use web_api::{database_actions, web_rount, ssl_config, };
use actix_files::Files;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    //std::env::set_var("RUST_LOG", "actix_web=info");

    //link is "https://192.168.0.139:443";
    let bind = "127.0.0.1:8080";
    //let bind = "0.0.0.0:443";

    // load ssl key
    let config = ssl_config::ssl_load();

    // connect data
    let pool = database_actions::connection_database_pool();

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(Files::new("/static", "static"))
            //.service(web::scope("/api").configure(f))
            .service(web_rount::check_question_version)
            .service(web_rount::account_get_all)
            .service(web_rount::account_create)
            .service(web_rount::account_get)
            .service(web_rount::account_update)
            .service(web_rount::account_login_check)
            .service(web_rount::account_login)
            .service(web_rount::account_logout)
            .service(web_rount::account_validation_code)
            .service(web_rount::account_validation_code_check)

            .service(web_rount::json_get)
            .service(web_rount::question_images_get)
            .service(web_rount::academy_images_get)
    })
    .bind(&bind)?
    //.bind_rustls(&bind, config)?
    .run()
    .await
}
