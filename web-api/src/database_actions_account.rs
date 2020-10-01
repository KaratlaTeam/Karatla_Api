use diesel::prelude::*;
use chrono::prelude::*;
use uuid::Uuid;
use serde_json;
use crate::models_http;
use crate::models_account;
use crate::schema;

pub fn get_data_all_accounts (
    connection: &MysqlConnection,
) -> Result<Option<Vec<models_account::Accounts>>, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;

    let account_get_all = accounts
        .load::<models_account::Accounts>(connection)
        .optional()?;
    Ok(account_get_all)
}

pub fn get_data_account (
    connection: &MysqlConnection,
    map_data: &serde_json::Map<String, serde_json::Value>
) -> Result<Option<models_account::Accounts>, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;

    let data_uuid = map_data.get("my_uuid").unwrap().as_str().unwrap();

    let account_get_by_uuid = accounts
        .filter(my_uuid.eq(data_uuid.to_string()))
        .first::<models_account::Accounts>(connection)
        .optional()?;

    Ok(account_get_by_uuid)
}

pub fn insert_data_account(
    connection: &MysqlConnection,
    map_data: &serde_json::Map<String, serde_json::Value>
) -> Result<models_account::NewAccount, diesel::result::Error> {
    
    //time now
    let time_now = Local::now();
    println!("register time: {}",time_now.to_string());

    // new uuid
    let new_uuid = Uuid::new_v4();
    println!("new uid: {}",new_uuid.to_string());

    //new account
    let new_account = models_account::NewAccount{
        my_uuid: new_uuid.to_string(),
        my_password: map_data.get("my_password").unwrap().as_str().unwrap().to_string(),
        my_state: "OFF".to_string(),
        my_name: map_data.get("my_name").unwrap().as_str().unwrap().to_string(),
        my_phone: map_data.get("my_phone").unwrap().as_str().unwrap().to_string(),
        my_email: Some("".to_string()),
        my_photo: Some("".to_string()),
        my_device_id_now: "".to_string(),
        my_device_id_last: "".to_string(),
        my_login_time_now: "".to_string(),
        my_login_ip_now: "".to_string(),
        my_login_time_last: "".to_string(),
        my_login_ip_last: "".to_string(),
        my_register_time: time_now.to_string() ,
        my_login_type: "PHONE".to_string(),
    };

    // insert data
    diesel::insert_into(schema::accounts::table)
        .values(&new_account)
        .execute(connection)
        .expect("Error saving new post");
        
    Ok(new_account)
}


pub fn update_data_account(
    connection: &MysqlConnection,
    map_data: &serde_json::Map<String, serde_json::Value>
) -> Result<usize, diesel::result::Error>{
    use crate::schema::accounts::dsl::*;

    let mut num_update = 0;

    // get update account uuid
    let update_uuid = map_data.get("my_uuid").unwrap().as_str().unwrap();

    // get map keys
    let map_keys: Vec<_> = map_data.keys().cloned().collect();

    for key in map_keys{

        // update name
        if key.to_string().eq("my_name"){

            print!("update data: {}",key.to_string());
            
            // get update data
            let update_data= map_data.get(key.as_str()).unwrap().as_str().unwrap();
             
            //find update
            let find_update_data = diesel::update(accounts.filter(my_uuid.like(update_uuid.to_string())));

            let amount = find_update_data.set(my_name.eq(update_data.to_string()))
                .execute(connection)
                .unwrap();
            num_update = amount;

            ////update phone
        }else if key.to_string().eq("my_phone"){

            print!("update data: {}",key.to_string());

            // get update data
            let update_data= map_data.get(key.as_str()).unwrap().as_str().unwrap();
            
            //find update
            let find_update_data = diesel::update(accounts.filter(my_uuid.like(update_uuid.to_string())));

            let amount = find_update_data.set(my_phone.eq(update_data.to_string()))
                .execute(connection)
                .unwrap();

            num_update = amount;

            //update state
        }
    }

    if num_update != 0 {
        println!("update account successful: uuid: {}, update amount: {}",update_uuid.to_string(), num_update.to_string(),);
    }

    Ok(num_update)

}


