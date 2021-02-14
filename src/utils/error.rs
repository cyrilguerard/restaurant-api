use rocket_contrib::databases::rusqlite::Error as RusqliteError;

pub enum Error {
    InvalidArgumentError(String),
    DatabaseAccessError(RusqliteError),
}
