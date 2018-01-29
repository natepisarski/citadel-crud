extern crate citadel_crud;
extern crate rusqlite;

mod common;

use citadel_crud::system::DatabaseConnection;
use citadel_crud::components::reader;
use citadel_crud::connections::sqlite_connection;

use rusqlite::Connection;
use rusqlite::*;

use common::schema::Employee;

pub struct EmployeeReader {
    pub first_name: String
}

impl reader::Reader<sqlite_connection::SqliteConnection, std::vec::Vec<common::schema::Employee>> for EmployeeReader {
    fn read(&self, connection: &sqlite_connection::SqliteConnection) -> std::vec::Vec<common::schema::Employee> {
        let connection_copy = connection.raw_connection();
        let usable_connection: &Connection = connection_copy.as_ref();
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

#[test]
pub fn test(){

}