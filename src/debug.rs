use crate::args::Args;
use std::path::Path;

pub fn display_debug_info(args: &Args, config_dir: &Path) {
    println!(
        r#"DEBUG INFO:

<your arguments>{}
<computed config>
config directory: {}"#,
        args,
        config_dir.to_string_lossy()
    )
}
