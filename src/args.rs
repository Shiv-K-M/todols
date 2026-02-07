use clap::{Parser,ValueEnum};
use crate::{TodoState,TaskItem};
use std::{error::Error,};
use chrono::{DateTime,Local,NaiveDateTime,TimeZone};
use regex::Regex;
use comfy_table::Table;
use lexicmp::natural_cmp;

#[derive(Debug, Clone, ValueEnum)]
pub enum SortBy {
    Task,
    Duedate,
    Status,
}

/// Simple program to create and manage todo list
#[derive(Parser)]
#[command(
    version, 
    about, 
    long_about = None,
    group(clap::ArgGroup::new("options").multiple(true)),
    )]

pub struct Args {
    /// create a new task    
    #[arg(short, long, requires = "name", requires = "datetime")]
    pub add: bool,

    /// update existing task [takes Sl.No and value you want to update: -N, -D or -T]
    #[arg(short, long, requires = "options")]
    pub update: Option<usize>,

    /// delete a task [takes one or more Sl.No]
    #[arg(short, long)]
    pub delete: Option<Vec<usize>>,

    /// sort the created tasks 
    #[arg(short, long )]
    pub sort: Option<SortBy>,

    /// reverse your args [used for: --sort and --filter]
    #[arg(short, long)]
    pub reverse: bool,

    /// filter created tasks [takes: -N, -D or -T]
    #[arg(short, long, requires = "options")]
    pub filter: bool,

    /// name of the task 
    #[arg(short = 'N', group = "options", num_args = 1..)]
    pub name: Option<Vec<String>>,

    /// task due date [format: "%d-%m-%Y %H:%M:%S"]
    //not done yet -> Note: If time is not provided it is considered as end of the day.
    #[arg(short = 'D', group = "options", num_args = 1..)]
    pub datetime: Option<Vec<String>>,

    /// task status [If not provided default value "todo" is considered]
    #[arg(short = 'T', group = "options", num_args= 1..)]
    pub taskstatus:  Option<Vec<TodoState>>, 
}

// check that value provided for --update and --delete args is more than zero
fn min_index_val(val: usize) -> Result<usize,String> {
    if val < 1 {
        Err(format!("Value must be atleast 1 received {val}"))
    }
    else 
    {
        Ok(val)
    }
}

// convert string into datetime datatype
fn string_to_datetime(value:&str)-> Result<DateTime<Local>,Box<dyn Error>> 
{
    let format = "%d-%m-%Y %H:%M:%S";
    let dt = NaiveDateTime::parse_from_str(value,format)?;
    Ok(Local.from_local_datetime(&dt).latest().ok_or("Invalid Time at {i}th task")?) 
}

impl Args {

    // handle all args execpt filter
    pub fn handle_input(mut self, list:&mut Vec<TaskItem>, table:&mut Table) -> Result<(),Box<dyn Error>> {
         if self.add
         {
            self.handle_add(list)?;
         }

         if self.update.is_some()
         {
             self.handle_update(list)?;
         }

         if self.delete.is_some() 
         {
             self.handle_delete(list)?;
         }

         if self.sort.is_some() 
         {
             self.handle_sort(list)?;
         }

         // display the todo list
         for i in 0..list.len() 
         {
            crate::display_task(&(i+1).to_string(), &list[i], table);
         }

        Ok(())
    }
   
