use diesel::{Connection, SqliteConnection};


///Init connection to DB
pub fn establish_connection() -> SqliteConnection {

    let database_url = "manager.db";

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}