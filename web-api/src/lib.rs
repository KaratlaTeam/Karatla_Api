#[macro_use]
extern crate diesel;
extern crate dotenv;


pub mod models_account;
pub mod models_validation;
pub mod models_http;

pub mod database_actions_account;
pub mod database_actions_vallidation;
pub mod database_actions;

pub mod web_rount;
pub mod schema;
pub mod type_file;