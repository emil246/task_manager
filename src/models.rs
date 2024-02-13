use diesel::prelude::*;
use chrono::NaiveDate;
use crate::schema::tasks;


#[derive(Queryable, Selectable)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub expire: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub name: String,
    pub description: String,
    pub expire: NaiveDate,
}

#[derive(Debug)]
pub struct FilterTasks {
    pub completed: Option<bool>
}