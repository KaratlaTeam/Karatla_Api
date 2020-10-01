use diesel::prelude::*;
use chrono::prelude::*;
use crate::models_validation;
use crate::schema;


pub fn insert_data_validation(
    connection: &MysqlConnection,
    validation_phone: &String,
    validation_code: &String
) -> Result<models_validation::NewValidation, diesel::result::Error> {

    //time now
    let time_now = Local::now().to_string();
    println!("insert_data_validation time: {}",time_now);

    //new_validation
    let new_validation = models_validation::NewValidation {
        phone_number: validation_phone.clone(),
        phone_code: validation_code.clone(),
        phone_code_get_time: time_now ,
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
    phone_n: &String
) -> Result<usize, diesel::result::Error> {
    use crate::schema::validations::dsl::*;

    let num_deleted = diesel::delete(schema::validations::table.filter(phone_number.like(phone_n)))
        .execute(connection)
        .expect("Error deleting validations");

    println!("Deleted {}, all: {}",phone_n ,num_deleted);

    Ok(num_deleted)
}

pub fn check_data_validation (
    connection: &MysqlConnection,
    code: &String,
    phone_n: &String
) -> Result<i32, diesel::result::Error> {
    use crate::schema::validations::dsl::*;

    let phone = phone_n.clone();
    let code = code.clone() ;

    let validation_get_by_code: Vec<models_validation::Validations>= validations
        .filter(phone_number.eq(phone))
        .load::<models_validation::Validations>(connection)
        .unwrap();
 
    let validation_get_by_code = validation_get_by_code.get(validation_get_by_code.len()-1).unwrap();

    println!("right code: {}",validation_get_by_code.phone_code);
    println!("customer code: {}",code);

    if validation_get_by_code.phone_code.eq(code.as_str()){
        println!("phone: {}, code right!",phone_n);
        Ok(1901)
    }else {
        println!("phone: {}, code wrong!",phone_n);
        Ok(1911)
    }
}