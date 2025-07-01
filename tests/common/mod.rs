use insta_cmd::get_cargo_bin;
use std::process::Command;
use tempfile::TempDir;

#[cfg(test)]
pub fn base_command() -> Command {
    Command::new(get_cargo_bin("ecscope"))
}

#[cfg(test)]
pub struct TestFixture {
    _temp_dir: TempDir,
    config_dir_path: String,
}

#[cfg(test)]
#[allow(unused)]
impl TestFixture {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("couldn't create temporary directory");
        let data_file_path = temp_dir
            .path()
            .to_str()
            .expect("temporary directory path is not valid utf-8")
            .to_string();
        Self {
            _temp_dir: temp_dir,
            config_dir_path: data_file_path,
        }
    }

    pub fn config_dir(&self) -> &str {
        &self.config_dir_path
    }
}

#[cfg(test)]
#[allow(unused)]
macro_rules! apply_common_filters {
    {} => {
        let mut settings = insta::Settings::clone_current();
        // Macos Temp Folder
        settings.add_filter(r"/var/folders/\S+?/T/\S+", "[TEMP_FILE]");
        // Linux Temp Folder
        settings.add_filter(r"/tmp/\.tmp\S+", "[TEMP_FILE]");
        // Windows Temp folder
        settings.add_filter(r"\b[A-Z]:\\.*\\Local\\Temp\\\S+", "[TEMP_FILE]");
        // Convert windows paths to Unix Paths.
        settings.add_filter(r"\\\\?([\w\d.])", "/$1");
        let _bound = settings.bind_to_scope();
    }
}
