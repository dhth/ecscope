#[macro_use]
mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn adding_a_profile_works() {
    // GIVEN
    let fx = Fixture::new();

    // WHEN
    let mut add_cmd_one = fx.cmd(["profiles", "add", "prof1"]);
    let mut add_cmd_two = fx.cmd(["profiles", "add", "prof2"]);
    let mut show_cmd = fx.cmd(["profiles", "list"]);

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
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["profiles", "add", "split in three"]);

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
    let fx = Fixture::new();
    let mut cmd_one = fx.cmd(["profiles", "add", "prof1"]);

    apply_common_filters!();
    assert_cmd_snapshot!(cmd_one, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    Profile config file added at:
    [TEMP_FILE]

    You can edit the file in your text editor, and use it via "ecscope -p prof1"

    ----- stderr -----
    "#);

    // WHEN
    let mut cmd = fx.cmd(["profiles", "add", "prof1"]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: profile already exists
    ");
}
