use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::dogs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Dog {
    pub id: i32,
    pub name: String,
    pub image_path: String    
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::dogs)]
pub struct NewDog {
    pub name: String,
    pub image_path: String,
}