pub fn delete_data_account(
    connection: &MysqlConnection, 
    account: &str
){
    use crate::schema::accounts::dsl::*;
    
    let num_deleted = diesel::delete(schema::accounts::table.filter(my_uuid.like(account)))
        .execute(connection)
        .expect("Error deleting account");

    println!("Deleted {}, all: {}",account ,num_deleted);
}

pub fn login_account(
    connection: &MysqlConnection,
    map_data: &serde_json::Map<String, serde_json::Value>,
    req_ip: &String
) -> Result<models_http::JsonData, diesel::result::Error>{

    use crate::schema::accounts::dsl::*;

    let back_code;

    let mut m_uuid = "".to_string();
    
    // get login account ip
    let ip_new = req_ip.clone();

    // get login account my_login_type
    let login_type = map_data.get("my_login_type").unwrap().as_str().unwrap();
    print!("login type: {}",login_type);

    // get login account device_id
    let device_id_new = map_data.get("my_device_id_now").unwrap().as_str().unwrap();

    // get login update_data_my_password
    let update_data_my_password = map_data.get("my_password").unwrap().as_str().unwrap();

    // get login_account
    let login_account = map_data.get("login_account").unwrap().as_str().unwrap();

    print!("login account: {}",login_account.to_string());


    if login_type.to_string().eq("PHONE"){

        let account_get: models_account::Accounts = accounts
            .filter(my_phone.eq(login_account.to_string()))
            .first::<models_account::Accounts>(connection)
            .unwrap();

        m_uuid = account_get.my_uuid;

        // find update
        let find_update_data = diesel::update(accounts.filter(my_phone.like(login_account.to_string())));


        // use now data pass to last data
        let account_login_ip_in_database = account_get.my_login_ip_now;
        let account_login_time_in_database = account_get.my_login_time_now;

        //let account_state_in_database = account_get_by_uuid.my_state;
        let device_id_in_database = account_get.my_device_id_now;
        let my_password_in_data = account_get.my_password;


        // password correct
        if update_data_my_password.eq(my_password_in_data.as_str()){
            back_code = 1501;

        // update account state
        find_update_data.clone().set(my_state.eq("ON".to_string()))
            .execute(connection)
            .unwrap();

        // update my_device_id_last
        find_update_data.clone().set(my_device_id_last.eq(device_id_in_database))
            .execute(connection)
            .unwrap();

        // update my_login_time_last
        find_update_data.clone().set(my_login_time_last.eq(account_login_time_in_database))
            .execute(connection)
            .unwrap();

        // update my_login_ip_last
        find_update_data.clone().set(my_login_ip_last.eq(account_login_ip_in_database))
            .execute(connection)
            .unwrap();

        // update my_device_id_now
        find_update_data.clone().set(my_device_id_now.eq(device_id_new.to_string()))
            .execute(connection)
            .unwrap();

        // update my_login_ip_now
        find_update_data.clone().set(my_login_ip_now.eq(ip_new))
            .execute(connection)
            .unwrap();

        //login time now
        let time_now = Local::now().to_string();
        println!("login time: {}",time_now);

        // update my_login_ip_now
        find_update_data.clone().set(my_login_time_now.eq(time_now))
            .execute(connection)
            .unwrap();
        }else{
            back_code = 1512;
        }




    }else if login_type.to_string().eq("EMAIL"){

        // get login account email
        let email = map_data.get("my_email").unwrap().as_str().unwrap();

        let account_get: models_account::Accounts = accounts
        .filter(my_email.eq(email.to_string()))
        .first::<models_account::Accounts>(connection)
        .unwrap();

        m_uuid = account_get.my_uuid;

        // find update
        let find_update_data = diesel::update(accounts.filter(my_email.like(email.to_string())));


        // use now data pass to last data
    let account_login_ip_in_database = account_get.my_login_ip_now;
    let account_login_time_in_database = account_get.my_login_time_now;

    //let account_state_in_database = account_get_by_uuid.my_state;
    let device_id_in_database = account_get.my_device_id_now;
    let my_password_in_data = account_get.my_password;


    // password correct
    if update_data_my_password.eq(my_password_in_data.as_str()){
        back_code = 1501;

    // update account state
    find_update_data.clone().set(my_state.eq("ON".to_string()))
        .execute(connection)
        .unwrap();

    // update my_device_id_last
    find_update_data.clone().set(my_device_id_last.eq(device_id_in_database))
        .execute(connection)
        .unwrap();

    // update my_login_time_last
    find_update_data.clone().set(my_login_time_last.eq(account_login_time_in_database))
        .execute(connection)
        .unwrap();

    // update my_login_ip_last
    find_update_data.clone().set(my_login_ip_last.eq(account_login_ip_in_database))
        .execute(connection)
        .unwrap();

    // update my_device_id_now
    find_update_data.clone().set(my_device_id_now.eq(device_id_new.to_string()))
        .execute(connection)
        .unwrap();

    // update my_login_ip_now
    find_update_data.clone().set(my_login_ip_now.eq(ip_new))
        .execute(connection)
        .unwrap();

    //login time now
    let time_now = Local::now().to_string();
    println!("login time: {}",time_now);

    // update my_login_ip_now
    find_update_data.clone().set(my_login_time_now.eq(time_now))
        .execute(connection)
        .unwrap();
    }else{
        back_code = 1512;
    }

} else {
    back_code = 1511;
}

    let mut map_data= serde_json::Map::new();
    map_data.insert("my_uuid".to_string(), serde_json::Value::String(m_uuid));

    let json = models_http::JsonData{
        code: back_code,
        data: map_data,
    };
    

    Ok(json)
}


