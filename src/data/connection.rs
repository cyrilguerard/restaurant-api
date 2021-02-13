use rocket_contrib::database;
use rocket_contrib::databases::rusqlite;

#[database("sqlite_db")]
pub struct SqliteConnection(rusqlite::Connection);
