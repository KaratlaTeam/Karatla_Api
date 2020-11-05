use crate::database_actions_account;
use crate::database_actions_vallidation;
use crate::models_http;
use crate::type_file;
use actix_web::{client::Client, get, post, web, Error, HttpResponse};
use rand::prelude::*;
use serde_json;
use tokio;

// get code
#[get("/api/account/validation/code/{phone}")]
pub async fn account_validation_code(
    phone_n: web::Path<i64>,
    pool: web::Data<type_file::DbPool>,
) -> Result<HttpResponse, Error> {
    let mut map_data = serde_json::Map::new();
    let state_code;
    let connection1 = pool.clone().get().expect("Fail get connection from pool!");
    let connection2 = pool.clone().get().expect("Fail get connection from pool!");

    // calculate validation code
    let mut rng = thread_rng();
    let rng_number = rng.gen_range(100000, 999999);
    println!("new validation code is: {}", rng_number);
    println!("request phone is: {}", phone_n);

    // edit validation code service url
    let head = "http://sms.360.my/gw/bulk360/v1.4?".to_string();
    let user = "user=3164926@qq.com".to_string();
    let pass = "&pass=MSj3164926".to_string();
    let a_type = "&type=0".to_string();
    let to = "&to=".to_string();
    let phone_number = phone_n.clone().to_string();
    let from = "&from=PPM".to_string();
    let text1 = "&text=[PPM]:+Your+validation+code+is:+".to_string();
    let validation_code = rng_number.to_string();
    let text2 = ".+Do+not+share+this+code+with+others.".to_string();
    let serv_id = "&servid=Karatla".to_string();
    let url = format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        head, user, pass, a_type, to, phone_number, from, text1, validation_code, text2, serv_id
    );
    let url = url.as_str();
    println!("connect: {}", url);

    // send validation code to customer
    let request = Client::default()
        .get("http://www.google.com")
        .send()
        .await
        .unwrap();
    let re_state = request.status().to_string();
    println!("{:?}", re_state);

    if re_state.as_str().eq("200 OK") {
        // insert validation code
        let phone1 = phone_n.clone().to_string();
        let new_validation = web::block(move || {
            database_actions_vallidation::insert_data_validation(
                &connection1,
                &phone1,
                &validation_code,
            )
        })
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        println!(
            "new_validation: phone_number {}, phone_code {}",
            new_validation.phone_number, new_validation.phone_code
        );

        // delete code after 60s
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        let phone2 = phone_n.clone().to_string();
        let delete_n = web::block(move || {
            database_actions_vallidation::delete_data_validation(&connection2, &phone2)
        })
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

        if delete_n.unwrap().to_string().as_str() != "0" {
            state_code = 1801;
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("delete successful".to_string()),
            );
            println!("delete successful")
        } else {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("delete fail".to_string()),
            );
            state_code = 1811;
        }
    } else {
        state_code = 1810;
        map_data.insert(
            "state".to_string(),
            serde_json::Value::String("send code fail".to_string()),
        );
    }

    let back_data = models_http::JsonData {
        code: state_code,
        data: map_data,
    };
    Ok(HttpResponse::Ok().json(back_data))
}

