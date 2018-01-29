use ::system::DatabaseConnection;
use rusqlite::Connection;
use std::rc::Rc;

pub struct SqliteConnection {
    pub connection_file_path: String,
    connection: Rc<Connection>
}

impl SqliteConnection {
    pub fn new(database_connection_string: &str) -> Connection {
        Connection::open(database_connection_string).unwrap()
    }
}

impl DatabaseConnection for SqliteConnection {
    fn establish_connection(&mut self) -> () {
        self.connection =
            Rc::new(Connection::open(&self.connection_file_path)
                .expect(
                    &format!("[SqliteConnection] Cannot connect to file: {}",
                             &self.connection_file_path)
                ))
    }

    fn end_connection(&mut self) -> () {
        // [TODO|]
    }

    fn raw_connection(&self) -> Rc<Connection> {
        self.connection.clone()
    }
}