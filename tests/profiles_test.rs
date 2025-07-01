#[macro_use]
mod common;

use common::{TestFixture, base_command};
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn adding_a_profile_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let config_dir = fixture.config_dir();

    // WHEN
    let mut add_cmd_one = base_command();
    let mut add_cmd_one =
        add_cmd_one.args(["profiles", "add", "prof1", "--config-dir", config_dir]);

    let mut add_cmd_two = base_command();
    let mut add_cmd_two =
        add_cmd_two.args(["profiles", "add", "prof2", "--config-dir", config_dir]);

    let mut show_cmd = base_command();
    let mut show_cmd = show_cmd.args(["profiles", "list", "--config-dir", config_dir]);

    // THEN
    apply_common_filters!();
    assert_cmd_snapshot!(add_cmd_one, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    Profile config file added at:
    [TEMP_FILE]

    You can edit the file in your text editor, and use it via "ecscope -p prof1"

    ----- stderr -----
    "#);
    assert_cmd_snapshot!(add_cmd_two, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    Profile config file added at:
    [TEMP_FILE]

    You can edit the file in your text editor, and use it via "ecscope -p prof2"

    ----- stderr -----
    "#);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    prof1	(located at [TEMP_FILE]
    prof2	(located at [TEMP_FILE]

    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn using_incorrect_profile_name_fails() {
    // GIVEN
    let mut cmd = base_command();
    let mut cmd = cmd.args(["profiles", "add", "split in three"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: profile name is invalid; valid regex: ^[a-z0-9_-]{1,20}$
    ");
}

#[test]
fn adding_duplicate_profile_fails() {
    // GIVEN
    let mut cmd_one = base_command();
    let mut add_cmd_one = cmd_one.args(["profiles", "add", "prof1"]);

    assert_cmd_snapshot!(add_cmd_one, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: profile already exists
    ");

    // WHEN
    let mut cmd = base_command();
    let mut cmd = cmd.args(["profiles", "add", "prof1"]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: profile already exists
    ");
}
