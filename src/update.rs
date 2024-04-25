use crate::res;
use std::io::Write;

use self_update::{cargo_crate_version, self_replace::self_replace, version};


const GITHUB_OWNER: &str = "lollipopkit";
const BIN_NAME: &str = "gcwd";
const GITHUB_REPO: &str = "gcwd";

pub fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    let target = get_target().unwrap_or("unknown");
    if target == "unknown" {
        eprintln!("Unsupported platform");
        return Ok(());
    }

    let release = self_update::backends::github::Update::configure()
        .repo_owner(GITHUB_OWNER)
        .repo_name(GITHUB_REPO)
        .bin_name(BIN_NAME)
        .current_version(cargo_crate_version!())
        .build()?
        .get_latest_release();

    if release.is_err() {
        eprintln!("No releases found");
        return Ok(());
    }
    let release = release.unwrap();

    let has_newer = version::bump_is_greater(cargo_crate_version!(), &release.version);
    match has_newer {
        Ok(true) => println!(
            "New version available: {}{}{}",
            res::TERM_YELLOW, release.version, res::TERM_RESET
        ),
        Ok(false) => {
            println!("Already up to date");
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error checking for updates: {}", e);
            return Ok(());
        }
    }

    let asset = release.asset_for(target, None);
    if asset.is_none() {
        eprintln!("No asset found for target: {}", target);
        return Ok(());
    }
    let asset = asset.unwrap();

    print!("Do you want to update? [Y/n]: ");
    std::io::stdout().flush()?;
    let mut ask_resume = String::new();
    std::io::stdin().read_line(&mut ask_resume)?;
    if !ask_resume.trim().is_empty() && ask_resume.trim().to_lowercase() != "y" {
        return Ok(());
    }

    let tmp_dir = tempfile::Builder::new().tempdir_in(::std::env::current_dir()?)?;
    let tmp_tarball_path = tmp_dir.path().join(&asset.name);
    let tmp_tarball = ::std::fs::File::create(&tmp_tarball_path)?;

    self_update::Download::from_url(&asset.download_url)
        .show_progress(true)
        .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        .download_to(&tmp_tarball)?;

    let cmd_out = std::process::Command::new("tar")
        .arg("xzf")
        .arg(&tmp_tarball_path)
        .arg("-C")
        .arg(tmp_dir.path())
        .output()?;
    if !cmd_out.status.success() {
        eprintln!("Failed to extract tarball: {:?}", cmd_out);
        return Ok(());
    }

    let new_exe = tmp_dir.path().join(BIN_NAME);
    self_replace(new_exe)?;

    std::process::Command::new(BIN_NAME)
        .arg("--version")
        .status()?;

    println!("Update successfully");
    Ok(())
}

// eg:
// x86_64-unknown-linux-gnu -> linux_x64
// aaarch64-apple-darwin -> macos_arm64
fn get_target() -> Option<&'static str> {
    match self_update::get_target() {
        "x86_64-unknown-linux-gnu" => Some("linux_x64"),
        "aarch64-apple-darwin" => Some("macos_arm64"),
        _ => None,
    }
}
