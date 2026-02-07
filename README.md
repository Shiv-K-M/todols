# todols (Todo List) v0.1

## Introduction 
todols is a simple CLI tool to create and manage your Todo List.

## Requirement 
- Linux Operating System
- Cargo build System [[link](https://doc.rust-lang.org/cargo/getting-started/installation.html)]

## Installation
Clone repository
```bash
git clone https://github.com/Shiv-K-M/{my repo}
```
Go to inside cloned repository and run:
```bash
cargo install --path .
```

## Usage
```bash
todols --help
```
```bash
Simple program to create and manage todo list

Usage: todols [OPTIONS]

Options:
  -a, --add              create a new task
  -u, --update <UPDATE>  update existing task [takes Sl.No and value you want to update: -N, -D or -T]
  -d, --delete <DELETE>  delete a task [takes one or more Sl.No]
  -s, --sort <SORT>      sort the created tasks [possible values: task, duedate, status]
  -r, --reverse          reverse your args [used for: --sort and --filter]
  -f, --filter           filter created tasks [takes: -N, -D or -T]
  -N <NAME>...           name of the task
  -D <DATETIME>...       task due date [format: "%d-%m-%Y %H:%M:%S"]
  -T <TASKSTATUS>...     task status [If not provided default value "todo" is considered] [possible values: todo, in-progress, completed]
  -h, --help             Print help
  -V, --version          Print version
```


## Sample
[image]()

## Contributing

## Todo
* The filter function should be able to take mulitple child flags and each child flag should be able to handle multiple input (as a vector) for example todols -rfT todo in-progress means all the task that does not contain todo and in-progress must be shown.

## License
This project is licensed under the MIT License - see the [LICENSE][License file] file for details.


[License file]: https://github.com/Shiv-K-M/todols/blob/main/LICENSE 