pub fn check_account_login_device(
    connection: &MysqlConnection,
    map_data: &serde_json::Map<String, serde_json::Value>
) -> Result<i32, diesel::result::Error>{

    use crate::schema::accounts::dsl::*;

    let back_code;

    // get update account
    let update_phone = map_data.get("my_phone").unwrap().as_str().unwrap();

    // get update account uuid
    let update_uuid = map_data.get("my_uuid").unwrap().as_str().unwrap();

    // get update account device_id
    let device_id_new = map_data.get("my_device_id_now").unwrap().as_str().unwrap();

    println!("Start to check uuid: {}. Phone: {}", &update_uuid.to_string(), &update_phone.to_string());

    let account_get_by_uuid: models_account::Accounts = accounts
        .filter(my_uuid.eq(update_uuid.to_string()))
        .first::<models_account::Accounts>(connection)
        .unwrap();

    // use now data pass to last data
    let device_id_in_database = account_get_by_uuid.my_device_id_now;

    // use now data pass to last data
    let phone_in_database = account_get_by_uuid.my_phone;

    //phone same
    if update_phone.eq(phone_in_database.as_str()){
        //device same        
        if device_id_new.eq(device_id_in_database.as_str()){
            back_code = 1601;

            // device different
        }else{
            back_code = 1602;
            logout_account(connection,map_data).unwrap();
        }
        
    }else{
        // account different
        back_code = 1603;
        logout_account(connection,map_data).unwrap();

    }

    Ok(back_code)
}


pub fn check_account_exit(
    connection: &MysqlConnection,
    phone_n: &String
) -> Result<usize, diesel::result::Error>{

    use crate::schema::accounts::dsl::*;

    // get check account phone
    let check_my_phone = phone_n.clone();

    let account_get_by_phone: Vec<models_account::Accounts> = accounts
        .filter(my_phone.eq(check_my_phone))
        .load::<models_account::Accounts>(connection)
        .unwrap();

    // exit_accounts_amount
    let exit_accounts_amount =account_get_by_phone.len();

    Ok(exit_accounts_amount)
}


pub fn logout_account(
    connection: &MysqlConnection,
    map_data: &serde_json::Map<String, serde_json::Value>
) -> Result<i32, diesel::result::Error>{

    use crate::schema::accounts::dsl::*;

    let back_code;

    // get update account uuid
    let update_uuid = map_data.get("my_uuid").unwrap().as_str().unwrap();

    //let test_answer_all_model_list:Vec<String> = map_data.get("my_test_answer_all_model_list").unwrap().as_array().unwrap().into_iter().map(|i|i.as_str().unwrap().to_string()).collect();

    //print!("trst: {:?}",test_answer_all_model_list);

    // find update
    let find_update_data = diesel::update(accounts.filter(my_uuid.like(update_uuid.to_string())));

    // update account state
    find_update_data.set(my_state.eq("OFF".to_string()))
        .execute(connection)
        .unwrap();

    println!("logout account : {}",update_uuid.to_string());

    back_code = 1701;

    Ok(back_code)
}
