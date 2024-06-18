use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::dogs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]

pub struct Dog {
    pub id: i32,
    pub name: String,
    pub image_path: String    
}