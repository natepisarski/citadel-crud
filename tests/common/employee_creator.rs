extern crate citadel_crud;
extern crate rusqlite;

use citadel_crud::system::DatabaseConnection;
use citadel_crud::components::creator;
use citadel_crud::connections::sqlite_connection;

use self::rusqlite::Connection;

use std::rc::Rc;

use common;

pub struct EmployeeCreator {

}

impl creator::Creator<sqlite_connection::SqliteConnection, common::schema::Employee> for EmployeeCreator {
    fn insert(&self, connection: &sqlite_connection::SqliteConnection, employee: common::schema::Employee) -> bool {

        let connection_copy = connection.raw_connection();

        let inner_connection_reference: Rc<Connection> = connection_copy.unwrap();

        let usable_connection = inner_connection_reference.as_ref();

        usable_connection.execute("INSERT INTO Employee (EmployeeId, FirstName, LastName, DepartmentId)\
            VALUES (?1, ?2, ?3, ?4)", &[&employee.id, &employee.first_name, &employee.last_name, &employee.department_id]).unwrap();
        return true;
    }
}

#[test]
pub fn test() {
    println!("TEST HIT")
}