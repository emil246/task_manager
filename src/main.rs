use chrono::NaiveDate;
use clap::Parser;
use args::Cli;
use crate::args::ActionType;
use task_manager::models::{NewTask, FilterTasks};
use args::RemoveCommandType;
mod db;
mod models;
mod schema;
mod helpers;
mod args;








fn main() {
    let mut connection = db::establish_connection();
    db::run_migrations(& mut connection).unwrap();
    let matches = Cli::parse();
    match matches.action {
        ActionType::List(list_command) => {
            let filter = FilterTasks {
                completed: helpers::get_kind(&list_command.kind)
            };

            let tasks = helpers::get_tasks(&mut connection, filter);

            let _ = helpers::print_table_result(tasks);
        },
        ActionType::Remove(remove_command) => match remove_command.kind {
            RemoveCommandType::All => {
                let result = helpers::delete_completed(&mut connection);

                let _ = helpers::print_result(result);
            }
            RemoveCommandType::Id => {
                let result = helpers::delete(&mut connection, remove_command.id);

                let _ = helpers::print_result(result);
            }
        },
        ActionType::Complete(complete_command) => {
            let result = helpers::complete(&mut connection, complete_command.id);

            let _ = helpers::print_result(result);
        },
        ActionType::Add(add_command) => {
            let date = NaiveDate::parse_from_str(&add_command.date_str, "%d.%m.%Y").expect("Wrong date format! Use %d.%m.%Y");

            let task = NewTask {
                name: add_command.name,
                description: add_command.description,
                expire: date,
            };

            let result = helpers::create_task(&mut connection, task);

            let _ = helpers::print_result(result);
        }
    }

    
    }

