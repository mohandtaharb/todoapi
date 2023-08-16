use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::task)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub accomplished: bool,   
}


#[derive(Insertable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::task)]
pub struct NewTask<'a> {
    pub name: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseMessage {
    pub msg: String,
}