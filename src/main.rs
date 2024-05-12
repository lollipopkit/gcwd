use std::process::Stdio;

use chrono::{Local, NaiveDate, NaiveTime};
use clap::Parser;
use ctx::Ctx;

mod ctx;
mod res;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = Ctx::parse();

    if let Some(ref time) = ctx.time {
        let full_time = parse_full_time(time)?;
        println!(
            "Committing at: {}{full_time}{}",
            res::TERM_YELLOW,
            res::TERM_RESET
        );
        ctx.time = Some(full_time);
        git_commit(&ctx)?;
    } else {
        eprintln!("No time provided");
    }

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
    let time = ctx.time.as_ref().unwrap();
    let mut cmd = std::process::Command::new("git");
    cmd.arg("commit");
    if ctx.sign {
        cmd.arg("-S");
    }
    if let Some(ref msg) = ctx.message {
        cmd.arg("-m").arg(msg);
    }
    cmd.env("GIT_AUTHOR_DATE", time);
    cmd.env("GIT_COMMITTER_DATE", time);
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
