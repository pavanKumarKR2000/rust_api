use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub details: JsonValue,
    pub name: String,
}