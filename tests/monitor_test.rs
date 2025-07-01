#[macro_use]
mod common;

use common::{TestFixture, base_command};
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn using_regex_for_search_filter_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let config_dir = fixture.config_dir();
    let mut cmd = base_command();
    let mut cmd = cmd.args([
        "monitor",
        "profile",
        "-s",
        ".*-service",
        "--config-dir",
        config_dir,
        "--debug",
    ]);

    // WHEN
    // THEN
    apply_common_filters!();
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    <your arguments>
    command             : Monitor resources
    profile             : profile
    service name filter : .*-service
    key filter          : <not provided>

    <computed config>
    config directory: [TEMP_FILE]

    ----- stderr -----
    ");
}

#[test]
fn using_regex_for_key_filter_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let config_dir = fixture.config_dir();
    let mut cmd = base_command();
    let mut cmd = cmd.args([
        "monitor",
        "profile",
        "-k",
        "qa|staging",
        "--config-dir",
        config_dir,
        "--debug",
    ]);

    // WHEN
    // THEN
    apply_common_filters!();
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    <your arguments>
    command             : Monitor resources
    profile             : profile
    service name filter : <not provided>
    key filter          : qa|staging

    <computed config>
    config directory: [TEMP_FILE]

    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn using_invalid_regex_for_search_filter_fails() {
    // GIVEN
    let mut cmd = base_command();
    let mut cmd = cmd.args(["monitor", "profile", "-s", "(a(bc", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value '(a(bc' for '--service-filter <REGEX>': query "(a(bc" is not valid regex: regex parse error:
        (a(bc
          ^
    error: unclosed group

    For more information, try '--help'.
    "#);
}

#[test]
fn using_invalid_regex_for_key_filter_fails() {
    // GIVEN
    let mut cmd = base_command();
    let mut cmd = cmd.args(["monitor", "profile", "-k", "(a(bc", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value '(a(bc' for '--key-filter <REGEX>': query "(a(bc" is not valid regex: regex parse error:
        (a(bc
          ^
    error: unclosed group

    For more information, try '--help'.
    "#);
}
