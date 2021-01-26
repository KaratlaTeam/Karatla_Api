use super::schema::validations;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Validations {
    pub id: i64,
    pub phone_number: String,
    pub phone_code: String,
    pub phone_safe_hour: u32,
    pub phone_safe_minute: u32,
    pub phone_safe_second: u32,
    pub phone_code_get_time: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="validations"]
pub struct NewValidation {
    pub phone_number: String,
    pub phone_code: String,
    pub phone_safe_hour: u32,
    pub phone_safe_minute: u32,
    pub phone_safe_second: u32,
    pub phone_code_get_time: String,
}