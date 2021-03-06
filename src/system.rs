use std::rc::Rc;
use rusqlite::Connection;

/// A database connection is the basic type for all database connections.
/// It requires only that the implementor provides access to
pub trait DatabaseConnection {
    fn establish_connection(&mut self);
    fn end_connection(&mut self) -> ();
    fn raw_connection(&self) -> Option<Rc<Connection>>;
}