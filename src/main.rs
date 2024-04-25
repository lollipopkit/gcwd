use std::process::Stdio;

use chrono::{Local, NaiveDate, NaiveTime};
use clap::Parser;
use ctx::Ctx;

mod ctx;
mod update;

static CYAN: &str = "\x1b[36m";
static RESET: &str = "\x1b[0m";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = Ctx::parse();

    if !ctx.skip_update {
        update::update()?;
    }

    let full_time = parse_full_time(&ctx.time)?;
    println!("Committing at: {CYAN}{full_time}{RESET}");
    ctx.time = full_time;
    git_commit(&ctx)?;

    Ok(())
}

fn parse_full_time(full_time: &str) -> Result<String, String> {
    let parts: Vec<&str> = full_time.split(' ').collect();
    match parts.len() {
        1 => {
            let time = parse_time(full_time)?.format("%H:%M:%S");
            let date = Local::now().format("%Y-%m-%d");
            Ok(format!("{date} {time}"))
        }
        2 => {
            let date = parse_date(parts[0])?.format("%Y-%m-%d");
            let time = parse_time(parts[1])?.format("%H:%M:%S");
            Ok(format!("{date} {time}"))
        }
        _ => Err(format!("Invalid time format: {full_time}")),
    }
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

fn git_commit(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::new("git");
    let cmd = match ctx.message {
        Some(ref msg) => cmd
            .arg("commit")
            .arg("-m")
            .arg(msg)
            .env("GIT_AUTHOR_DATE", &ctx.time)
            .env("GIT_COMMITTER_DATE", &ctx.time),
        None => cmd
            .arg("commit")
            .env("GIT_AUTHOR_DATE", &ctx.time)
            .env("GIT_COMMITTER_DATE", &ctx.time),
    };
    let output = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    if output.success() {
        Ok(())
    } else {
        Err("Failed to commit".into())
    }
}
