use insta_cmd::get_cargo_bin;
use std::{ffi::OsStr, path::PathBuf, process::Command};
use tempfile::{TempDir, tempdir};

pub struct Fixture {
    _bin_path: PathBuf,
    _temp_dir: TempDir,
    config_dir_path: String,
}

#[cfg(test)]
#[allow(unused)]
impl Fixture {
    pub fn new() -> Self {
        let bin_path = get_cargo_bin("ecscope");
        let temp_dir = tempdir().expect("temporary directory should've been created");
        let config_dir_path = temp_dir
            .path()
            .to_str()
            .expect("temporary directory path is not valid utf-8")
            .to_string();

        Self {
            _bin_path: bin_path,
            _temp_dir: temp_dir,
            config_dir_path,
        }
    }

    pub fn base_cmd(&self) -> Command {
        let mut cmd = Command::new(&self._bin_path);
        cmd.args(["--config-dir", &self.config_dir_path]);
        cmd
    }

    pub fn cmd<I, S>(&self, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new(&self._bin_path);
        command.args(args);
        command.args(["--config-dir", &self.config_dir_path]);
        command
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
