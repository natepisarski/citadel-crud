extern crate citadel_crud;
extern crate rusqlite;

use citadel_crud::system::DatabaseConnection;
use citadel_crud::components::deleter;
use citadel_crud::connections::sqlite_connection;

use self::rusqlite::Connection;

use std::rc::Rc;

use common;
use common::schema::*;

pub struct EmployeeDeleter {
    pub employee_id: u32
}

impl deleter::Deleter<sqlite_connection::SqliteConnection> for EmployeeDeleter {
    fn delete(&self, connection: &sqlite_connection::SqliteConnection) -> bool {

        let connection_copy = connection.raw_connection();

        let inner_connection_reference: Rc<Connection> = connection_copy.unwrap();

        let usable_connection = inner_connection_reference.as_ref();

        usable_connection.execute(
            "DELETE FROM Employee WHERE EmployeeId = ?1", &[&self.employee_id]
        );

        return true;
    }
}

#[test]
pub fn test() {
    println!("TEST HIT")
}