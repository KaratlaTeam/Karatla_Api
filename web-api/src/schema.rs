table! {
    /// Representation of the `accounts` table.
    ///
    /// (Automatically generated by Diesel.)
    accounts (id) {
        /// The `id` column of the `accounts` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `my_uuid` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_uuid -> Varchar,
        /// The `my_password` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_password -> Varchar,
        /// The `my_state` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_state -> Varchar,
        /// The `my_name` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_name -> Varchar,
        /// The `my_phone` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_phone -> Varchar,
        /// The `my_email` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        my_email -> Nullable<Varchar>,
        /// The `my_photo` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        my_photo -> Nullable<Varchar>,
        /// The `my_login_type` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_login_type -> Varchar,
        /// The `my_device_id_now` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_device_id_now -> Varchar,
        /// The `my_device_id_last` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_device_id_last -> Varchar,
        /// The `my_login_time_now` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_login_time_now -> Varchar,
        /// The `my_login_time_last` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_login_time_last -> Varchar,
        /// The `my_login_ip_now` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_login_ip_now -> Varchar,
        /// The `my_login_ip_last` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_login_ip_last -> Varchar,
        /// The `my_register_time` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        my_register_time -> Varchar,
    }
}

table! {
    /// Representation of the `validations` table.
    ///
    /// (Automatically generated by Diesel.)
    validations (id) {
        /// The `id` column of the `validations` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `phone_number` column of the `validations` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        phone_number -> Varchar,
        /// The `phone_code` column of the `validations` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        phone_code -> Varchar,
        /// The `phone_code_get_time` column of the `validations` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        phone_code_get_time -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    validations,
);