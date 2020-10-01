use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

#[derive( Serialize, Deserialize)]
pub struct JsonData{
    pub code: i32,
    pub data: Map<String,Value>,
}
