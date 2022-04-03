use std::error::Error;

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

pub fn btime_to_naive_date(duration: std::time::Duration) -> chrono::NaiveDateTime {
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

    let date_created = btime_to_naive_date(time_stamp);
    Ok(date_created)
}
