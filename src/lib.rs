use std::error::Error;



pub fn get_file_creation_date(f: std::fs::DirEntry) -> Result<chrono::NaiveDateTime, Box<dyn Error>>
{
    let time_stamp = std::fs::metadata(f.path())
        .unwrap()
        .created()
        .unwrap()
        .elapsed()
        .unwrap();
    let unix_epoch_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let date_created = chrono::NaiveDateTime::from_timestamp(
        (unix_epoch_time - time_stamp.as_secs()) as i64,
        032
    );
    Ok(date_created)
}