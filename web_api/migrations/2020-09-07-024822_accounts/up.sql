-- Your SQL goes here
CREATE TABLE accounts (
    id BIGINT PRIMARY KEY auto_increment ,
    my_uuid VARCHAR(256) NOT NULL ,
    my_password VARCHAR(256) NOT NULL ,
    my_state VARCHAR(256) NOT NULL ,
    my_name VARCHAR(256) NOT NULL ,
    my_phone VARCHAR(256) NOT NULL ,
    my_email VARCHAR(256) NULL ,
    my_photo VARCHAR(256) NULL ,
    my_login_type VARCHAR(256) NOT NULL ,
    
    my_device_id_now VARCHAR(256) NOT NULL ,
    my_device_id_last VARCHAR(256) NOT NULL ,
    
    my_login_time_now VARCHAR(256) NOT NULL ,
    my_login_time_last VARCHAR(256) NOT NULL ,

    my_login_ip_now VARCHAR(256) NOT NULL ,
    my_login_ip_last VARCHAR(256) NOT NULL ,
    
    my_register_time VARCHAR(256) NOT NULL
);

CREATE TABLE validations (
    id BIGINT PRIMARY KEY auto_increment ,
    phone_number VARCHAR(256) NOT NULL ,
    phone_code VARCHAR(256) NOT NULL ,
    phone_code_get_time VARCHAR(256) NOT NULL
);