// check code
#[post("/api/account/validation/code/check/{phone}")]
pub async fn account_validation_code_check(
    phone: web::Path<i64>,
    form: web::Json<models_http::JsonData>,
    pool: web::Data<type_file::DbPool>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Fail get connection from pool!");
    let connection2 = pool.get().expect("Fail get connection from pool!");
    let mut map_data = serde_json::Map::new();
    let phone_n = phone.clone().to_string();
    let phone_n2 = phone.clone().to_string();

    if form.code == 1900 {
        let back_code: i32;

        let account_amount = web::block(move || {
            database_actions_account::check_account_exit(&connection2, &phone_n2)
        })
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
        println!("Phone amounts: {}", account_amount);

        if account_amount > 0 {
            back_code = 1912;
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("Phone already exit".to_string()),
            );
        } else {
            let back = web::block(move || {
                database_actions_vallidation::check_data_validation(
                    &connection,
                    &form
                        .data
                        .get("v_code")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                    &phone_n,
                )
            })
            .await
            .map_err(|e| {
                eprint!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

            if back == 1911 {
                back_code = 1911;
                map_data.insert(
                    "state".to_string(),
                    serde_json::Value::String("phone code wrong".to_string()),
                );
            } else {
                back_code = 1901;
                map_data.insert(
                    "state".to_string(),
                    serde_json::Value::String("code right".to_string()),
                );
            }
        }

        let back_data = models_http::JsonData {
            code: back_code,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    } else {
        map_data.insert(
            "state".to_string(),
            serde_json::Value::String("request code wrong".to_string()),
        );
        let back_data = models_http::JsonData {
            code: 1910,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    }
}

// get all accounts
#[get("/api/account/get/all")]
pub async fn account_get_all(pool: web::Data<type_file::DbPool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Fail get connection from pool!");

    let accounts = web::block(move || database_actions_account::get_data_all_accounts(&connection))
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(accounts) = accounts {
        Ok(HttpResponse::Ok().json(accounts))
    } else {
        let res = HttpResponse::NotFound().body(format!("Not find any data"));
        Ok(res)
    }
}

// create new account
#[post("/api/account/new")]
pub async fn account_create(
    pool: web::Data<type_file::DbPool>,
    form: web::Json<models_http::JsonData>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Fail get connection from pool!");
    let mut map_data = serde_json::Map::new();

    if form.code == 1100 {
        let account = web::block(move || {
            database_actions_account::insert_data_account(&connection, &form.data)
        })
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        println!(
            "new account successful: uuid: {}, password: {}, phone: {}",
            account.my_uuid, account.my_password, account.my_phone
        );

        map_data.insert(
            "my_uuid".to_string(),
            serde_json::Value::String(account.my_uuid),
        );
        let back_data = models_http::JsonData {
            code: 1101,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    } else {
        map_data.insert(
            "state".to_string(),
            serde_json::Value::String("code wrong".to_string()),
        );
        let back_data = models_http::JsonData {
            code: 1110,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    }
}

//get a account data
#[post("/api/account/get")]
pub async fn account_get(
    pool: web::Data<type_file::DbPool>,
    form: web::Json<models_http::JsonData>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Fail get connection from pool!");
    let mut map_data = serde_json::Map::new();

    if form.code == 1400 {
        let account =
            web::block(move || database_actions_account::get_data_account(&connection, &form.data))
                .await
                .map_err(|e| {
                    eprint!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
        let account = account.unwrap();

        println!(
            "get account successful: id: {}, password: {}, phone: {}",
            account.id, account.my_password, account.my_phone
        );

        map_data.insert(
            "id".to_string(),
            serde_json::Value::String(account.id.to_string()),
        );
        map_data.insert(
            "my_uuid".to_string(),
            serde_json::Value::String(account.my_uuid),
        );
        map_data.insert(
            "my_password".to_string(),
            serde_json::Value::String(account.my_password),
        );
        map_data.insert(
            "my_state".to_string(),
            serde_json::Value::String(account.my_state),
        );
        map_data.insert(
            "my_name".to_string(),
            serde_json::Value::String(account.my_name),
        );
        map_data.insert(
            "my_phone".to_string(),
            serde_json::Value::String(account.my_phone),
        );
        map_data.insert(
            "my_email".to_string(),
            serde_json::Value::String(account.my_email.unwrap_or_default()),
        );
        map_data.insert(
            "my_photo".to_string(),
            serde_json::Value::String(account.my_photo.unwrap_or_default()),
        );
        map_data.insert(
            "my_login_time_now".to_string(),
            serde_json::Value::String(account.my_login_time_now),
        );
        map_data.insert(
            "my_login_ip_now".to_string(),
            serde_json::Value::String(account.my_login_ip_now),
        );
        map_data.insert(
            "my_login_time_last".to_string(),
            serde_json::Value::String(account.my_login_time_last),
        );
        map_data.insert(
            "my_login_ip_last".to_string(),
            serde_json::Value::String(account.my_login_ip_last),
        );
        map_data.insert(
            "my_register_time".to_string(),
            serde_json::Value::String(account.my_register_time),
        );
        map_data.insert(
            "my_device_id_now".to_string(),
            serde_json::Value::String(account.my_device_id_now),
        );
        map_data.insert(
            "my_device_id_last".to_string(),
            serde_json::Value::String(account.my_device_id_last),
        );
        map_data.insert(
            "my_login_type".to_string(),
            serde_json::Value::String(account.my_login_type),
        );

        let back_data = models_http::JsonData {
            code: 1401,
            data: map_data,
        };
        Ok(HttpResponse::Ok().json(back_data))
    } else {
        map_data.insert(
            "state".to_string(),
            serde_json::Value::String("code wrong".to_string()),
        );
        let back_data = models_http::JsonData {
            code: 1410,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    }
}

//update account data
#[post("/api/account/update")]
pub async fn account_update(
    pool: web::Data<type_file::DbPool>,
    form: web::Json<models_http::JsonData>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Fail get connection from pool!");
    let mut map_data = serde_json::Map::new();

    if form.code == 1200 {
        let back_code: i32;

        let num = web::block(move || {
            database_actions_account::update_data_account(&connection, &form.data)
        })
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

        if num != 0 {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("update successful".to_string()),
            );
            back_code = 1201;
        } else {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("did not find update data".to_string()),
            );
            back_code = 1211;
        }

        let back_data = models_http::JsonData {
            code: back_code,
            data: map_data,
        };
        Ok(HttpResponse::Ok().json(back_data))
    } else {
        map_data.insert(
            "state".to_string(),
            serde_json::Value::String("code wrong".to_string()),
        );
        let back_data = models_http::JsonData {
            code: 1210,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    }
}

// check_account_login_device
#[post("/api/account/check")]
pub async fn account_login_check(
    pool: web::Data<type_file::DbPool>,
    form: web::Json<models_http::JsonData>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Fail get connection from pool!");
    let mut map_data = serde_json::Map::new();

    if form.code == 1600 {
        let back_code = web::block(move || {
            database_actions_account::check_account_login_device(&connection, &form.data)
        })
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

        if back_code == 1601 {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("device same".to_string()),
            );
        } else if back_code == 1602 {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("device different, automatic logout!".to_string()),
            );
        } else if back_code == 1603 {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("account different, automatic logout!".to_string()),
            );
        }

        let back_data = models_http::JsonData {
            code: back_code,
            data: map_data,
        };
        Ok(HttpResponse::Ok().json(back_data))
    } else {
        map_data.insert(
            "state".to_string(),
            serde_json::Value::String("code wrong".to_string()),
        );
        let back_data = models_http::JsonData {
            code: 1610,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    }
}

// account login
#[post("/api/account/login")]
pub async fn account_login(
    pool: web::Data<type_file::DbPool>,
    form: web::Json<models_http::JsonData>,
    request: web::HttpRequest,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Fail get connection from pool!");
    let mut map_data = serde_json::Map::new();
    let req = request.clone();
    let req_socket = req.peer_addr().unwrap();
    let req_ip = req_socket.ip().to_string();

    println!("login ip: {}", req_ip);

    if form.code == 1500 {
        let back_json = web::block(move || {
            database_actions_account::login_account(&connection, &form.data, &req_ip)
        })
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

        let back_code = back_json.code;
        let back_data = back_json.data.clone();
        let back_uuid = back_data
            .get("my_uuid")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        print!("Login uuid: {}", &back_uuid);

        if back_code == 1501 {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("successful login accounts".to_string()),
            );
            map_data.insert("my_uuid".to_string(), serde_json::Value::String(back_uuid));
            println!("successful login accounts")
        } else if back_code == 1511 {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("account login type code wrong".to_string()),
            );
            println!("account login type code wrong")
        } else if back_code == 1512 {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("password wrong".to_string()),
            );
            println!("password wrong")
        }

        let back_data = models_http::JsonData {
            code: back_code,
            data: map_data,
        };
        Ok(HttpResponse::Ok().json(back_data))
    } else {
        map_data.insert(
            "state".to_string(),
            serde_json::Value::String("code wrong".to_string()),
        );
        println!("code wrong");
        let back_data = models_http::JsonData {
            code: 1510,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    }
}

// account logout
#[post("/api/account/logout")]
pub async fn account_logout(
    pool: web::Data<type_file::DbPool>,
    form: web::Json<models_http::JsonData>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Fail get connection from pool!");
    let mut map_data = serde_json::Map::new();

    if form.code == 1700 {
        let back_code =
            web::block(move || database_actions_account::logout_account(&connection, &form.data))
                .await
                .map_err(|e| {
                    eprint!("{}", e);
                    HttpResponse::InternalServerError().finish();
                })?;

        if back_code == 1701 {
            map_data.insert(
                "state".to_string(),
                serde_json::Value::String("logout account".to_string()),
            );
        }

        let back_data = models_http::JsonData {
            code: back_code,
            data: map_data,
        };
        Ok(HttpResponse::Ok().json(back_data))
    } else {
        map_data.insert(
            "state".to_string(),
            serde_json::Value::String("code wrong".to_string()),
        );
        let back_data = models_http::JsonData {
            code: 1710,
            data: map_data,
        };

        Ok(HttpResponse::Ok().json(back_data))
    }
}
