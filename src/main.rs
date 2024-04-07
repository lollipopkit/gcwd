use std::process::Stdio;

use chrono::{Local, NaiveDate, NaiveTime};

static CYAN: &str = "\x1b[36m";
static RESET: &str = "\x1b[0m";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = std::env::args().nth(1).expect("No time provided");
    let full_time = parse_full_time(&time)?;
    println!("Committing at: {CYAN}{full_time}{RESET}");
    git_commit(full_time)
}

fn parse_full_time(full_time: &str) -> Result<String, String> {
    let parts: Vec<&str> = full_time.split(' ').collect();
    if parts.len() != 2 {
        let time = parse_time(full_time)?.format("%H:%M:%S");
        let date = Local::now().format("%Y-%m-%d");
        return Ok(format!("{date} {time}"));
    }
    let date = parse_date(parts[0])?.format("%Y-%m-%d");
    let time = parse_time(parts[1])?.format("%H:%M:%S");
    Ok(format!("{date} {time}"))
}

fn parse_time(time: &str) -> Result<NaiveTime, String> {
    let fmts = vec!["%H:%M", "%H-%M", "%H:%M:%S", "%H-%M-%S"];
    for fmt in fmts {
        match NaiveTime::parse_from_str(time, fmt) {
            Ok(t) => return Ok(t),
            Err(_) => continue,
        }
    }
    Err(format!("No matching format found: {time}"))
}

fn parse_date(date: &str) -> Result<NaiveDate, String> {
    let fmts = vec!["%Y-%m-%d", "%Y-%m-%d", "%Y/%m/%d", "%Y/%m/%d"];
    for fmt in fmts {
        match NaiveDate::parse_from_str(date, fmt) {
            Ok(t) => return Ok(t),
            Err(_) => continue,
        }
    }
    Err(format!("No matching format found: {date}"))
}

fn git_commit(time: String) -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new("git")
        .arg("commit")
        .env("GIT_AUTHOR_DATE", &time)
        .env("GIT_COMMITTER_DATE", &time)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?.wait()?;
    if output.success() {
        Ok(())
    } else {
        Err("Failed to commit".into())
    }
}
