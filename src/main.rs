use std::{
    collections::HashMap,
    fs::{read_to_string, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use clap::{arg, command, value_parser, Command};

fn main() {
    let config_file_path = home::home_dir()
        .expect("Failed to find home directory")
        .join(".clover");
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .subcommand(
            Command::new("addclass")
                .about("Adds a new class to the overview")
                .arg(arg!([name] "The class name")),
        )
        .subcommand(
            Command::new("addtask")
                .about("Adds a new task to the overview")
                .arg(arg!([name] "The task name")),
        )
        .subcommand(Command::new("show").about("Shows the class overview"))
        .subcommand(
            Command::new("complete")
                .about("Completes the task for the given class")
                .arg(
                    arg!(
                        -c --class <NAME> "The class to complete the task for"
                    )
                    .required(true)
                    .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(
                        -t --task <TASK> "The task to complete"
                    )
                    .required(true)
                    .value_parser(value_parser!(String)),
                ),
        )
        .get_matches();

    let config = if let Some(config_file_path) = matches.get_one::<PathBuf>("config") {
        println!("Loaded custom config file {}", config_file_path.display());
        read_to_string(config_file_path).expect(&format!(
            "Invalid config file path {}",
            config_file_path.display()
        ))
    } else {
        if !config_file_path.exists() {
            File::create(&config_file_path).expect(
                "Failed to create config file, does the directory exist or is it read-only?",
            );
        }
        read_to_string(&config_file_path)
            .expect("Failed to read config file, is the directory read-only?")
    };

    let mut overview = if config.is_empty() {
        HashMap::new()
    } else {
        if let Ok(overview) =
            serde_json::from_str::<HashMap<String, HashMap<String, bool>>>(&config)
        {
            overview
        } else {
            println!("Failed to parse config file!");
            HashMap::new()
        }
    };

    if let Some(matches) = matches.subcommand_matches("addclass") {
        if let Some(name) = matches.get_one::<String>("name") {
            println!("Added class {name}");
            overview.insert(name.to_owned(), HashMap::new());
            save_current_configuration(&overview, &config_file_path);
        }
    }

    if let Some(matches) = matches.subcommand_matches("addtask") {
        if let Some(name) = matches.get_one::<String>("name") {
            println!("Added task {name}");
            for (_, tasks) in &mut overview {
                tasks.insert(name.to_owned(), false);
            }
            save_current_configuration(&overview, &config_file_path);
        }
    }

    if let Some(matches) = matches.subcommand_matches("complete") {
        if let Some(class) = matches.get_one::<String>("class") {
            if let Some(task) = matches.get_one::<String>("task") {
                println!("Completing {task} for {class}");
                if let Some(tasks) = overview.get_mut(class) {
                    if tasks.contains_key(task) {
                        tasks.insert(task.to_owned(), true);
                        save_current_configuration(&overview, &config_file_path);
                    } else {
                        println!(
                            "Invalid task supplied! Current tasks: {:?}",
                            overview[class].keys()
                        );
                    }
                } else {
                    println!(
                        "Invalid class supplied! Current classes: {:?}",
                        overview.keys()
                    );
                }
            } else {
                println!("No task supplied, the task is necessary to flag its completion!");
            }
        } else {
            println!("No class supplied, the class is necessary to flag the task's completion!");
        }
    }

    if let Some(_) = matches.subcommand_matches("show") {
        println!("{overview:?}");
        return;
    }
}

fn save_current_configuration(
    overview: &HashMap<String, HashMap<String, bool>>,
    config_file_path: &Path,
) {
    if let Ok(mut config_file) = OpenOptions::new()
        .read(true)
        .write(true)
        .open(config_file_path)
    {
        config_file
            .write(
                serde_json::to_string(&overview)
                    .expect("Failed to parse current configuration!")
                    .as_bytes(),
            )
            .expect("Failed to write to config file!");
    } else {
        println!("Failed to open config file and save current configuration!")
    }
}
