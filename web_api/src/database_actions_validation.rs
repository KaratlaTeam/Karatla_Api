use crate::models_validation;
use crate::schema;
use chrono::prelude::*;
use diesel::prelude::*;


pub fn get_validation_all(
    connection: &MysqlConnection,
) -> Result<Option<Vec<models_validation::Validations>>, diesel::result::Error>{
    use crate::schema::validations::dsl::*;

    let validation_get_all: Option<Vec<models_validation::Validations>> = validations
        .load::<models_validation::Validations>(connection)
        .optional()?;

    Ok(validation_get_all)
}

pub fn insert_data_validation(
    connection: &MysqlConnection,
    validation_phone: &String,
    validation_code: &String,
) -> Result<models_validation::NewValidation, diesel::result::Error> {
    //time now
    let time_now = Local::now().to_string();
    println!("insert_data_validation time: {}", time_now);

    // TODO: bug: checking safe_day
    //let day = Local::now().day(); 
    let hour = Local::now().hour();
    let minute = Local::now().minute();
    let second = Local::now().second();
    println!("time now: {}:{}:{}", hour, minute, second);

    let safe_second:u32 = second;
    let mut safe_minute:u32 = &minute + 1;
    let mut safe_hour:u32 = hour;
    //let mut safe_day:u32 = day;

    if minute == 59 
    {
        safe_minute = 0;
        if safe_hour == 23 
        {
            safe_hour = 0;
        }
        else 
        {
            safe_hour += 1;
        }
        
    }
    println!("time safe: {}:{}:{}", safe_hour, safe_minute, safe_second);

    //new_validation
    let new_validation = models_validation::NewValidation {
        phone_number: validation_phone.clone(),
        phone_code: validation_code.clone(),
        phone_safe_hour: safe_hour,
        phone_safe_minute: safe_minute,
        phone_safe_second: safe_second,
        phone_code_get_time: time_now,
    };

    // insert data
    diesel::insert_into(schema::validations::table)
        .values(&new_validation)
        .execute(connection)
        .expect("Error saving new_validation");

    Ok(new_validation)
}

pub fn delete_data_validation(
    connection: &MysqlConnection,
    phone_n: &String,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::validations::dsl::*;

    //let validation_get_by_code: Vec<models_validation::Validations> = validations
    //    .filter(phone_number.eq(phone_n))
    //    .load::<models_validation::Validations>(connection)
    //    .unwrap();
//
    //let validation_get_by_code = validation_get_by_code.get(0).unwrap();

    let num_deleted =
        diesel::delete(schema::validations::table.filter(phone_number.eq(phone_n)))
            .execute(connection)
            .expect("Error deleting validations");

    println!("Deleted {}, all: {}", phone_n, num_deleted);

    Ok(num_deleted)
}

pub fn check_data_validation(
    connection: &MysqlConnection,
    code: &String,
    phone_n: &String,
) -> Result<i32, diesel::result::Error> {
    use crate::schema::validations::dsl::*;

    let phone = phone_n.clone();
    let code = code.clone();
    let mut return_code:i32 = 1911;

    let validation_get_by_code: Vec<models_validation::Validations> = validations
        .filter(phone_number.eq(phone))
        .load::<models_validation::Validations>(connection)
        .unwrap();

    
    let validation_get_by_code = validation_get_by_code
        .get(validation_get_by_code.len() - 1)
        .unwrap();

    //time 
    // TODO: bug: checking safe_day
    //let day = Local::now().day(); 
    let hour = Local::now().hour();
    let minute = Local::now().minute();
    let second = Local::now().second();
    println!("time now: {}:{}:{}", hour, minute, second);

    let safe_second = validation_get_by_code.phone_safe_second;
    let safe_minute = validation_get_by_code.phone_safe_minute;
    let safe_hour = validation_get_by_code.phone_safe_hour;
    //let mut safe_day:u32 = day;
    println!("time safe: {}:{}:{}", safe_hour, safe_minute, safe_second);

    if safe_hour == hour 
    {
        if safe_minute - minute == 1
        {
            if safe_second <= second
            {
                // can check
                return_code = check_validation(&validation_get_by_code.phone_code, code, &phone_n);
            }
        }
        else if safe_minute == minute
        {
            if safe_second >= second
            {
                // can check
                return_code = check_validation(&validation_get_by_code.phone_code, code, &phone_n);
            }
        }
    }
    else if safe_hour - hour == 1 || safe_hour == 0
    {
        if safe_minute == 0
        {
            if safe_second >= second
            {
                // can check
                return_code = check_validation(&validation_get_by_code.phone_code, code, &phone_n);
            }
        }
    }

    Ok(return_code)
}

fn check_validation 
(
    correct_code: &String,
    customer_code: String,
    phone_n: &String,
)
-> i32
{
    println!("correct_code: {}", correct_code);
    println!("customer_code: {}", customer_code);

    if correct_code.eq(customer_code.as_str()) {
        println!("phone: {}, code right!", phone_n);
        1901
    } else {
        println!("phone: {}, code wrong!", phone_n);
        1911
    }
}
