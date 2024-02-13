use chrono::{NaiveDate,Utc};
use task_manager::models::*;
use diesel::prelude::*;
use crate::schema::tasks;
use crate::args::ListCommandType;
use std::error::Error;
use std::io::{self, Write};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub fn create_task(conn: &mut SqliteConnection, new_task: NewTask) -> Result<(), Box<dyn Error>> {
    use task_manager::schema::tasks;
    match diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}

pub fn get_tasks(conn: &mut SqliteConnection, filter: FilterTasks) -> QueryResult<Vec<Task>>{
    let mut query = tasks::table.into_boxed();

        if let Some(completed) = filter.completed {
            query = query.filter(tasks::completed.eq(completed));
        }
        query.load::<Task>(conn)
}

pub fn get_kind(kind: &ListCommandType) -> Option<bool> {
    match kind {
        ListCommandType::Completed => Some(true),
        ListCommandType::Uncompleted => Some(false),
        ListCommandType::All => None,
    }
}

pub fn delete_completed(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    use task_manager::schema::tasks;
    let mut query = tasks::table.into_boxed();
    query = query.filter(tasks::completed.eq(true));
    let filtered_tasks = query.load::<Task>(conn);

    for task in filtered_tasks.unwrap() {
        let _ = diesel::delete(tasks::table.find(task.id)).execute(conn);
    }

    Ok(())
}

pub fn print_result<T, E>(result: Result<T, E>) -> Result<(), io::Error> {
    match result {
        Ok(_) => writeln!(io::stdout(), "{}", "Success!"),
        Err(_) => writeln!(io::stdout(), "{}", "Something wrong!"),
    }
}

pub fn delete(conn: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
    use task_manager::schema::tasks;
    diesel::delete(tasks::table.find(id)).execute(conn)
}

pub fn complete(conn: &mut SqliteConnection, id: i32) -> Result<(), Box<dyn Error>> {
    use task_manager::schema::tasks;
    match diesel::update(tasks::table.find(id))
        .set(tasks::completed.eq(true))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}


pub fn check_date(date:NaiveDate) -> bool{
    let now = Utc::now().date_naive();
    if (date - now).num_days() < 0 {
        true
    } else {false}
}

pub fn print_table_result(
    tasks: Result<Vec<Task>, diesel::result::Error>,
) -> Result<(), io::Error> {
    let mut table = comfy_table::Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(100)
        .set_header(vec!["ID","Name", "Description", "Completed", "Expire"]);
    for task in tasks.as_ref().unwrap(){
        let mut row = Row::new();
            row.add_cell(Cell::new(&task.id).set_alignment(CellAlignment::Center));
            row.add_cell(Cell::new(&task.name));
            row.add_cell(Cell::new(&task.description));
            if task.completed == true {
                row.add_cell(Cell::new("✅"))} else {
                    row.add_cell(Cell::new("❌"))
            };
            if check_date(task.expire) == true {
                row.add_cell(Cell::new(&task.expire).fg(Color::Red))
            } else {
                row.add_cell(Cell::new(&task.expire))
            };
            table.add_row(row);
        };
    println!("{table}");
    Ok(())
}