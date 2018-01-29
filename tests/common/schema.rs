pub struct Employee {
    pub id: u32,

    pub department_id: u32,

    pub first_name: String,

    pub last_name: String
}

pub struct Department {
    pub id: u32,

    pub name: String
}

pub struct Organization {
    pub employees: Vec<Employee>,
    pub departments: Vec<Department>
}