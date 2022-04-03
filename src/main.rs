use edik::unwrap_or_exit;

fn main() {
    let dir_path = "photos/";
    let files = std::fs::read_dir(dir_path).unwrap_or_else(|err| {
        println!("Couldn't find specified directory: {}", dir_path);
        std::process::exit(-1);
    });
    let re_naive_date = unwrap_or_exit!(regex::Regex::new(edik::RE_NAIVE_DATE));
    let re_correct_data_fmt = unwrap_or_exit!(regex::Regex::new(edik::RE_CORRECT_DATA_FMT));
    let mut count: u32 = 3u32;

    let mut file_dates: Vec<edik::FileDate> = Vec::new();

    // jpeg, mov, mp3, png
    files.for_each(|f_res| {
        let f = match f_res {
            Ok(dir_entry) => dir_entry,
            Err(err) => {
                println!("Something went wrong...");
                println!("{}", err);
                std::process::exit(0);
            }
        };
        let name = edik::get_file_name(&f);
        if re_correct_data_fmt.is_match(&name) {
            file_dates.push(match edik::FileDate::from_name(name) {
                Some(fd) => fd,
                None => {
                    println!("Couldn't parse correct format to date, please report this to maintainers of edik");
                    std::process::exit(1);
                }
            });
            return;
        }

        let date_created = unwrap_or_exit!(edik::get_file_creation_date(&f));
        let date_string: String = match re_naive_date.find(&date_created.to_string()) {
            Some(ma) => ma.as_str().to_owned(),
            None => {
                println!("debug: `{}`", &date_created.to_string());
                println!("Couldn't parse naive date, please report this to maintainers of edik.");
                std::process::exit(0);
            }
        };
        let mut new_file_path = String::new();
        new_file_path.push_str(dir_path);
        new_file_path.push_str(&date_string);
        new_file_path.push_str(" #");
        new_file_path.push_str(&count.to_string());
        new_file_path.push_str(".png");

        count += 1;
        // TODO: Check if file names match, and number them (temporarily) accordingly
        std::fs::rename(f.path(), &new_file_path);


        println!("{:?} -> {:?}", name, &new_file_path);
    });

    file_dates.into_iter().for_each(|fd| {
        println!("{:?}", fd);
    })

}