extern crate citadel_crud;

mod common;

use common::*;
use common::schema::*;

use citadel_crud::components::creator::Creator;
use citadel_crud::components::reader::Reader;
use citadel_crud::components::updater::Updater;
use citadel_crud::components::deleter::Deleter;

use citadel_crud::connections::sqlite_connection;

use citadel_crud::system::DatabaseConnection;

#[test]
/// This function acts as the entirety of citadel-crud's integration tests. It bootstraps an
/// empty diesel sqlite database, and performs CRUD operations on it. It should be left in the state
/// it started in by the end of the test.
pub fn integration_test() {

    // Step 1: Bootstrap a connection to the database
    let mut db_connection = sqlite_connection::SqliteConnection::new(&"tests/testdb.sqlite");
    db_connection.establish_connection();

    // Step 2: Create a new Employee, named "Fred Hicks"
    let creator = employee_creator::EmployeeCreator { };
    let employee_object = schema::Employee { first_name: "Fred".to_string(), last_name: "Hicks".to_string(), id: 0, department_id: 0};
    assert!(creator.insert(&db_connection, employee_object));

    // Step 3: See if we can read the employee "Fred Hicks", but not "Tom Hicks"
    let fred_reader = employee_reader::EmployeeReader { first_name: "Fred".to_string()};
    let tom_reader = employee_reader::EmployeeReader { first_name: "Tom".to_string()};
    let employee_list: Vec<Employee> = fred_reader.read(&db_connection);
    let fred = &employee_list[0];

    assert_eq!(1, employee_list.len());
    assert_eq!(fred.last_name, "Hicks".to_string());
    // TODO: Make it not crash for non-found rows

    // Step 4: Change "Fred Hicks" last name to "Fred Hickman"
    let fred_updater = employee_updater::EmployeeUpdater { employee_id: 0,
        replacement_employee: &Employee { department_id: 0, first_name: "Fred".to_string(), last_name: "Hickman".to_string(), id: 0}};
    fred_updater.update(&db_connection);

    // Step 4.5: Re-use reader to validate name change
    let new_employee_list = &fred_reader.read(&db_connection);
    assert!(new_employee_list.len() > 0);
    assert_eq!("Hickman".to_string(), new_employee_list[0].last_name);

    // Step 5: Delete the employee "Fred Hickman"
    let fred_deleter = employee_deleter::EmployeeDeleter { employee_id: 0};

    // Step 5.5: Validate that we cannot read the employee "Fred Hickman"
    // TODO: Make it not crash for non-found rows
}