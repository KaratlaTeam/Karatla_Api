use super::schema::accounts;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Accounts {
    pub id: i64,
    pub my_uuid: String,
    pub my_password: String,
    /// "ON": Online, "OFF": Logout, "IN": Not Online.
    pub my_state: String,
    pub my_name: String,
    pub my_phone: String,
    pub my_email: Option<String>,
    pub my_photo: Option<String>,
    pub my_login_type: String,
    
    pub my_device_id_now: String,
    pub my_device_id_last: String,
    
    pub my_login_time_now: String,
    pub my_login_time_last: String,
    
    pub my_login_ip_now: String,
    pub my_login_ip_last: String,
    
    pub my_register_time: String,
    
    // my_favorite_list_part_list

    // my_test_answer_all_model_list
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="accounts"]
pub struct NewAccount{
    pub my_uuid: String,
    pub my_password: String,
    /// "ON": Online, "OFF": Logout, "IN": Not Online.
    pub my_state: String,
    pub my_name: String,
    pub my_phone: String,
    pub my_email: Option<String>,
    pub my_photo: Option<String>,
    pub my_login_type: String,
    
    pub my_device_id_now: String,
    pub my_device_id_last: String,
    
    pub my_login_time_now: String,
    pub my_login_time_last: String,
    
    pub my_login_ip_now: String,
    pub my_login_ip_last: String,
    
    pub my_register_time: String,
}
