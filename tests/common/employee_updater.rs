extern crate citadel_crud;
extern crate rusqlite;

use citadel_crud::system::DatabaseConnection;
use citadel_crud::components::updater;
use citadel_crud::connections::sqlite_connection;

use self::rusqlite::Connection;

use std::rc::Rc;

use common;
use common::schema::*;

pub struct EmployeeUpdater<'a> {
    employee_id: u32,
    replacement_employee: &'a Employee
}

impl <'a> updater::Updater<sqlite_connection::SqliteConnection> for EmployeeUpdater<'a> {
    fn update(&self, connection: &sqlite_connection::SqliteConnection) -> bool {

        let connection_copy = connection.raw_connection();

        let inner_connection_reference: Rc<Connection> = connection_copy.unwrap();

        let usable_connection = inner_connection_reference.as_ref();

        let employee: &Employee = self.replacement_employee;

        usable_connection.execute(
            "UPDATE Employee\
            SET FirstName='?1', LastName='?2', DepartmentId=?3\
            WHERE EmployeeId = ?4",
            &[&employee.first_name,
                &employee.last_name,
                &employee.department_id,
                &self.employee_id]).unwrap();
        return true;
    }
}

#[test]
pub fn test() {
    println!("TEST HIT")
}