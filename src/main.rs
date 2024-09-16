use std::{
    cmp::max,
    fs::{read_to_string, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use clap::{arg, command, value_parser, Command};

#[derive(serde::Deserialize, serde::Serialize)]
struct ClassOverview<'a> {
    overview: Vec<Vec<Vec<bool>>>,
    #[serde(borrow)]
    class_names: Vec<&'a str>,
    #[serde(borrow)]
    task_names: Vec<&'a str>,
}

impl<'a> ClassOverview<'a> {
    fn new(configuration: &'a str) -> ClassOverview {
        if !configuration.is_empty() {
            if let Ok(overview) = serde_json::from_str::<ClassOverview>(&configuration) {
                return overview;
            } else {
                println!("Failed to parse config file!");
            }
        }
        return ClassOverview {
            overview: Vec::new(),
            class_names: Vec::new(),
            task_names: Vec::new(),
        };
    }
    fn generate_empty_class(task_count: &usize) -> Vec<bool> {
        vec![false; *task_count]
    }
    fn generate_empty_week(class_count: &usize, task_count: &usize) -> Vec<Vec<bool>> {
        vec![vec![false; *task_count]; *class_count]
    }
    fn push_class(&mut self, class_name: &'a str) {
        self.class_names.push(class_name);
        for classes in &mut self.overview {
            classes.push(Self::generate_empty_class(&self.task_names.len()))
        }
    }
    fn push_task(&mut self, task_name: &'a str) {
        self.task_names.push(task_name);
        for classes in &mut self.overview {
            for tasks in classes {
                tasks.push(false);
            }
        }
    }
    fn set_time_frame(&mut self, week_count: &usize) {
        if self.overview.len() > *week_count {
            self.overview.truncate(*week_count);
        } else {
            for _ in self.overview.len()..*week_count {
                self.overview.push(Self::generate_empty_week(
                    &self.class_names.len(),
                    &self.task_names.len(),
                ))
            }
        }
    }
    fn complete_task_for_class(
        &mut self,
        week: &usize,
        class_name: &str,
        task_name: &'a str,
    ) -> Result<(), &'static str> {
        if *week < self.overview.len() {
            if let Some(class_name_index) = self.class_names.iter().position(|&c| c == class_name) {
                if let Some(task_name_index) = self.task_names.iter().position(|&t| t == task_name)
                {
                    self.overview[*week][class_name_index][task_name_index] = true;
                    return Ok(());
                }
                return Err("Invalid task supplied!");
            }
            return Err("Invalid class supplied!");
        }
        return Err("Invalid week supplied!");
    }
    fn save_configuration(&self, configuration_file_path: &Path) {
        if let Ok(mut configuration_file) = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(configuration_file_path)
        {
            configuration_file
                .write(
                    serde_json::to_string(&self)
                        .expect("Failed to parse current configuration!")
                        .as_bytes(),
                )
                .expect("Failed to write to config file!");
        } else {
            println!("Failed to open config file and save current configuration!")
        }
    }
}

