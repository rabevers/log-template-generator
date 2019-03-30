use chrono::{NaiveDate, Weekday};
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{Write}; // , BufReader, BufRead
use std::error::Error;

fn main() {
    // let year = 2019;
    let week_number = 11;
    println!("Generating folder and files for week {} log", week_number);
    let path_string = &String::from(format!("./2019/{}", get_week_folder_name(week_number)));
    let path = Path::new(path_string);

    fs::create_dir_all(path).expect("Kon directory niet aanmaken");

    generate_log(path_string).expect("Kon log bestand niet schrijven");
    println!("Generating log");
    generate_uren(path_string, week_number).expect("Kon uren bestand niet schrijven");
    println!("Generating uren");
}

fn get_week_folder_name(week_number: u32) -> String {
    return format!("{}_-_{}_-_{}", week_number, get_week_start_date(week_number), get_week_end_date(week_number));
}

fn get_week_start_date(week_number : u32) -> String {
    let d = NaiveDate::from_isoywd(2019, week_number, Weekday::Mon);
    return d.format("%Y-%m-%d").to_string();
}

fn get_week_end_date(week_number: u32) -> String {
    let d = NaiveDate::from_isoywd(2019, week_number, Weekday::Sun);
    return d.format("%Y-%m-%d").to_string();
}

fn generate_log(path : &String) -> Result<(), Box<Error>> {
    let file_location = format!("{}/log.md", path);

    let mut log_file = File::create(file_location)?;
    write!(log_file, "# Maandag\n\n\n# Dinsdag\n\n\n# Woensdag\n\n\n# Donderdag\n\n\n# Vrijdag")?;

    Ok(())
}

fn generate_uren(path : &String, week_number: u32) -> Result<(), Box<Error>> {
    let file_location = format!("{}/uren.md", path);

    let monday = get_week_start_date(week_number);

    let d_tuesday = NaiveDate::from_isoywd(2019, week_number, Weekday::Tue);
    let tuesday = d_tuesday.format("%Y-%m-%d").to_string();

    let d_wednesday = NaiveDate::from_isoywd(2019, week_number, Weekday::Wed);
    let wednesday = d_wednesday.format("%Y-%m-%d").to_string();

    let d_thursday = NaiveDate::from_isoywd(2019, week_number, Weekday::Thu);
    let thursday = d_thursday.format("%Y-%m-%d").to_string();

    let d_friday = NaiveDate::from_isoywd(2019, week_number, Weekday::Fri);
    let friday = d_friday.format("%Y-%m-%d").to_string();

    let mut uren_file = File::create(file_location)?;
    let content = format!("# Uren\n\n[Log](./log.md)\n\n**Week 01**\n
|Datum      |Begin  | Eind	| Opmerkingen                        |
|---		    |---	  |---	  |---			                 |
| {} |  |  |          	         |
| {} |  |  |		                 |
| {} |  |  |	                     |
| {} |  |  |	                   |
| {} |  |  |         |", monday, tuesday, wednesday, thursday, friday);

    write!(uren_file, "{}", content)?;

    Ok(())
}