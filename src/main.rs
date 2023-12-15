use std::io::Error;

use chrono::{DateTime, Datelike, Local, NaiveDate};
use inquire::{DateSelect, Select};

const DECEMBER: u32 = 12;
const FIRST_ADVENT_OF_CODE: i32 = 2015;
const LAST_ADVENT_DAY: u32 = 25;

mod file;

fn main() -> Result<(), Error> {
    let current_time = Local::now();
    let (latest_advent_year, latest_advent_day) = get_latest_advent(&current_time);

    let user_chosen_year = select_year(FIRST_ADVENT_OF_CODE, latest_advent_year);

    let latest_advent_day = if user_chosen_year == latest_advent_year {
        latest_advent_day
    } else {
        LAST_ADVENT_DAY
    };

    let chosen_advent = select_advent_day(user_chosen_year, latest_advent_day);

    let code_folder =
        file::operations::create_directories(chosen_advent.year(), chosen_advent.day())?;
    file::operations::create_files(chosen_advent.year(), chosen_advent.day())?;

    println!(
        "Puzzle is at https://adventofcode.com/{}/day/{}\nCode is at {}",
        chosen_advent.year(),
        chosen_advent.day(),
        code_folder.display()
    );

    Ok(())
}

fn get_latest_advent(current_time: &DateTime<Local>) -> (i32, u32) {
    if current_time.month() == DECEMBER {
        (
            current_time.year(),
            current_time.day().clamp(0, LAST_ADVENT_DAY),
        )
    } else {
        (current_time.year() - 1, LAST_ADVENT_DAY)
    }
}

fn select_year(start_year: i32, latest_year: i32) -> i32 {
    let years = (start_year..=latest_year)
        .map(|x| x.to_string())
        .rev()
        .collect::<Vec<_>>();

    Select::new("Which Year are you doing?", years)
        .prompt()
        .expect("year to be chosen")
        .parse::<i32>()
        .expect("valid number")
}

fn select_advent_day(user_chosen_year: i32, latest_advent_day: u32) -> NaiveDate {
    let dec_first = NaiveDate::from_ymd_opt(user_chosen_year, 12, 1).expect("Valid Date");
    let max_date =
        NaiveDate::from_ymd_opt(user_chosen_year, 12, latest_advent_day).expect("Valid Date");

    DateSelect::new("Which Challenge are you doing?")
        .with_min_date(dec_first)
        .with_starting_date(max_date)
        .with_max_date(max_date)
        .prompt()
        .expect("challenge to be chosen")
}
