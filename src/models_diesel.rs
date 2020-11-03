use crate::schema::bbs;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct Bbs {
    pub id: i64,
    pub name: String,
    pub path_name: String,
    pub description: String,
    pub category: String,
    pub hidden: bool,
}

#[derive(Insertable)]
#[table_name = "bbs"]
pub struct NewBbs<'a> {
    pub name: &'a str,
    pub path_name: &'a str,
    pub description: &'a str,
    pub category: &'a str,
}