impl<'a> std::fmt::Display for ClassOverview<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_column_width = " week ".len() + self.overview.len().to_string().len() + " ".len();

        let total_task_name_length = self
            .task_names
            .iter()
            .map(|c| " ".len() + c.len() + " ".len())
            .sum::<usize>()
            + max(self.task_names.len() - 1, 0) * "|".len();
        let column_widths = self
            .class_names
            .iter()
            .map(|c| max(c.len() + 2, total_task_name_length))
            .collect::<Vec<usize>>();

        let total_length = first_column_width
            + column_widths.iter().sum::<usize>()
            + (column_widths.len() + 1) * "|".len();

        write!(f, "{}\n", "-".repeat(total_length))?;
        write!(f, "{}|", " ".repeat(first_column_width))?;

        for (index, class_name) in self.class_names.iter().enumerate() {
            write!(
                f,
                " {}{}|",
                class_name,
                " ".repeat(column_widths[index] - class_name.len() - " ".len()),
            )?;
        }
        write!(f, "\n")?;

        write!(
            f,
            "{}{}\n",
            " ".repeat(first_column_width),
            "-".repeat(total_length - (first_column_width))
        )?;
        write!(f, "{}|", " ".repeat(first_column_width))?;
        for column_index in 0..self.class_names.len() {
            let mut current_width = 0;
            for (index, task_name) in self.task_names.iter().enumerate() {
                current_width += format!(" {task_name} ").len();
                write!(
                    f,
                    " {}{}|",
                    task_name,
                    " ".repeat(if index < (self.task_names.len() - 1) {
                        1
                    } else {
                        (column_widths[column_index] - current_width) - 1
                    })
                )?;
            }
        }
        write!(f, "\n")?;

        for (week, classes) in self.overview.iter().enumerate() {
            write!(f, "{}\n", "-".repeat(total_length))?;
            write!(
                f,
                " week {}{week} |",
                "0".repeat(self.overview.len().to_string().len() - format!("{week}").len())
            )?;
            for (column_index, states) in classes.iter().enumerate() {
                let mut current_width = 0;
                for (index, task_name) in self.task_names.iter().enumerate() {
                    current_width += format!(" {task_name} ").len();
                    write!(
                        f,
                        " {}{}|",
                        {
                            if states[index] {
                                "█"
                            } else {
                                " "
                            }
                        }
                        .repeat(task_name.len()),
                        " ".repeat(if index < (self.task_names.len() - 1) {
                            1
                        } else {
                            (column_widths[column_index] - current_width) - 1
                        })
                    )?;
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "{}", "-".repeat(total_length))?;

        Ok(())
    }
}

fn main() {
    let configuration_file_path = home::home_dir()
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
                .arg(arg!([class_name] "The class name").value_parser(value_parser!(String))),
        )
        .subcommand(
            Command::new("addtask")
                .about("Adds a new task to the overview")
                .arg(arg!([task_name] "The task name").value_parser(value_parser!(String))),
        )
        .subcommand(
            Command::new("settime")
                .about("Sets the amount of weeks the overview is supposed to track")
                .arg(arg!([week_count] "The week count").value_parser(value_parser!(usize))),
        )
        .subcommand(Command::new("show").about("Shows the class overview"))
        .subcommand(
            Command::new("complete")
                .about("Completes the task for the given class")
                .arg(
                    arg!(
                        -c --class <CLASS> "The class to complete the task in"
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
                )
                .arg(
                    arg!(
                        -w --week <WEEK> "The week to complete the task for"
                    )
                    .required(true)
                    .value_parser(value_parser!(usize)),
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
        if !configuration_file_path.exists() {
            File::create(&configuration_file_path).expect(
                "Failed to create config file, does the directory exist or is it read-only?",
            );
        }
        read_to_string(&configuration_file_path)
            .expect("Failed to read config file, is the directory read-only?")
    };

    let mut class_overview = ClassOverview::new(&config);

    if let Some(matches) = matches.subcommand_matches("addclass") {
        if let Some(class_name) = matches.get_one::<String>("class_name") {
            println!("Added class {class_name}");
            class_overview.push_class(class_name);
        }
    }

    if let Some(matches) = matches.subcommand_matches("addtask") {
        if let Some(task_name) = matches.get_one::<String>("task_name") {
            println!("Added task {task_name}");
            class_overview.push_task(task_name);
        }
    }

    if let Some(matches) = matches.subcommand_matches("settime") {
        if let Some(week_count) = matches.get_one::<usize>("week_count") {
            println!("Set week count to {week_count}");
            class_overview.set_time_frame(week_count);
        }
    }

    if let Some(matches) = matches.subcommand_matches("complete") {
        if let Some(week) = matches.get_one::<usize>("week") {
            if let Some(class_name) = matches.get_one::<String>("class") {
                if let Some(task_name) = matches.get_one::<String>("task") {
                    if let Err(error) =
                        class_overview.complete_task_for_class(week, class_name, task_name)
                    {
                        println!("Failed to completed {task_name} for {class_name} in week {week}: {error}");
                    } else {
                        println!("Completed {task_name} for {class_name} in week {week}");
                    }
                } else {
                    println!("No task supplied, the task is necessary to flag its completion!");
                }
            } else {
                println!(
                    "No class supplied, the class is necessary to flag the task's completion!"
                );
            }
        } else {
            println!("No week supplied, the week is necessary to flag the task's completion!");
        }
    }

    if let Some(_) = matches.subcommand_matches("show") {
        println!("{class_overview}");
        return;
    }

    class_overview.save_configuration(&configuration_file_path);
}
