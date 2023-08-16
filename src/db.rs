use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failed to connect to the database"))
}
