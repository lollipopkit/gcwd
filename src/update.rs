use self_update::cargo_crate_version;

pub fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    self_update::backends::github::Update::configure()
        .repo_owner("lollipopkit")
        .repo_name("gcwd")
        .bin_name("gcwd")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .target(&get_target())
        .show_output(false)
        .build()?
        .update()?;
    Ok(())
}

// eg: 
// x86_64-unknown-linux-gnu -> linux_x64
// aaarch64-apple-darwin -> macos_arm64
fn get_target() -> &'static str {
    match self_update::get_target() {
        "x86_64-unknown-linux-gnu" => "linux_x64",
        "aarch64-apple-darwin" => "macos_arm64",
        _ => panic!("Unsupported target"),
    }
}
