#[macro_use]
mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn using_regex_for_search_filter_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["monitor", "profile", "-s", ".*-service", "--debug"]);

    // WHEN
    // THEN
    apply_common_filters!();
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    [your arguments]
    command:                Monitor resources
    profile:                profile
    service name filter:    .*-service
    key filter:             <not provided>

    [computed config]
    config directory:    [TEMP_FILE]

    ----- stderr -----
    ");
}

#[test]
fn using_regex_for_key_filter_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["monitor", "profile", "-k", "qa|staging", "--debug"]);

    // WHEN
    // THEN
    apply_common_filters!();
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    [your arguments]
    command:                Monitor resources
    profile:                profile
    service name filter:    <not provided>
    key filter:             qa|staging

    [computed config]
    config directory:    [TEMP_FILE]

    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn using_invalid_regex_for_search_filter_fails() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["monitor", "profile", "-s", "(a(bc", "--debug"]);

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
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["monitor", "profile", "-k", "(a(bc", "--debug"]);

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
