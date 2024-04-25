use self_update::cargo_crate_version;

pub fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    self_update::backends::github::Update::configure()
        .repo_owner("lollipopkit")
        .repo_name("gcwd")
        .bin_name("gcwd")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .show_output(false)
        .build()?
        .update()?;
    Ok(())
}
