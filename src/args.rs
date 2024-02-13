use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum Commands {
    Remove,
    List,
    Complete,
    Add
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum RemoveCommandType {
    Id,
    All,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum ListCommandType {
    All,
    Completed,
    Uncompleted,
}


#[derive(Parser, Debug)]
#[clap(author, about = "CLI task manager", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {

    /// Remove a task.
    Remove(RemoveCommand),

    /// List tasks.
    List(ListCommand),

    ///Complete task
    Complete(CompleteCommand),

    ///Add task
    Add(AddCommand)

}
#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// Remove Command
    #[clap(short = 't', long = "type")]
    pub kind: RemoveCommandType,

    /// The id of the task.
    #[clap(short = 'i', long = "id", default_value = "-1")]
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct ListCommand {
    /// The type of the task.
    #[clap(short = 'k', long = "kind", default_value = "all")]
    pub kind: ListCommandType,

}

#[derive(Debug, Args)]
pub struct CompleteCommand {
    /// The id of the task.
    #[clap(short = 'i', long = "id")]
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The content of the todo.
    #[clap(short = 'n', long = "name")]
    pub name: String,

    /// The content of the todo.
    #[clap(short = 'd', long = "description")]
    pub description: String,

    /// The date of the todo.
    #[clap(short = 'e', long = "expire", default_value = "01.01.1970")]
    pub date_str: String,
}
