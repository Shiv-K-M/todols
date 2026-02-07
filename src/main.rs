use todols::{TaskItem,};
use comfy_table::Table;
use std::error::Error;
use clap::{Parser};
use whoami;

fn main()-> Result<(),Box<dyn Error>> {
    // create a new directory at ~/.local/share/ to store the tasks 
    let uname = whoami::username()?;
    let dir = format!("/home/{}/.local/share/todols",uname);
    std::fs::create_dir_all(&dir)?;
    
    let filepath = format!("{}/todols_saved.json",dir);
    let read_result = todols::load_task_data(std::path::Path::new(&filepath));

    let mut list:Vec<TaskItem> = match read_result {
        Ok(result_list) => {
                //println!("File read success");
                result_list
        },
        Err(e)  => {
                println!("Error reading file: Maybe\n 1.No task has been saved yet.\n 2.Saved file 
                    has been modified/moved.\n Try creating new task \nAnyway here the error statement if you want:{e:#?}");
                vec![]
        },
    };

    // get arguments from the terminal
    let args = todols::Args::parse();
    let mut table = Table::new();

    todols::display_init(&mut table);

    if args.filter {
        args.handle_filter(&mut list, &mut table)?;
    } 
    else {
        args.handle_input(&mut list, &mut table)?;
    }

    println!("{table}");

    todols::save_task_data(std::path::Path::new(&filepath),&list)?;

    Ok(())
}
