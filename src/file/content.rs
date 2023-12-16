pub const LIB_RS_CONTENT: &[u8] = r#"use std::io::{Error, ErrorKind};

pub fn part_1(input: &str) -> Result<usize, Error> {
    Err(Error::new(ErrorKind::Other, "No Code"))
}

pub fn part_2(input: &str) -> Result<usize, Error> {
    Err(Error::new(ErrorKind::Other, "No Code"))
}
"#
.as_bytes();

const SESSION_COOKIE_HEADER: &str = include_str!("../../.session.cookie");

pub fn input_txt_content(chosen_year: &str, chosen_day: &str) -> String {
    let url = format!("https://adventofcode.com/{chosen_year}/day/{chosen_day}/input");

    match ureq::get(&url).set("Cookie", SESSION_COOKIE_HEADER).call() {
        Ok(res) => res.into_string().expect("to convert to string"),
        Err(err) => {
            panic!(
                "Request failed:\n{}",
                if err.kind() == ureq::ErrorKind::BadHeader {
                    "Make sure .session.cookie doesn't have a newline".to_string()
                } else {
                    err.to_string()
                }
            )
        }
    }
}

pub fn cargo_toml_content(chosen_day: u32) -> String {
    format!(
        r#"[package]
name = "day-{chosen_day:02}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
"#
    )
}

pub fn main_rs_content(chosen_day: u32) -> String {
    format!(
        r#"const INPUT: &str = include_str!("../input.txt");

fn main() {{
    let answer_1 = day_{chosen_day:02}::part_1(INPUT);
    let answer_2 = day_{chosen_day:02}::part_2(INPUT);

    match answer_1 {{
        Ok(answer) => println!("1: {{}}", answer),
        Err(err) => eprintln!("1: {{}}", err),
    }};

    match answer_2 {{
        Ok(answer) => println!("2: {{}}", answer),
        Err(err) => eprintln!("2: {{}}", err),
    }};
}}
"#
    )
}
