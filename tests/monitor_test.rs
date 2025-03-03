mod common;
use common::{ExpectedFailure, ExpectedSuccess, Fixture};

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn using_regex_for_search_filter_works() {
    // GIVEN
    let fixture = Fixture::new();
    let mut cmd = fixture.command();
    cmd.args(["monitor", "profile", "-s", ".*-service", "--debug"]);

    // WHEN
    let output = cmd.output().expect("command should've run");

    // THEN
    output.print_stderr_if_failed(None);
    assert!(output.status.success());
}

#[test]
fn using_regex_for_key_filter_works() {
    // GIVEN
    let fixture = Fixture::new();
    let mut cmd = fixture.command();
    cmd.args(["monitor", "profile", "-k", "qa|staging", "--debug"]);

    // WHEN
    let output = cmd.output().expect("command should've run");

    // THEN
    output.print_stderr_if_failed(None);
    assert!(output.status.success());
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn using_invalid_regex_for_search_filter_fails() {
    // GIVEN
    let fixture = Fixture::new();
    let mut cmd = fixture.command();
    cmd.args(["monitor", "profile", "-s", "(a(bc", "--debug"]);

    // WHEN
    let output = cmd.output().expect("command should've run");

    // THEN
    output.print_stdout_if_succeeded(None);
    assert!(!output.status.success());
}

#[test]
fn using_invalid_regex_for_key_filter_fails() {
    // GIVEN
    let fixture = Fixture::new();
    let mut cmd = fixture.command();
    cmd.args(["monitor", "profile", "-k", "(a(bc", "--debug"]);

    // WHEN
    let output = cmd.output().expect("command should've run");

    // THEN
    output.print_stdout_if_succeeded(None);
    assert!(!output.status.success());
}