    // filter the tasks according to user input 
    pub fn handle_filter(&self, list:&Vec<TaskItem>, table:&mut Table) -> Result<(),Box<dyn Error>> {
        if let Some(todov) = &self.name {
            let re = Regex::new(&todov[0])?;

            let matchs:Vec<(usize,&TaskItem)> = list
                .iter()
                .enumerate()
                .filter(|(_,s)| 
                    if self.reverse {
                        !re.is_match(&s.task)
                    }
                    else {
                        re.is_match(&s.task)
                    })
                .collect();
           
            for (index,item) in matchs {
                crate::display_task(&(index+1).to_string(),item,table);
            }
            return Ok(());
        }

        if let Some(datetimev) = &self.datetime {

            let re = Regex::new(&datetimev[0])?;
            let format = "%d-%m-%Y %H:%M:%S";

            let matchs:Vec<(usize,&TaskItem)> = list
                .iter()
                .enumerate()
                .filter(|(_,s)| 
                    if self.reverse {
                        !re.is_match(&s.dueDateTime.format(&format).to_string())
                    }
                    else {
                        re.is_match(&s.dueDateTime.format(&format).to_string())
                    })
                .collect();

            for (index,item) in matchs {
                crate::display_task(&(index+1).to_string(), item, table);
            }
            return Ok(())
        }

        if let Some(taskstatusv) = &self.taskstatus {
            let e = taskstatusv[0];
            
            let matchs:Vec<(usize,&TaskItem)> = list
                .iter()
                .enumerate()
                .filter(|(_,s)| 
                    if self.reverse {
                        s.status != e
                    }
                    else {
                        s.status == e
                    })
                .collect();

            for (index,item) in matchs {
                crate::display_task(&(index+1).to_string(), item, table);
            }
            return Ok(());
        }
        
        Err(Box::new(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unalbe to process the provided input",
            )
        ))
        
    }

    fn handle_add(&self, list:&mut Vec<TaskItem>) -> Result<(),Box<dyn Error>>{
        
        let todov: &[String] = self.name.as_deref().ok_or("Unable to parse value provided for -N")?;

        let datetimev: &[String] = self.datetime.as_deref().unwrap_or_else(|| {
            println!("No value provided for flag -D");
            &[] 
        });
    
        let taskstatusv: &[TodoState] = self.taskstatus.as_deref().unwrap_or_else(|| {
            &[]     
        });
       
        for i in 0..todov.len() {
            let dt = string_to_datetime(&datetimev[i])?;
            let mut todostate = TodoState::Todo;
            if i<taskstatusv.len() {
                todostate = taskstatusv[i];
            }
    
            let newtask = TaskItem::new(&todov[i],dt,todostate);
            list.push(newtask);
        }
    
        Ok(())
    }

    fn handle_update(&self, list:&mut Vec<TaskItem>) -> Result<(),Box<dyn Error>> {
        let index = min_index_val(self.update.ok_or("Unable to parse value of --update")?)? - 1;
        
        if index < list.len() {
            if let Some(value) = &self.name 
            {
                list[index].update_task(&value[0]);
            }

            if let Some(value) = &self.datetime
            {
                let dt = string_to_datetime(&value[0])?;
                list[index].update_duedate(dt);
            }

            if let Some(value) = &self.taskstatus
            {
                list[index].update_status(value[0]);
            }

            return Ok(());
        }
            
        Err(Box::new(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Provided Sl.No. of a task for --update is out of bound",
            )
        ))

    }

    fn handle_delete(&mut self, list:&mut Vec<TaskItem>) -> Result<(),Box<dyn Error>> {
        let delete_list = self.delete.as_mut().ok_or("Unable to parse value of --delete, requires a number")?;
        delete_list.sort();
        delete_list.reverse();
        for index_item in delete_list {
            let index = min_index_val(*index_item)? - 1;
            
            if index < list.len() {
                list.remove(index);
            }
            else {
                return Err(Box::new(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Provided Sl.No. of a task for --delete is out of bound",
                    )
                ));
            }
        }

        Ok(())
    }

    fn handle_sort(&self, list:&mut Vec<TaskItem>) -> Result<(),Box<dyn Error>> {
       match self.sort {
           Some(SortBy::Task) => {
               list.sort_by( |first, second| {
                   if self.reverse {
                       natural_cmp(&second.task.to_lowercase(),&first.task.to_lowercase())
                   }
                   else {
                       natural_cmp(&first.task.to_lowercase(),&second.task.to_lowercase())
                   }
               });

               return Ok(());
           },
           Some(SortBy::Duedate) => {
               list.sort_by( |first, second| {
                   if self.reverse {
                       second.dueDateTime.cmp(&first.dueDateTime)
                   }
                   else { 
                       first.dueDateTime.cmp(&second.dueDateTime)
                   }
               });

               return Ok(());
           },
           Some(SortBy::Status) => {
               list.sort_by( |first, second| {
                   if self.reverse {
                       second.status.cmp(&first.status)
                   }
                   else {
                       first.status.cmp(&second.status)
                   }
               });

               return Ok(());
           },
           None => ()
       };

       Err(Box::new(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unable to parse value provided for --sort",
            )
        ))

    } 
}


