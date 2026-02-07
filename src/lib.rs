use uuid::Uuid;
use chrono::{DateTime, Local,Duration};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::error::Error;
use clap::{ValueEnum};
use comfy_table::{
    presets,
    modifiers,
    Attribute,
    Table,
    ContentArrangement,
    CellAlignment,
    Cell,
    Color,
    Width,
    ColumnConstraint,
};

pub mod args;
pub use crate::args::Args;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, ValueEnum, Eq, PartialEq, Ord, PartialOrd)]
pub enum TodoState {
    Todo=0,
    InProgress,
    Completed,
}

impl TodoState {
    fn to_str(&self) -> &'static str {
        match &self {
            TodoState::Todo =>  "todo",
            TodoState::InProgress => "in-progress",
            TodoState::Completed => "completed"
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskItem {
    id: Uuid,
    task: String,
    dueDateTime: DateTime<Local>,
    status: TodoState, 
}

impl TaskItem {
    pub fn new(task:&str, datetime:DateTime<Local>, todostate:TodoState) -> Self {
        Self {
            id: Uuid::new_v4(),
            task: String::from(task),
            dueDateTime: datetime,
            status: todostate,
        }
    }

    pub fn update_task(&mut self, task:&str) {
        self.task = String::from(task);
    }

    pub fn update_duedate(&mut self, datetime:DateTime<Local>){
        self.dueDateTime = datetime;
    }

    pub fn update_status(&mut self, status: TodoState) {
        self.status = status;
    }

}

pub fn save_task_data(file: &std::path::Path, value: &Vec<TaskItem>) -> Result<(), Box<dyn Error>> {
    let file = File::options().write(true).create(true).truncate(true).open(file)?;
    //let file = std::fs::File::create(file)?;
    let file = BufWriter::new(file);
    serde_json::to_writer(file, value)?;
    Ok(())
}

pub fn load_task_data(file: &std::path::Path) -> Result<Vec<TaskItem>, Box<dyn Error>> {
    let file = File::options().read(true).open(file)?;
    let file = BufReader::new(file);

    let result_list:Vec<TaskItem> = serde_json::from_reader(file)?;

    Ok(result_list)
}

pub fn display_init(table:&mut Table){
    table
        .load_preset(presets::UTF8_FULL)
        .apply_modifier(modifiers::UTF8_SOLID_INNER_BORDERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Sl.No.").add_attribute(Attribute::Bold).set_alignment(CellAlignment::Center),
            Cell::new("Task").add_attribute(Attribute::Bold).set_alignment(CellAlignment::Center), 
            Cell::new("Duedate").add_attribute(Attribute::Bold).set_alignment(CellAlignment::Center), 
            Cell::new("Status").add_attribute(Attribute::Bold).set_alignment(CellAlignment::Center)
        ])
        .set_constraints(vec![
            ColumnConstraint::UpperBoundary(Width::Fixed(10)),
            ColumnConstraint::UpperBoundary(Width::Percentage(40))
        ]);
}

pub fn display_task(slno:&str, taskitem:&TaskItem, table:&mut Table) {
    let format = "%d-%m-%Y %H:%M:%S";
    let time_now = Local::now(); 

    table
        .add_row(vec![
            Cell::new(slno), 
            Cell::new(&taskitem.task), 
            Cell::new(&taskitem.dueDateTime.format(format).to_string()).fg(
                match taskitem.status {
                    TodoState::Todo | TodoState::InProgress => {
                        let three_h = Duration::hours(3);
                        if taskitem.dueDateTime > time_now {
                            if taskitem.dueDateTime - time_now > three_h {
                                Color::Green
                            }
                            else 
                            {
                                Color::Yellow
                            }
                        }
                        else 
                        {
                            Color::Red
                        }
                    },
                    _ => Color::White
                }),
            Cell::new(&taskitem.status.to_str()).fg(
                match taskitem.status {
                    TodoState::Todo => Color::Rgb{r:255,g:165,b:0},
                    TodoState::InProgress => Color::Yellow,
                    TodoState::Completed => Color::Green,
                })
        ]);    
}


