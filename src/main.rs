use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;

fn main() {
    let dir_path = "photos/";

    let files = std::fs::read_dir(dir_path).unwrap();

    // jpeg, mov, mp4, png
    files.for_each(|f_res| {
        let f = match f_res {
            Ok(dir_entry) => dir_entry,
            Err(err) => {
                println!("Something went wrong...");
                println!("{}", err);
                std::process::exit(1);
            }
        };
        let name = f.file_name();
        let date_created = match edik::get_file_creation_date(f){
            Ok(date) => date,
            Err(err) => {
                println!("Error: Please report this to maintainers of edik, {}", err);
                std::process::exit(1);
            }
        };
        println!("{:?}: {:?}", name, date_created);
    });
}
