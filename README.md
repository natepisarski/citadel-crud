# Citadel-Crud
Super-duper simple wrapper / dependency inversion layer for Rust ORM's like Diesel.

Mostly useless for real-world projects today, but it made some Rust code slightly simpler in very early Rust versions.

# Original README
Here's a very small rundown of this rust library:

* Abstract database connections to form DBMS-agnostic pools

`impl DatabaseConnection for MyConnection`

* Create in-memory objects for CRUD operations

`impl Updater for Calendar`

* Compose CRUD components to build larger ones

`let dashboard: Component = calendar.bind(messages).bind(users)`
