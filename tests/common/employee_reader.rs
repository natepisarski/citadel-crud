extern crate citadel_crud;
extern crate rusqlite;

use citadel_crud::system::DatabaseConnection;
use citadel_crud::components::reader;
use citadel_crud::connections::sqlite_connection;

use citadel_crud::components::creator::Creator;
use citadel_crud::components::reader::Reader;
use citadel_crud::components::updater::Updater;
use citadel_crud::components::deleter::Deleter;

use self::rusqlite::Connection;
use self::rusqlite::*;

use std;
use common::schema::Employee;
use common;

use std::rc::Rc;

pub struct EmployeeReader {
    pub first_name: String
}

impl reader::Reader<sqlite_connection::SqliteConnection, std::vec::Vec<common::schema::Employee>> for EmployeeReader {
    fn read(&self, connection: &sqlite_connection::SqliteConnection) -> std::vec::Vec<common::schema::Employee> {

        let connection_copy = connection.raw_connection();
        let connection_reference: Rc<Connection> = connection_copy.unwrap();
        let usable_connection = connection_reference.as_ref();

        let mut statement = usable_connection.prepare("select * from Employee where FirstName = ?1 limit 1");
        let mut prepared_statement = statement.unwrap();

        let employee_results = prepared_statement.query_map(&[&self.first_name], |row| {Employee {
            first_name: row.get("FirstName"),
            last_name: row.get("LastName"),
            id: row.get("EmployeeId"),
            department_id: row.get("DepartmentId")
        }}).unwrap();
        let mut employees: Vec<Employee> = vec![];
        for employee in employee_results {
            employees.push(employee.unwrap());
        }
        return employees;
    }
}