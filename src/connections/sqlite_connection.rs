use ::system::DatabaseConnection;
use rusqlite::Connection;
use std::rc::Rc;

pub struct SqliteConnection {
    pub connection_file_path: String,
    connection: Option<Rc<Connection>>
}

impl SqliteConnection {
    pub fn new(database_connection_string: &str) -> SqliteConnection {
        SqliteConnection {
            connection_file_path: database_connection_string.to_string(),
            connection: None
        }
    }
}

impl DatabaseConnection for SqliteConnection {
    fn establish_connection(&mut self) -> () {
        self.connection =
            Some(Rc::new(Connection::open(&self.connection_file_path)
                .expect(
                    &format!("[SqliteConnection] Cannot connect to file: {}",
                             &self.connection_file_path)
                )))
    }

    fn end_connection(&mut self) -> () {
        // [TODO|]
    }

    fn raw_connection(&self) -> Option<Rc<Connection>> {
        self.connection.clone()
    }
}