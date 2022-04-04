use std::error::Error;
use std::ffi::OsStr;

pub const RE_NAIVE_DATE: &str = r#"^[0-9]{4}-[0-9]{2}-[0-9]{2}"#;
pub const RE_CORRECT_DATA_FMT: &str = r#"^[0-9]{4}-[0-9]{2}-[0-9]{2} #[0-9]+"#;

#[macro_export]
macro_rules! unwrap_or_exit {
    ( $a:expr ) => {
        match $a {
            Ok(val) => val,
            Err(err) => {
                println!("Error: Please report this to maintainers of edik\n{}", err);
                std::process::exit(1);
            }
        }
    };
}

#[derive(Debug)]
pub struct FileDate {
    pub path: String,
    pub duration: i64,
    pub new_path: String,
}

impl FileDate {
    pub fn from_name(path: String) -> Option<FileDate> {
        let mut y = String::new();
        let mut m = String::new();
        let mut d = String::new();
        let mut string_iter = path.chars();
        y.push(string_iter.next()?);
        y.push(string_iter.next()?);
        y.push(string_iter.next()?);
        y.push(string_iter.next()?);
        string_iter.next();
        m.push(string_iter.next()?);
        m.push(string_iter.next()?);
        string_iter.next();
        d.push(string_iter.next()?);
        d.push(string_iter.next()?);
        let date = chrono::NaiveDate::from_ymd(
            unwrap_or_exit!(y.parse::<i32>()),
            unwrap_or_exit!(m.parse::<u32>()),
            unwrap_or_exit!(d.parse::<u32>())
        ).and_hms(0, 0, 0);
        Some(FileDate {
            path,
            duration: date.timestamp(),
            new_path: String::new(),
        })
    }

    pub fn from_date(path: String, date: chrono::NaiveDateTime) -> FileDate {
        FileDate {
            path,
            duration: date.timestamp(),
            new_path: String::new(),
        }
    }
}

pub fn build_date_file_path(path: &str, duration: i64, index: u32) -> String {
    let date = btime_to_naive_date(duration).to_string();
    let extension = match get_file_extension(&path) {
        Some(p) => p,
        None => {
            println!("Couldn't get extension of file {}", path);
            std::process::exit(1);
        }
    };
    let mut new_path: String  = String::new();
    new_path.push_str(&date);
    new_path.push_str(" #");
    new_path.push_str(&index.to_string());
    new_path.push_str(extension);
    new_path
}

pub fn get_file_name(f: &std::fs::DirEntry) -> String {
    match f.file_name().to_str() {
        Some(s) => s.to_owned(),
        None => {
            println!("Error: Please report this to maintainers of edik");
            println!("Couldn't parse OsString when reading file name");
            std::process::exit(1);
        }
    }
}

pub fn get_file_extension(path: &str) -> Option<&str> {
    std::path::Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn btime_to_naive_date(duration: i64) -> chrono::NaiveDateTime {
    let unix_epoch_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    chrono::NaiveDateTime::from_timestamp(
        (unix_epoch_time - (duration as u64)) as i64,
        0u32
    )
}

pub fn duration_to_naive_date(duration: std::time::Duration) -> chrono::NaiveDateTime {
    let unix_epoch_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    chrono::NaiveDateTime::from_timestamp(
        (unix_epoch_time - duration.as_secs()) as i64,
        0u32
    )
}

pub fn get_file_creation_date(f: &std::fs::DirEntry) -> Result<chrono::NaiveDateTime, Box<dyn Error>>
{
    let time_stamp = std::fs::metadata(f.path())
        .unwrap()
        .created()
        .unwrap()
        .elapsed()
        .unwrap();

    let date_created = duration_to_naive_date(time_stamp);
    Ok(date_created)
}
